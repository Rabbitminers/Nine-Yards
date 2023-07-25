use chrono::{NaiveDateTime, Utc};
use futures::TryStreamExt;
use validator::Validate;

use crate::{database::Database, error};

use super::id::{TaskGroupId, ProjectId, TaskId, ProjectMemberId, SubTaskId};

#[derive(Serialize, Deserialize)]
pub struct TaskGroup {
    // The task group's id (unique)
    pub id: TaskGroupId,
    // The parent project's id
    pub project_id: ProjectId,
    // The task group's name (3 -> 30 characters)
    pub name: String,
    // The position of the task group in the project
    pub position: i64 // Got to have room for your 9223372036854775807 task groups!
}

#[derive(Serialize, Deserialize, Validate)]
pub struct TaskGroupBuilder {
    // The task group's name (3 -> 30 characters)
    #[validate(length(min = 3, max = 30))]
    pub name: String,
}

impl TaskGroup {
    /// Creates a new `TaskGroup` and inserts it into the `task_groups` table in the database.
    ///
    /// # Arguments
    ///
    /// * `form`: A `TaskGroupBuilder` containing the details to create the task group.
    /// * `project_id`: The `ProjectId` to associate the new task group with.
    /// * `transaction`: A mutable reference to a `sqlx::Transaction` representing the database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Self, error::ApiError>`, where:
    /// - `Ok(group)` is returned with the newly created `TaskGroup` instance if the insertion is successful.
    /// - An `error::ApiError` is returned if there is an error executing the database query or generating the task group ID.
    ///
    pub async fn create(
        form: TaskGroupBuilder,
        project_id: ProjectId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, error::ApiError> {
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
        .fetch_one(&mut **transaction)
        .await?
        .next_available_position as i64; // Defaults to 0

        let group = TaskGroup { 
            id, 
            project_id, 
            name: form.name, 
            position 
        };

        group.insert(&mut **transaction).await?;

        Ok(group)
    }  

    /// Removes a `TaskGroup` from the database along with its associated tasks.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction` representing the database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), error::ApiError>`, where:
    /// - `Ok(())` is returned if the removal is successful and the task group is deleted from the database.
    /// - An `error::ApiError` is returned if there is an error executing the database query.
    ///
    pub async fn remove(
        &self, // Take self to prevent double get
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), error::ApiError> {
        // TODO: Remove associated sub-tasks

        // Remove associated tasks
        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE task_group_id = $1
            ",
            self.id,
        )
        .execute(&mut **transaction)
        .await?;
        // Remove associated task groups
        sqlx::query!(
            "
            DELETE FROM task_groups
            WHERE id = $1
            ",
            self.id
        )
        .execute(&mut **transaction)
        .await?;
        // Move other task groups back to fill gap
        sqlx::query!(
            "
            UPDATE task_groups
            SET position = position - 1
            WHERE position >= $1
            AND project_id = $2
            ",
            self.position,
            self.project_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

impl TaskGroup {
    /// Inserts a new `TaskGroup` into the `task_groups` table in the database.
    ///
    /// # Arguments
    ///
    /// * `executor`: A type implementing `sqlx::Executor` that represents the database connection.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the insertion is successful, and the `TaskGroup` is added to the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn insert<'a, E>(
        &self,
        executor: E,
    ) -> Result<(), sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>
    {
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
        .execute(executor)
        .await?;

        Ok(())
    }

    /// Retrieves a `TaskGroup` from the `task_groups` table based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id`: The `TaskGroupId` of the `TaskGroup` to retrieve.
    /// * `executor`: A type implementing `sqlx::Executor` that represents the database connection.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Option<Self>, sqlx::error::Error>`, where:
    /// - `Ok(Some(group))` is returned with the retrieved `TaskGroup` instance if found in the database.
    /// - `Ok(None)` is returned if no `TaskGroup` with the specified ID is found in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get<'a, E>(
        id: TaskGroupId,
        executor: E,
    ) -> Result<Option<Self>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        sqlx::query_as!(
            TaskGroup,
            "
            SELECT id, project_id, name, position
            FROM task_groups
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(executor)
        .await
    }

    /// Retrieves a list of `TaskGroup` instances from the `task_groups` table
    /// based on the specified column and value.
    ///
    /// # Arguments
    ///
    /// * `column`: The name of the column to use in the WHERE clause of the query.
    /// * `value`: The value to use for filtering the results in the specified column.
    /// * `executor`: A type implementing `sqlx::Executor` that represents the database connection.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Vec<Self>, sqlx::error::Error>`, where:
    /// - `Ok(groups)` is returned with the list of `TaskGroup` instances if found in the database.
    /// - An empty vector is returned if no `TaskGroup` with the specified column and value is found in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get_many<'a, E>(
        column: &str,
        value: String,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database>,
    {
        sqlx::query_as!(
            TaskGroup,
            "
            SELECT id, project_id, name, position
            FROM task_groups
            WHERE $1 = $2
            ",
            column,
            value
        )
        .fetch_all(executor)
        .await
    }

    /// Retrieves a list of `TaskGroup` instances for a given `ProjectId`.
    ///
    /// # Arguments
    ///
    /// * `project_id`: The `ProjectId` for which to retrieve the task groups.
    /// * `executor`: A type implementing `sqlx::Executor` that represents the database connection.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Vec<Self>, sqlx::error::Error>`, where:
    /// - `Ok(groups)` is returned with the list of `TaskGroup` instances if found in the database.
    /// - An empty vector is returned if the project does not have any task groups in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get_from_project<'a, E>(
        project_id: ProjectId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        Self::get_many("project_id", project_id.0, executor).await
    }
}

#[derive(Serialize, Validate)]
pub struct Task {
    // The task's id (unique)
    pub id: TaskId,
    // The parent project's id
    pub project_id: ProjectId,
    // The parent task group's id
    pub task_group_id: TaskGroupId,
    // The task's name (3 -> 30 characters)
    #[validate(length(min = 3, max = 30))]
    pub name: String,
    // The task's description
    pub information: Option<String>,
    // The task's creator's membership id
    pub creator: ProjectMemberId,
    // The task's due date (if any) (ms)
    pub due: Option<NaiveDateTime>,
    // The task's primary colour (hex) - background
    pub primary_colour: String,
    // The task's accent colour (hex) - progress bar
    pub accent_colour: String,
    // The task's position in the task group
    pub position: i64,
    // The time the task was created (ms)
    pub created: NaiveDateTime
}

// TODO: Implement hex code validators
#[derive(Deserialize, Validate)]
pub struct TaskBuilder {
    // The name of the project (3 -> 30 characters)
    #[validate(length(min = 3, max = 30))]
    pub name: String,
    // The task's primary colour (hex) - background
    pub primary_colour: String,
    // The task's accent colour (hex) - progress bar
    pub accent_colour: String,
}

#[derive(Serialize, Validate)]
pub struct FullTask {
    // The task
    pub task: Task,
    // The tasks sub-tasks
    pub sub_tasks: Vec<SubTask>
}

impl Task {
    /// Creates a new `Task` and inserts it into the `tasks` table in the database.
    ///
    /// # Arguments
    ///
    /// * `task_group_id`: The `TaskGroupId` to associate the new task with.
    /// * `project_id`: The `ProjectId` to associate the new task with.
    /// * `creator`: The `ProjectMemberId` of the project member who created the task.
    /// * `form`: A `TaskBuilder` containing the details to create the task.
    /// * `transaction`: A mutable reference to a `sqlx::Transaction` representing the database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Task, error::ApiError>`, where:
    /// - `Ok(task)` is returned with the newly created `Task` instance if the insertion is successful.
    /// - An `error::ApiError` is returned if there is an error executing the database query or generating the task ID.
    ///
    pub async fn create(
        task_group_id: TaskGroupId,
        project_id: ProjectId,
        creator: ProjectMemberId, // Assume the membership was collected from the group id
        form: TaskBuilder,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Task, error::ApiError> {
        let id = TaskId::generate(&mut *transaction).await?;

        let position: i64 = sqlx::query!(
            " 
            SELECT COALESCE(MAX(position), 0) + 1 
            AS next_available_position
            FROM tasks
            WHERE task_group_id = $1
            ",
            task_group_id
        )
        .fetch_one(&mut **transaction)
        .await?
        .next_available_position as i64;

        let now = Utc::now();
        
        let task = Task {
            id,
            project_id,
            task_group_id,
            name: form.name,
            information: None,
            creator,
            due: None,
            primary_colour: form.primary_colour,
            accent_colour: form.accent_colour,
            position,
            created: now.naive_utc()
        };

        task.insert(&mut *transaction).await?;

        Ok(task)
    }

    /// Removes a `Task` from the database along with its associated sub-tasks.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction` representing the database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), error::ApiError>`, where:
    /// - `Ok(())` is returned if the removal is successful and the task is deleted from the database.
    /// - An `error::ApiError` is returned if there is an error executing the database query.
    ///
    pub async fn remove(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), error::ApiError> {
        // Remove the associated sub-tasks
        sqlx::query!(
            "
            DELETE FROM sub_tasks
            WHERE task_id = $1
            ",
            self.id
        )
        .execute(&mut **transaction)
        .await?;
        // Remove the task itself
        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE id = $1
            ",
            self.id
        )
        .execute(&mut **transaction)
        .await?;
        // Update the other tasks' positions to fill the gap left
        sqlx::query!(
            "
            UPDATE tasks
            SET position = position - 1
            WHERE position > $1
            AND project_id = $2
            ",
            self.position,
            self.project_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

impl Task {
    /// Inserts a new task into the database using the given SQL transaction.
    ///
    /// # Parameters
    ///
    /// - `transaction`: A mutable reference to the SQL transaction in which the insertion will occur.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the insertion is successful.
    /// - `Err`: If an error occurs during the insertion.
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
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// Retrieves a task with the specified `TaskGroupId` from the database.
    ///
    /// # Parameters
    ///
    /// - `id`: The `TaskGroupId` of the task to retrieve.
    /// - `executor`: An SQL executor used to execute the database query.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(task))`: If a task with the specified `TaskGroupId` is found in the database.
    /// - `Ok(None)`: If no task is found with the specified `TaskGroupId`.
    /// - `Err`: If an error occurs during the retrieval.
    pub async fn get<'a, E>(
        id: TaskGroupId,
        executor: E,
    ) -> Result<Option<Self>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        sqlx::query_as!(
            Task,
            "
            SELECT id, project_id, task_group_id, 
            name, information, creator, due, 
            primary_colour, accent_colour, position,
            created
            FROM tasks
            WHERE id = $1
            ",
            id,
        )
        .fetch_optional(executor)
        .await
    }

    /// Retrieves a full task (including sub-tasks) with the specified `TaskGroupId` from the database.
    ///
    /// # Parameters
    ///
    /// - `id`: The `TaskGroupId` of the task to retrieve.
    /// - `executor`: An SQL executor used to execute the database query.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(full_task))`: If a full task with the specified `TaskGroupId` is found in the database.
    /// - `Ok(None)`: If no task is found with the specified `TaskGroupId`.
    /// - `Err`: If an error occurs during the retrieval.
    pub async fn get_full<'a, E>(
        id: TaskGroupId,
        executor: E,
    ) -> Result<Option<FullTask>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database> + Copy
    {   
        // Dont use join query to prevent parent task being fetched repeatedly
        match Self::get(id, executor).await? {
            Some(task) => {
                let sub_tasks = SubTask::get_from_task(task.id.clone(), executor).await?;
                Ok(Some(FullTask { task, sub_tasks }))
            }
            None => Ok(None)
        }
    }

    /// Retrieves multiple tasks that match the specified column and value from the database.
    ///
    /// # Parameters
    ///
    /// - `column`: The column name to match against.
    /// - `value`: The value to match with the specified column.
    /// - `executor`: An SQL executor used to execute the database query.
    ///
    /// # Returns
    ///
    /// - `Ok(tasks)`: A vector of tasks that match the specified column and value.
    /// - `Err`: If an error occurs during the retrieval.
    pub async fn get_many<'a, E>(
        column: &str,
        value: String,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database>,
    {
        sqlx::query_as!(
            Task,
            "
            SELECT id, project_id, task_group_id, 
            name, information, creator, due, 
            primary_colour, accent_colour, position,
            created
            FROM tasks
            WHERE $1 = $2
            ",
            column,
            value
        )
        .fetch_all(executor)
        .await
    }

    /// Retrieves multiple full tasks (including sub-tasks) that match the specified column and value from the database.
    ///
    /// # Parameters
    ///
    /// - `column`: The column name to match against.
    /// - `value`: The value to match with the specified column.
    /// - `executor`: An SQL executor used to execute the database query.
    ///
    /// # Returns
    ///
    /// - `Ok(full_tasks)`: A vector of full tasks that match the specified column and value.
    /// - `Err`: If an error occurs during the retrieval.
    pub async fn get_many_full<'a, E>(
        column: &str,
        value: String,
        executor: E,
    ) -> Result<Vec<FullTask>, sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database> + Copy,
    {
        let responses = sqlx::query_as!(
            Task,
            "
            SELECT id, project_id, task_group_id, 
            name, information, creator, due, 
            primary_colour, accent_colour, position,
            created
            FROM tasks
            WHERE $1 = $2
            ",
            column,
            value
        )
        .fetch_many(executor)
        .try_filter_map(|e| async {
            if let Some(task) = e.right() {
                let sub_tasks = SubTask::get_from_task(task.id.clone(), executor).await?;
                Ok(Some(Ok(FullTask { task, sub_tasks })))
            } else {
                Ok(None)
            }
        })
        .try_collect::<Vec<Result<FullTask, sqlx::error::Error>>>()
        .await?;

        responses.into_iter().collect::<Result<Vec<FullTask>, sqlx::error::Error>>()
    }

    /// Retrieves multiple full tasks (including sub-tasks) associated with the specified `ProjectId` from the database.
    ///
    /// # Parameters
    ///
    /// - `project_id`: The `ProjectId` of the project whose tasks will be retrieved.
    /// - `executor`: An SQL executor used to execute the database query.
    ///
    /// # Returns
    ///
    /// - `Ok(full_tasks)`: A vector of full tasks associated with the specified `ProjectId`.
    /// - `Err`: If an error occurs during the retrieval.
    pub async fn get_from_project<'a, E>(
        project_id: ProjectId,
        executor: E,
    ) -> Result<Vec<FullTask>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database> + Copy,
    {
        Self::get_many_full("project_id", project_id.0, executor).await
    }

    /// Retrieves multiple full tasks (including sub-tasks) associated with the specified `TaskGroupId` from the database.
    ///
    /// # Parameters
    ///
    /// - `task_group_id`: The `TaskGroupId` of the task group whose tasks will be retrieved.
    /// - `executor`: An SQL executor used to execute the database query.
    ///
    /// # Returns
    ///
    /// - `Ok(full_tasks)`: A vector of full tasks associated with the specified `TaskGroupId`.
    /// - `Err`: If an error occurs during the retrieval.
    pub async fn get_from_task_group<'a, E>(
        task_group_id: TaskGroupId,
        executor: E,
    ) -> Result<Vec<FullTask>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database> + Copy,
    {
        Self::get_many_full("task_group_id", task_group_id.0, executor).await
    }
}

#[derive(Serialize, Validate)]
pub struct SubTask {
    // The sub-task's id (unique)
    pub id: SubTaskId,
    // The parent task's id
    pub task_id: TaskId,
    // The parent project's id
    pub project_id: ProjectId,
    // The assigned member's id (optional)
    pub assignee: Option<String>, // Kept as string to for simplicity
    // The sub-task's description (0 -> 90 chars)
    pub body: String,
    // The weight of the sub-task when calculating completion (optional) (default 100)
    pub weight: Option<i64>,
    // The position of the sub-task in the task
    pub position: i64,
    // Weather the sub task is completed
    pub completed: bool
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SubTaskBuilder {
    // The sub-task's description (0 -> 90 chars)
    #[validate(length(min = 3, max = 90))]
    pub body: String,
}

impl SubTask {
    /// Creates a new sub-task and inserts it into the database within the provided transaction.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the parent task associated with the sub-task.
    /// * `project_id` - The ID of the project to which the task belongs. This value is assumed to be collected from the `task_id`.
    /// * `form` - A `SubTaskBuilder` containing the information to create the sub-task.
    /// * `transaction` - A mutable reference to the SQLx transaction that the creation and insertion will be performed within.
    ///
    /// # Returns
    ///
    /// This function returns `Result<Self, error::ApiError>`.
    ///
    /// - `Ok(sub_task)`: The created `SubTask` object if the insertion is successful.
    /// - `Err`: If an error occurs during the creation or insertion process, `error::ApiError` will be returned.
    pub async fn create(
        task_id: TaskId,
        project_id: ProjectId, // Assume the project was collected from the task id
        form: SubTaskBuilder,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, error::ApiError> {
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
        .fetch_one(&mut **transaction)
        .await?
        .next_available_position as i64;

        let sub_task = SubTask {
            id,
            task_id,
            project_id,
            assignee: None,
            body: form.body,
            weight: None,
            position,
            completed: false
        };

        sub_task.insert(&mut *transaction).await?;
        
        Ok(sub_task)
    }

    /// Removes the sub-task from the database within the provided transaction.
    ///
    /// # Arguments
    ///
    /// * `transaction` - A mutable reference to the SQLx transaction that the removal will be performed within.
    ///
    /// # Returns
    ///
    /// This function returns `Result<(), error::ApiError>`.
    ///
    /// - `Ok(())`: If the sub-task is successfully removed from the database.
    /// - `Err`: If an error occurs during the removal process, `error::ApiError` will be returned.
    pub async fn remove(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), error::ApiError> {
        sqlx::query!(
            "
            DELETE FROM sub_tasks
            WHERE id = $1
            ",
            self.id
        )
        .execute(&mut **transaction)
        .await?;

        sqlx::query!(
            "
            UPDATE sub_tasks
            SET position = position - 1
            WHERE position > $1
            ",
            self.position
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

impl SubTask {
    /// Inserts a new sub-task into the database using the provided transaction.
    ///
    /// # Arguments
    ///
    /// * `transaction` - A mutable reference to the SQLx transaction that the insert operation will be performed within.
    ///
    /// # Returns
    ///
    /// This function returns `Result<(), sqlx::error::Error>`.
    ///
    /// - `Ok(())`: If the sub-task is successfully inserted into the database.
    /// - `Err`: If an error occurs during the insertion process.
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
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }


    /// Retrieves a sub-task by its ID from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the sub-task to retrieve.
    /// * `executor` - An SQLx executor (e.g., a connection pool) to execute the query.
    ///
    /// # Returns
    ///
    /// This function returns `Result<Option<SubTask>, sqlx::error::Error>`.
    ///
    /// - `Ok(Some(sub_task))`: If a sub-task with the specified ID is found, it returns the retrieved `SubTask` inside `Some`.
    /// - `Ok(None)`: If no sub-task with the specified ID is found, it returns `None`.
    /// - `Err`: If an error occurs during the retrieval process.
    pub async fn get<'a, E>(
        id: TaskGroupId,
        executor: E,
    ) -> Result<Option<Self>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        sqlx::query_as!(
            SubTask,
            "
            SELECT id, task_id, project_id,
            assignee, body, weight, position, 
            completed
            FROM sub_tasks
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(executor)
        .await
    }

    /// Retrieves a list of sub-tasks that match a specific column value from the database.
    ///
    /// # Arguments
    ///
    /// * `column` - The column name to filter the sub-tasks.
    /// * `value` - The value to match in the specified column.
    /// * `executor` - An SQLx executor (e.g., a connection pool) to execute the query.
    ///
    /// # Returns
    ///
    /// This function returns `Result<Vec<SubTask>, sqlx::error::Error>`.
    ///
    /// - `Ok(sub_tasks)`: A vector of sub-tasks that match the specified column value.
    /// - `Err`: If an error occurs during the retrieval process.
    pub async fn get_many<'a, E>(
        column: &str,
        value: String,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database>,
    {
        sqlx::query_as!(
            SubTask,
            "
            SELECT id, task_id, project_id,
            assignee, body, weight, position, 
            completed
            FROM sub_tasks
            WHERE $1 = $2
            ",
            column,
            value
        )
        .fetch_all(executor)
        .await
    }

    /// Retrieves a list of sub-tasks associated with a specific project from the database.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to retrieve sub-tasks from.
    /// * `executor` - An SQLx executor (e.g., a connection pool) to execute the query.
    ///
    /// # Returns
    ///
    /// This function returns `Result<Vec<SubTask>, sqlx::error::Error>`.
    ///
    /// - `Ok(sub_tasks)`: A vector of sub-tasks associated with the specified `ProjectId`.
    /// - `Err`: If an error occurs during the retrieval process.
    pub async fn get_from_project<'a, E>(
        project_id: ProjectId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        Self::get_many("project_id", project_id.0, executor).await
    }

    /// Retrieves a list of sub-tasks associated with a specific task group from the database.
    ///
    /// # Arguments
    ///
    /// * `task_group_id` - The ID of the task group to retrieve sub-tasks from.
    /// * `executor` - An SQLx executor (e.g., a connection pool) to execute the query.
    ///
    /// # Returns
    ///
    /// This function returns `Result<Vec<SubTask>, sqlx::error::Error>`.
    ///
    /// - `Ok(sub_tasks)`: A vector of sub-tasks associated with the specified `TaskGroupId`.
    /// - `Err`: If an error occurs during the retrieval process.
    pub async fn get_from_task_group<'a, E>(
        task_group_id: TaskGroupId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        Self::get_many("task_group_id", task_group_id.0, executor).await
    }

    /// Retrieves a list of sub-tasks associated with a specific task from the database.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task to retrieve sub-tasks from.
    /// * `executor` - An SQLx executor (e.g., a connection pool) to execute the query.
    ///
    /// # Returns
    ///
    /// This function returns `Result<Vec<SubTask>, sqlx::error::Error>`.
    ///
    /// - `Ok(sub_tasks)`: A vector of sub-tasks associated with the specified `TaskId`.
    /// - `Err`: If an error occurs during the retrieval process.
    pub async fn get_from_task<'a, E>(
        task_id: TaskId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        Self::get_many("task_id", task_id.0, executor).await
    }
}