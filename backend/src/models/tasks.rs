use actix_web::http::StatusCode;
use sqlx::SqlitePool;

use super::{
    ids::{
        TaskId, 
        ProjectId, 
        TaskGroupId, 
        UserId, 
        generate_task_group_id
    }, 
    users::User, error::ServiceError
};

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

impl TaskGroupBuilder {
    pub async fn create(
        &self,
        conn: &SqlitePool
    ) -> Result<TaskGroup, super::error::ServiceError> {
        if let Ok(Some(_)) = TaskGroup::find_by_name(&self.name, conn).await {
            return Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                format!("A task group with name '{}' already exists", &self.name)
            ));
        }

        let id = generate_task_group_id(&conn).await?;

        let group = TaskGroup {
            id,
            project_id: self.project_id.clone(),
            name: self.name.clone(),
            position: self.position
        };

        group.insert(conn).await?;
        
        Ok(group)
    }
}

impl TaskGroup {
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
    pub project_id: ProjectId,
    pub task_group_id: TaskGroupId,
    pub name: String,
    pub information: String,
    pub creator: UserId,

}

impl Task {
    pub async fn create(
        data: &TaskBuilder,
        conn: &SqlitePool,
    ) ->  Result<String, String> {
        unimplemented!()
    }

    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO tasks (
                id, project_id, task_group_id, 
                name, information, creator, 
                assignee
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
            ",
            self.id.0,
            self.project_id.0,
            self.task_group_id.0,
            self.name,
            self.information,
            self.creator.0,
            self.assignee.0
        )
        .execute(conn)
        .await?;

        Ok(())
    }
}