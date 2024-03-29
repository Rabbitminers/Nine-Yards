use chrono::{NaiveDateTime, Local};

use crate::database::Database;
use futures::{TryStreamExt};

use super::ids::{TaskId, ProjectId, TaskGroupId, UserId, SubTaskId, ProjectMemberId, TaskLabelId};
use super::DatabaseError;

#[derive(Serialize, Deserialize)]
pub struct TaskGroup {
    pub id: TaskGroupId,
    pub project_id: ProjectId,
    pub name: String,
    pub position: i64 // Got to have room for your 9223372036854775807 task groups!
}

#[derive(Serialize, Deserialize, Validate)]
pub struct TaskGroupBuilder {
    #[validate(length(min = 3, max = 30))]
    pub name: String,
}

impl TaskGroupBuilder {
    pub async fn create(
        self,
        project_id: ProjectId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<TaskGroup, super::DatabaseError> {
        if TaskGroup::get_by_name(project_id.clone(), &self.name, &mut *transaction).await?.is_some() {
            return Err(super::DatabaseError::AlreadyExists);
        }

        let id = TaskGroupId::generate(&mut *transaction).await?;

        let position = sqlx::query!(
            "
            SELECT COALESCE(MAX(position), 0) + 1 
            AS next_available_position
            FROM task_groups
            WHERE project_id = $1
            ",
            project_id
        )
        .fetch_one(&mut *transaction)
        .await?
        .next_available_position
        .unwrap_or(0) as i64;

        let group = TaskGroup {
            id,
            project_id,
            name: self.name,
            position
        };

        group.insert(&mut *transaction).await?;
        
        Ok(group)
    }
}

impl TaskGroup {
    pub async fn get<'a, E>(
        task_group_id: TaskGroupId,
        transaction: E
    ) -> Result<Option<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        let result = sqlx::query!(
            "
            SELECT id, project_id, name, position
            FROM task_groups
            WHERE id = $1
            ",
            task_group_id, // Prevent unauthorized reads
        )
        .fetch_optional(transaction)
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

    pub async fn get_by_name(
        project_id: ProjectId,
        name: &str,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT id, project_id, name, position
            FROM task_groups
            WHERE name = $1
            AND project_id = $2
            ",
            name, 
            project_id // Prevent unauthorized reads
        )
        .fetch_optional(&mut *transaction)
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
        transaction: &mut sqlx::Transaction<'_, Database>,
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
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn move_surrounding(
        project_id: ProjectId,
        new_position: i64,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE task_groups
            SET position = position + 1
            WHERE position >= $1
            AND project_id = $2
            ",
            new_position,
            project_id
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn get_tasks(
        project_id: ProjectId,
        group_id: TaskGroupId,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<Vec<Task>, sqlx::error::Error> {
        let tasks = sqlx::query!(
            "
            SELECT id, project_id, task_group_id, 
            name, information, creator, due, 
            primary_colour, accent_colour, position,
            created
            FROM tasks
            WHERE task_group_id = $1
            AND project_id = $2
            ",
            group_id,
            project_id
        )
        .fetch_many(&mut *transaction)
        .try_filter_map(|e| async {
            Ok(e.right().map(|m| Task {
                id: TaskId(m.id),
                project_id: ProjectId(m.project_id),
                task_group_id: TaskGroupId(m.task_group_id),
                name: m.name,
                information: m.information,
                creator: UserId(m.creator),
                due: m.due,
                primary_colour: m.primary_colour,
                accent_colour: m.accent_colour,
                position: m.position,
                created: m.created
            }))
        })
        .try_collect::<Vec<Task>>()
        .await?;

        Ok(tasks)
    }

    pub async fn get_tasks_full<'a, E>(
        group_id: TaskGroupId,
        exec: E,
    ) -> Result<Vec<TaskResponse>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database> + Copy,
    {
        let query = sqlx::query!(
            "
            SELECT id, project_id, task_group_id, 
            name, information, creator, due, 
            primary_colour, accent_colour, position,
            created
            FROM tasks
            WHERE task_group_id = $1
            ",
            group_id,
        )
        .fetch_many(exec)
        .try_filter_map(|e| async {
            if let Some(m) = e.right() {
                let id = TaskId(m.id);
                let sub_tasks = Task::get_sub_tasks(id.clone(), exec).await?;

                Ok(Some(Ok(TaskResponse {
                    id,
                    project_id: ProjectId(m.project_id),
                    task_group_id: TaskGroupId(m.task_group_id),
                    name: m.name,
                    information: m.information,
                    creator: UserId(m.creator),
                    due: m.due,
                    primary_colour: m.primary_colour,
                    accent_colour: m.accent_colour,
                    position: m.position,
                    sub_tasks,
                    created: m.created
                })))
            } else {
                Ok(None)
            }
        })
        .try_collect::<Vec<Result<TaskResponse, sqlx::error::Error>>>()
        .await?;

        let responses = query
            .into_iter()
            .collect::<Result<Vec<TaskResponse>, sqlx::error::Error>>()?;

        Ok(responses)
    }

    pub async fn is_in_project(
        id: TaskGroupId,
        project_id: ProjectId,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<bool, sqlx::error::Error> {
        let query = sqlx::query!(
            "
            SELECT exists(
                SELECT 1
                FROM task_groups
                WHERE id = $1
                AND project_id = $2
            ) AS is_in_project
            ",
            id,
            project_id
        )
        .fetch_one(&mut *transaction)
        .await?;

        Ok(query.is_in_project.is_positive())
    }

    pub async fn remove(
        id: TaskGroupId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE task_group_id = $1
            ",
            id,
        )
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            "
            DELETE FROM task_groups
            WHERE id = $1
            ",
            id
        )
        .execute(&mut *transaction)
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
    pub information: Option<String>,
    pub creator: UserId,
    pub due: Option<NaiveDateTime>,
    pub primary_colour: String,
    pub accent_colour: String,
    pub position: i64,
    pub created: NaiveDateTime
}

#[derive(Serialize, Deserialize, Validate)]
pub struct TaskBuilder {
    #[validate(length(min = 3, max = 30))]
    pub name: String,
    pub creator: UserId,
    pub primary_colour: String,
    pub accent_colour: String,
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: TaskId,
    pub project_id: ProjectId,
    pub task_group_id: TaskGroupId,
    pub name: String,
    pub information: Option<String>,
    pub creator: UserId,
    pub due: Option<NaiveDateTime>,
    pub primary_colour: String,
    pub accent_colour: String,
    pub position: i64,
    pub sub_tasks: Vec<SubTask>,
    pub created: NaiveDateTime,
}

impl TaskBuilder {
    pub async fn create(
        self,
        project_id: ProjectId,
        task_group_id: TaskGroupId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Task, super::DatabaseError> {
        if TaskGroup::is_in_project(
            task_group_id.clone(), 
            project_id.clone(), 
            &mut *transaction
        ).await? {
            return Err(DatabaseError::Other("Task group not found".to_string()));
        }

        let id = TaskId::generate(&mut *transaction).await?;

        let position = sqlx::query!(
            "
            SELECT COALESCE(MAX(position), 0) + 1 
            AS next_available_position
            FROM tasks
            WHERE task_group_id = $1
            ",
            task_group_id
        )
        .fetch_one(&mut *transaction)
        .await?
        .next_available_position
        .unwrap_or(0) as i64;

        let task = Task {
            id,
            project_id,
            task_group_id,
            name: self.name,
            information: None,
            creator: self.creator,
            due: None,
            primary_colour: self.primary_colour,
            accent_colour: self.accent_colour,
            position,
            created: Local::now().naive_local()
        };

        task.insert(&mut *transaction).await?;

        Ok(task)
    }
}

impl Task {
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
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
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn get<'a, E>(
        task_id: TaskId,
        transaction: E,
    ) -> Result<Option<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        let query = sqlx::query!(
            "
            SELECT id, project_id, task_group_id, 
            name, information, creator, due, 
            primary_colour, accent_colour, position,
            created
            FROM tasks
            WHERE id = $1
            ",
            task_id,
        )
        .fetch_optional(transaction)
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
                position: row.position,
                created: row.created,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_full<'a, E>(
        task_id: TaskId,
        transaction: E,
    ) -> Result<Option<TaskResponse>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database> + Copy,
    {
        let query = sqlx::query!(
            "
            SELECT id, project_id, task_group_id, 
            name, information, creator, due, 
            primary_colour, accent_colour, position,
            created
            FROM tasks
            WHERE id = $1
            ",
            task_id,
        )
        .fetch_optional(transaction)
        .await?;
        
        if let Some(row) = query {
            let sub_tasks = Self::get_sub_tasks(task_id, transaction).await?;

            Ok(Some(TaskResponse {
                id: TaskId(row.id),
                project_id: ProjectId(row.project_id),
                task_group_id: TaskGroupId(row.task_group_id),
                name: row.name,
                information: row.information,
                creator: UserId(row.creator),
                due: row.due,
                primary_colour: row.primary_colour,
                accent_colour: row.accent_colour,
                position: row.position,
                sub_tasks,
                created: row.created,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_sub_tasks<'a, E>(
        task_id: TaskId,
        transaction: E
    ) -> Result<Vec<SubTask>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        let sub_tasks = sqlx::query!(
            "
            SELECT id, task_id, project_id,
            assignee, body, weight, position, 
            completed
            FROM sub_tasks
            WHERE task_id = $1
            ",
            task_id
        )
        .fetch_many(transaction)
        .try_filter_map(|e| async {
            Ok(e.right().map(|m| SubTask {
                id: SubTaskId(m.id),
                task_id: TaskId(m.task_id),
                project_id: ProjectId(m.project_id),
                assignee: m.assignee.map(ProjectMemberId),
                body: m.body,
                position: m.position,
                weight: m.weight,
                completed: m.completed
            }))
        })
        .try_collect::<Vec<SubTask>>()
        .await?;

        Ok(sub_tasks)
    }

    pub async fn remove(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM sub_tasks
            WHERE task_id = $1
            ",
            self.id
        )
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE id = $1
            ",
            self.id
        )
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            "
            UPDATE tasks
            SET position = position - 1
            WHERE position >= $1
            AND project_id = $2
            ",
            self.position,
            self.project_id
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }   
}


#[derive(Serialize, Deserialize)]
pub struct SubTask {
    pub id: SubTaskId,
    pub task_id: TaskId,
    pub project_id: ProjectId,
    pub assignee: Option<ProjectMemberId>,
    pub body: String,
    pub weight: Option<i64>,
    pub position: i64,
    pub completed: bool
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SubTaskBuilder {
    #[validate(length(min = 3, max = 90))]
    pub body: String,
}

impl SubTaskBuilder {
    pub async fn create(
        self,
        project_id: ProjectId,
        task_id: TaskId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<SubTask, super::DatabaseError> {
        let id = SubTaskId::generate(&mut *transaction).await?;

        let position = sqlx::query!(
            "
            SELECT COALESCE(MAX(position), 0) + 1
            AS next_available_position
            FROM sub_tasks
            WHERE task_id = $1
            ",
            task_id
        )
        .fetch_one(&mut *transaction)
        .await?
        .next_available_position
        .unwrap_or(0) as i64;

        let sub_task = SubTask {
            id,
            task_id,
            project_id,
            assignee: None,
            body: self.body,
            weight: None,
            position,
            completed: false
        };

        sub_task.insert(&mut *transaction).await?;
        
        Ok(sub_task)
    }
}

impl SubTask {
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
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
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn get<'a, E>(
        id: SubTaskId,
        transaction: E,
    ) -> Result<Option<Self>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        let query = sqlx::query!(
            "
            SELECT id, task_id, project_id,
            assignee, body, weight, position, 
            completed
            FROM sub_tasks
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(transaction)
        .await?;

        if let Some(row) = query {
            Ok(Some(Self {
                id: SubTaskId(row.id),
                task_id: TaskId(row.task_id),
                project_id: ProjectId(row.project_id),
                assignee: row.assignee.map(ProjectMemberId),
                body: row.body,
                weight: row.weight,
                position: row.position,
                completed: row.completed
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn assign(
        user: Option<ProjectMemberId>,
        id: SubTaskId,
        transaction: &mut sqlx::Transaction<'_, Database>,
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
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn set_status(
        id: SubTaskId,
        completed: bool,
        transaction: &mut sqlx::Transaction<'_, Database>,
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
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn remove(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM sub_tasks
            WHERE id = $1
            ",
            self.id
        )
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            "
            UPDATE sub_tasks
            SET position = position - 1
            WHERE position >= $1
            ",
            self.position
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }
}

#[derive(Serialize)]
pub struct TaskLabel {
    id: TaskLabelId,
    task_id: TaskId,
    project_id: ProjectId,
    body: String,
    colour: String
}

#[derive(Deserialize)]
pub struct TaskLabelBuilder {
    body: String,
    colour: String
}

impl TaskLabelBuilder {
    pub async fn create(
        self,
        task_id: TaskId, // Validate these two match at route level
        project_id: ProjectId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<TaskLabel, super::DatabaseError> {
        let id = TaskLabelId::generate(&mut *transaction).await?;

        let label = TaskLabel {
            id,
            task_id,
            project_id,
            body: self.body,
            colour: self.colour,
        };
        
        label.insert(&mut *transaction).await?;
        
        Ok(label)
    }
}

impl TaskLabel {
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        
        sqlx::query!(
            "
            INSERT INTO labels (
                id, task_id, project_id,
                body, colour
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.task_id,
            self.project_id,
            self.body,
            self.colour
        )
        .execute(transaction)
        .await?;

        Ok(())
    }
}