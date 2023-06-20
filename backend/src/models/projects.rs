use super::ids::{ProjectId, TaskGroupId, TaskId, UserId, ProjectMemberId};
use super::tasks::{TaskGroup, Task};
use actix_web::HttpRequest;
use futures::TryStreamExt;
use sqlx::SqlitePool;
use actix_web::HttpMessage;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub owner: UserId,
    pub icon_url: String,
    pub public: bool
}

#[derive(Serialize, Deserialize, Validate)]
pub struct ProjectBuilder {
    #[validate(length(min = 3, max = 30))]
    name: String,
    icon_url: String,
    public: bool
}

impl ProjectBuilder {
    pub async fn create(
        &self,
        creator: UserId,
        conn: &SqlitePool
    ) -> Result<ProjectId, super::DatabaseError> {
        let id = ProjectId::generate(conn).await?;

        let project = Project {
            id,
            name: self.name.clone(),
            owner: creator.clone(),
            icon_url: self.icon_url.clone(),
            public: self.public
        };

        project.insert(conn).await?;

        let id = ProjectMemberId::generate(conn).await?;

        let project_member = ProjectMember {
            id,
            project_id: project.id.clone(),
            user_id: creator,
            permissions: Permissions::all(),
            accepted: true
        };

        project_member.insert(conn).await?;

        Ok(project.id)
    }
}

impl Project {
    async fn insert(
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
            self.id,
            self.name,
            self.owner,
            self.icon_url
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn get(
        id: ProjectId,
        conn: &SqlitePool
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let query = sqlx::query!(
            "
            SELECT id, name, owner,
                   icon_url, public
            FROM projects
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(conn)
        .await?;

        if let Some(row) = query {
            Ok(Some(Self {
               id: ProjectId(row.id),
               name: row.name,
               owner: UserId(row.owner),
               icon_url: row.icon_url,
               public: row.public
            }))
        } else  {
            Ok(None)
        }
    }

    pub async fn from_user(
        id: UserId,
        conn: &SqlitePool
    ) -> Result<Vec<Self>, sqlx::error::Error> {
        let memberships = ProjectMember::from_user(id, conn).await?;

        let mut projects: Vec<Self> = vec![];

        for member in memberships {
            if let Some(project) = Self::get(member.project_id, conn).await? {
                projects.push(project)
            }
        }

        Ok(projects)
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
            self.id
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            DELETE FROM projects
            WHERE id = $1
            ",
            self.id
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
            self.id
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
            self.id
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
                 name, information, creator, due, 
                 primary_colour, accent_colour
            FROM tasks
            WHERE project_id = $1
            ",
            self.id
        )
        .fetch_many(conn)
        .try_filter_map(|result| async {
            Ok(result.right().map(|row| Task {
                id: TaskId(row.id),
                project_id: ProjectId(row.project_id),
                task_group_id: TaskGroupId(row.task_group_id),
                name: row.name,
                information: row.information,
                creator: UserId(row.creator),
                due: row.due,
                primary_colour: row.primary_colour,
                accent_colour: row.accent_colour,
            }))
        })
        .try_collect::<Vec<Task>>()
        .await?;

        Ok(tasks)
    }
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

#[derive(Serialize, Deserialize)]
pub struct ProjectMember {
    pub id: ProjectMemberId,
    pub project_id: ProjectId,
    pub user_id: UserId,
    pub permissions: Permissions,
    pub accepted: bool,
}


#[derive(Serialize, Deserialize)]
pub struct ProjectInvitation {
    pub user: UserId,
    pub project: ProjectId,
}

impl ProjectMember {
    pub async fn create(
        user: UserId,
        project: &Project,
        conn: &SqlitePool
    ) -> Result<ProjectMember, super::DatabaseError> {
        if project.owner.0 != user.0 {
            return Err(super::DatabaseError::AlreadyExists);
        }

        let member_id = ProjectMemberId::generate(conn).await?;

        let member = ProjectMember {
            id: member_id,
            project_id: project.id.clone(),
            user_id: user,
            permissions: Permissions::all(),
            accepted: true,
        };

        member.insert(conn).await?;

        Ok(member)
    }

    pub async fn invite(
        invitation: ProjectInvitation,
        conn: &SqlitePool
    ) -> Result<ProjectMember, super::DatabaseError> {
        let query = sqlx::query!(
            "
            SELECT EXISTS (
                SELECT 1
                FROM project_members
                WHERE user_id = $1 
                AND project_id = $2
            ) 
            AS member_exists
            ",
            invitation.user.0,
            invitation.project.0
        )
        .fetch_one(conn)
        .await?;

        if query.member_exists.is_positive() {
            return Err(super::DatabaseError::AlreadyExists);
        }

        let member_id = ProjectMemberId::generate(conn).await?;

        let member = ProjectMember {
            id: member_id,
            project_id: invitation.project,
            user_id: invitation.user,
            permissions: Permissions::default(),
            accepted: false,
        };

        member.insert(conn).await?;

        Ok(member)
    }

    pub async fn from_request(
        req: HttpRequest,
        conn: &SqlitePool
    ) -> Result<Option<Self>, super::DatabaseError> {
        if let Some(id) = req.extensions().get::<ProjectMemberId>() {
            let query = sqlx::query!(
                "
                SELECT id, project_id,
                user_id, permissions, 
                accepted
                FROM project_members
                WHERE id = $1
                ",
                id
            )
            .fetch_optional(conn)
            .await?;

            if let Some(row) = query {
                return Ok(Some(Self {
                    id: ProjectMemberId(row.id),
                    project_id: ProjectId(row.project_id),
                    user_id: UserId(row.user_id),
                    permissions: Permissions::from_bits(row.permissions as u64).unwrap_or_default(),
                    accepted: row.accepted,
                }))
            }
        }
        Ok(None)
    }

    pub async fn from_user_for_project(
        user: UserId,
        project: ProjectId,
        conn: &SqlitePool
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let query = sqlx::query!(
            "
            SELECT id, project_id,
                user_id, permissions, 
                accepted
            FROM project_members
            WHERE user_id = $1
            AND project_id = $2
            ",
            user.0,
            project.0
        )
        .fetch_optional(conn)
        .await?;

        if let Some(row) = query {
            Ok(Some(Self {
                id: ProjectMemberId(row.id),
                project_id: ProjectId(row.project_id),
                user_id: UserId(row.user_id),
                permissions: Permissions::from_bits(row.permissions as u64).unwrap_or_default(),
                accepted: row.accepted,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn from_user(
        user: UserId,
        conn: &SqlitePool
    ) -> Result<Vec<Self>, sqlx::error::Error> {
        let query = sqlx::query!(
            "
            SELECT id, project_id,
                user_id, permissions, 
                accepted
            FROM project_members
            WHERE user_id = $1
            ",
            user.0
        )
        .fetch_many(conn)
        .try_filter_map(|e| async {
            Ok(e.right().map(|m| Self {
                id: ProjectMemberId(m.id),
                project_id: ProjectId(m.project_id),
                user_id: UserId(m.user_id),
                permissions: Permissions::from_bits(m.permissions as u64).unwrap_or_default(),
                accepted: m.accepted,
            }))
        })
        .try_collect::<Vec<Self>>()
        .await?;

        Ok(query)
    }

    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        let permssions = self.permissions.bits() as i64;

        sqlx::query!(
            "
            INSERT INTO project_members (
                id, project_id, user_id,
                permissions, accepted
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.project_id,
            self.user_id,
            permssions,
            self.accepted
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn accept_invitation(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE project_members
            SET accepted = true
            WHERE id = $1
            ",
            self.id
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn deny_invitation(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM project_members
            WHERE id = $1
            ",
            self.id
        )
        .execute(conn)
        .await?;

        Ok(())
    }
}