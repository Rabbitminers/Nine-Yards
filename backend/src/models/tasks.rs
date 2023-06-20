use chrono::NaiveDateTime;
use sqlx::SqlitePool;

use super::{
    ids::{
        TaskId, 
        ProjectId, 
        TaskGroupId, 
        UserId, SubTaskId, ProjectMemberId, 
    }, 
    users::User
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
        self,
        conn: &SqlitePool
    ) -> Result<TaskGroup, super::DatabaseError> {
        if let Ok(Some(_)) = TaskGroup::find_by_name(&self.name, conn).await {
            return Err(super::DatabaseError::AlreadyExists);
        }

        let id = TaskGroupId::generate(&conn).await?;

        let group = TaskGroup {
            id,
            project_id: self.project_id,
            name: self.name,
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
            self.id,
            self.project_id,
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
            self.id
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            DELETE FROM task_groups
            WHERE id = $1
            ",
            self.id
        )
        .execute(conn)
        .await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct Task {
    pub id: TaskId,
    pub project_id: ProjectId,
    pub task_group_id: TaskGroupId,
    #[validate(length(min = 3, max = 30))]
    pub name: String,
    pub information: String,
    pub creator: UserId,
    pub due: Option<NaiveDateTime>,
    pub primary_colour: String,
    pub accent_colour: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct TaskBuilder {
    pub project_id: ProjectId,
    pub task_group_id: TaskGroupId,
    #[validate(length(min = 3, max = 30))]
    pub name: String,
    pub information: String,
    pub creator: UserId,
    pub primary_colour: String,
    pub accent_colour: String,
}

impl TaskBuilder {
    pub async fn create(
        self,
        conn: &SqlitePool
    ) -> Result<Task, super::DatabaseError> {
        let id = TaskId::generate(&conn).await?;

        let task = Task {
            id,
            project_id: self.project_id,
            task_group_id: self.task_group_id,
            name: self.name,
            information: self.information,
            creator: self.creator,
            due: None,
            primary_colour: self.primary_colour,
            accent_colour: self.accent_colour,
        };

        task.insert(conn).await?;

        Ok(task)
    }
}

impl Task {
    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO tasks (
                id, project_id, task_group_id, 
                name, information, creator, due, 
                primary_colour, accent_colour
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8,
                $9
            )
            ",
            self.id,
            self.project_id,
            self.task_group_id,
            self.name,
            self.information,
            self.creator,
            self.due,
            self.primary_colour,
            self.accent_colour
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn get(
        id: TaskId,
        conn: &SqlitePool
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let query = sqlx::query!(
            "
            SELECT id, project_id, task_group_id, 
                 name, information, creator, due, 
                 primary_colour, accent_colour
            FROM tasks
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(conn)
        .await?;
        
        if let Some(row) = query {
            Ok(Some(Self {
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
        } else {
            Ok(None)
        }
    }

    pub async fn delete(
        id: TaskId,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM sub_tasks
            WHERE task_id = $1
            ",
            id
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE id = $1
            ",
            id
        )
        .execute(conn)
        .await?;

        Ok(())
    }   
}


#[derive(Serialize, Deserialize)]
pub struct SubTask {
    pub id: SubTaskId,
    pub task_id: TaskId,
    pub assignee: Option<ProjectMemberId>,
    pub body: String,
    pub completed: bool
}

#[derive(Serialize, Deserialize)]
pub struct SubTaskBuilder {
    pub task_id: TaskId,
    pub body: String,
}

impl SubTaskBuilder {
    pub async fn create(
        self,
        conn: &SqlitePool
    ) -> Result<SubTask, super::DatabaseError> {
        let id = SubTaskId::generate(&conn).await?;

        let sub_task = SubTask {
            id,
            task_id: self.task_id,
            assignee: None,
            body: self.body,
            completed: false
        };

        sub_task.insert(conn).await?;
        
        Ok(sub_task)
    }
}

impl SubTask {
    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO sub_tasks (
                id, task_id, assignee, 
                body, completed
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.task_id,
            self.assignee,
            self.body,
            self.completed
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn assign(
        user: Option<ProjectMemberId>,
        id: SubTaskId,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE sub_tasks
            SET assignee = $1
            WHERE id = $2
            ",
            user,
            id
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn set_status(
        id: SubTaskId,
        completed: bool,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE sub_tasks
            SET completed = $1
            WHERE id = $2
            ",
            completed,
            id
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn remove(
        id: SubTaskId,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM sub_tasks
            WHERE id = $1
            ",
            id
        )
        .execute(conn)
        .await?;

        Ok(())
    }
}
