use sqlx::SqlitePool;

use crate::constants;

use super::{ids::{
    TaskId, 
    ProjectId, 
    TaskGroupId, 
    UserId, generate_task_group_id
}, users::User};

pub struct TaskGroup {
    pub id: TaskGroupId,
    pub project_id: ProjectId,
    pub name: String,
    pub position: i64
}

pub struct TaskGroupBuilder {
    pub creator: User,
    pub project_id: ProjectId,
    pub name: String,
    pub position: i64 // Got to have room for your 9223372036854775807 task groups!
}

impl TaskGroup {
    pub async fn create(
        data: TaskGroupBuilder,
        conn: &SqlitePool
    ) -> Result<String, String> {
        if let Ok(Some(_)) = Self::find_by_name(&data.name, conn).await {
            return Err(format!("A task group with name '{}' already exists", &data.name));
        }

        let group_id = match generate_task_group_id(conn).await {
            Ok(generated_id) => generated_id,
            Err(_) => return Err(constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()),
        };

        TaskGroup {
            id: group_id,
            project_id: data.project_id,
            name: data.name,
            position: data.position
        }.insert(conn);

        Ok(constants::MESSAGE_CREATE_TASK_GROUP_SUCCESS.to_string())
    }

    pub async fn find_by_name(
        name: &String,
        conn: &SqlitePool
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT id, project_id, name, position
            FROM task_groups
            WHERE name = $1
            ",
            name
        )
        .fetch_optional(conn)
        .await?;
        
        if let Some(row) = result {
            Ok(Some(Self {
                id: TaskGroupId(row.id),
                project_id: ProjectId(row.project_id),
                name: row.name,
                position: row.position
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO task_groups (
                id, project_id, name, position
            )
            VALUES (
                $1, $2, $3, $4
            )
            ",
            self.id.0,
            self.project_id.0,
            self.name,
            self.position
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn remove(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE task_group_id = $1
            ",
            self.id.0
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            DELETE FROM task_groups
            WHERE id = $1
            ",
            self.id.0
        )
        .execute(conn)
        .await?;

        Ok(())
    }
}

pub struct Task {
    pub id: TaskId,
    pub project_id: ProjectId,
    pub task_group_id: TaskGroupId,
    pub name: String,
    pub information: String,
    pub creator: UserId,
    pub assignee: UserId
}

pub struct TaskBuilder {
    pub task_group_id: TaskGroupId,
    pub name: String,
    pub information: String,
    pub creator: UserId
}
