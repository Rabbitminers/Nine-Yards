use super::{
    ids::{
        ProjectId, 
        TeamId, 
        TaskGroupId, 
        TaskId, 
        UserId
    }, 
    tasks::{
        TaskGroup, 
        Task
    }, 
    teams::Team
};


use futures::TryStreamExt;
use sqlx::SqlitePool;

pub struct Project {
    id: ProjectId,
    team: TeamId,
    name: String,
    icon_url: String
}

pub struct ProjectBuilder {
    name: String,
    icon_url: String
}

impl Project {
    pub async fn create(
        data: ProjectBuilder,
        conn: &SqlitePool
    ) -> Result<String, String> {
        unimplemented!()
    }

    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {   
        sqlx::query!(
            "
            INSERT INTO projects (
                id, team_id, name, icon_url
            )
            VALUES (
                $1, $2, $3, $4
            )
            ",
            self.id.0,
            self.team.0,
            self.name,
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
            group.remove(conn);
        }

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

    pub async fn get_team(
        &self,
        conn: &SqlitePool
    ) -> Result<Option<Team>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT id
            FROM teams
            WHERE id = $1
            ",
            self.team.0
        )
        .fetch_optional(conn)
        .await?;

        if let Some(row) = result {
            Ok(Some(Team {
                id: TeamId(row.id)
            }))
        } else {
            Ok(None)
        }
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