use super::{
    ids::{
        ProjectId, 
        TaskGroupId, 
        TaskId, 
        UserId, generate_project_id, ProjectMemberId
    }, 
    tasks::{
        TaskGroup, 
        Task
    }, 
    error::ServiceError
};

use futures::{TryStreamExt};
use sqlx::SqlitePool;

pub struct Project {
    id: ProjectId,
    name: String,
    owner: UserId,
    icon_url: String
}

pub struct ProjectBuilder {
    name: String,
    creator: UserId, // Used to create a team
    icon_url: String
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Permissions: u64 {
        const MANAGE_TASKS = 1 << 0;
        const MANAGE_PROJECT = 1 << 1;
        const MANAGE_TEAM = 1 << 2;
        const ALL = Self::MANAGE_TASKS.bits 
            | Self::MANAGE_PROJECT.bits
            | Self::MANAGE_TEAM.bits;
    }
}

impl Default for Permissions {
    fn default() -> Permissions {
        Permissions::MANAGE_TASKS
    }
}

pub struct ProjectMember {
    pub id: ProjectMemberId,
    pub project_id: ProjectId,
    pub user_id: UserId,
    pub permissions: Permissions,
    pub accepted: bool,
}

impl Project {
    pub async fn create(
        data: ProjectBuilder,
        conn: &SqlitePool
    ) -> Result<Project, ServiceError> {
        let project_id = generate_project_id(conn).await?;

        let project: Project = Project{
            id: project_id,
            name: data.name,
            owner: data.creator,
            icon_url: data.icon_url,
        };

        project.insert(conn).await;
        Ok(project)
    }

    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {   
        sqlx::query!(
            "
            INSERT INTO projects (
                id, name, owner, icon_url
            )
            VALUES (
                $1, $2, $3, $4
            )
            ",
            self.id.0,
            self.name,
            self.owner.0,
            self.icon_url
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn remove(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        let task_groups = self.get_task_groups(conn).await?;
        
        for group in task_groups {
            group.remove(conn).await?;
        }

        sqlx::query!(
            "
            DELETE FROM project_members
            WHERE project_id = $1
            ",
            self.id.0
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            DELETE FROM projects
            WHERE id = $1
            ",
            self.id.0
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn get_members(
        &self, 
        conn: &SqlitePool
    ) -> Result<Vec<ProjectMember>, sqlx::Error> {
        let users =sqlx::query!(
            "
            SELECT id, project_id, user_id, 
                permissions, accepted 
            FROM project_members 
            WHERE project_id = ?
            ",
            self.id.0
        )
        .fetch_many(conn)
        .try_filter_map(|e| async {
            Ok(e.right().map(|m| ProjectMember {
                id: ProjectMemberId(m.id),
                project_id: ProjectId(m.project_id),
                user_id: UserId(m.user_id),
                permissions: Permissions::from_bits(m.permissions as u64).unwrap_or_default(),
                accepted: m.accepted,
            }))
        })
        .try_collect::<Vec<ProjectMember>>()
        .await?;

        Ok(users)
    }

    pub async fn get_task_groups(
        &self,
        conn: &SqlitePool
    ) -> Result<Vec<TaskGroup>, sqlx::error::Error> {
        let task_groups = sqlx::query!(
            "
            SELECT id, project_id, name, position
            FROM task_groups
            WHERE project_id = $1
            ",
            self.id.0
        )
        .fetch_many(conn)
        .try_filter_map(|e| async {
            Ok(e.right().map(|m| TaskGroup {
                id: TaskGroupId(m.id),
                project_id: ProjectId(m.project_id),
                name: m.name,
                position: m.position
            }))
        })
        .try_collect::<Vec<TaskGroup>>()
        .await?;

        Ok(task_groups)
    }   

    pub async fn get_tasks(
        &self,
        conn: &SqlitePool
    ) -> Result<Vec<Task>, sqlx::error::Error> {
        let tasks = sqlx::query!(
            "
            SELECT id, project_id, task_group_id,
            name, information, creator, assignee
            FROM tasks
            WHERE project_id = $1
            ",
            self.id.0
        )
        .fetch_many(conn)
        .try_filter_map(|e| async {
            Ok(e.right().map(|m| Task {
                id: TaskId(m.id),
                project_id: ProjectId(m.project_id),
                task_group_id: TaskGroupId(m.task_group_id),
                name: m.name,
                information: m.information,
                creator: UserId(m.creator),
                assignee: UserId(m.assignee)
            }))
        })
        .try_collect::<Vec<Task>>()
        .await?;

        Ok(tasks)
    }   
}