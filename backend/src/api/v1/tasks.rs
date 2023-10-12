use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::error::ApiError;
use crate::middleware::extractors::TaskMember;
use crate::models::id::TaskId;
use crate::models::projects::Permissions;
use crate::models::tasks::{EditTask, FullTask, SubTask, SubTaskBuilder, Task};
use crate::response::Result;
use crate::api::ApiContext;

/// Create a router to be nested on the main api router with
/// endpoints for task item endpoints and generic sub-task routes
///
pub fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/tasks/:id",
            get(get_task).put(edit_task).delete(remove_task),
        )
        .route(
            "/tasks/:id/sub-tasks",
            get(get_sub_tasks).post(create_sub_task),
        )
}

/// Fetches the task specified by the id path parameter aswell as
/// each of its subtasks and information about the creator of the
/// task aswell as it's assignees in order to reduce the need for
/// subsequent requests.
///
/// If the project is public then this endpoint requires no
/// authentication, if it is private then a membership of the
/// project is required.
///
#[utoipa::path(
    get,
    path = "/tasks/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the task", max_length = 10, min_length = 10)),
    responses(
        (status = 200, description = "Successfully retrieved task and sub-tasks", body = FullTask, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to access this task"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = [])),
)]
async fn get_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskId>,
    TaskMember(membership): TaskMember
) -> Result<Json<FullTask>> {
    membership.check_permissions(Permissions::READ_PROJECT)?;

    Task::get_full(id, &ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(|task| Json(task))
}

/// Edits the values of a task such as it's name or description,
/// fields like the task's id or the parent project's id cannot
/// be changed.
///
/// All fields are optional, except for when the task group is
/// being updated, in which case the position of the task must
/// also be provided
///
/// This endpoint always requires authentication even if the
/// project is public and for the given member to have permission
/// to manage tasks
///
#[utoipa::path(
    put,
    path = "/tasks/{id}",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = EditTask, description = "The values to update", content_type = "application/json"),
    params(("id" = String, Path, description = "The id of the task", max_length = 10, min_length = 10)),
    responses(
        (status = 200, description = "Successfully edited the task", body = Task, content_type = "application/json"),
        (status = 400, description = "Bad request, if the task's group is updated a new position must be given"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to edit this task"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn edit_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskId>,
    TaskMember(membership): TaskMember,
    Json(form): Json<EditTask>,
) -> Result<Json<Task>> {
    let mut transaction = ctx.pool.begin().await?;

    membership.check_permissions(Permissions::EDIT_TASKS)?;

    if form.task_group.is_some() && form.position.is_none() {
        return Err(ApiError::BadRequest);
    }

    Task::edit(id.clone(), form, &mut transaction).await?;

    let task = Task::get(id, &mut *transaction)
        .await?
        .ok_or(ApiError::Forbidden)?; // This should be unreachable

    transaction.commit().await?;

    Ok(Json(task))
}

/// Deletes a given task aswell as any references to it such as
/// sub-tasks, edges and assignments. This will also create an
/// audit entry
///
/// This endpoint always requires authentication even if the
/// project is public and for the given member to have permission
/// to manage tasks
///
#[utoipa::path(
    delete,
    path = "/tasks/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the task", max_length = 10, min_length = 10)),
    responses(
        (status = 200, description = "Successfully removed the task"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to remove this task"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn remove_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskId>,
    TaskMember(membership): TaskMember,
) -> Result<()> {
    let mut transaction = ctx.pool.begin().await?;

    membership.check_permissions(Permissions::DELETE_TASKS)?;

    let task = Task::get(id, &mut *transaction)
        .await?
        .ok_or(ApiError::Forbidden)?;

    task.remove(&mut transaction).await?;
    transaction.commit().await?;

    Ok(())
}

/// Fetches all the sub-tasks on a given task aswell as information
/// about the creators and assignees of the sub-tasks to reduce the
/// need for subsequent requests
///
/// If the project is public then this endpoint requires no
/// authentication, if it is private then a membership of the
/// project is required.
///
#[utoipa::path(
    get,
    path = "/tasks/{id}/sub-tasks",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the parent tasks", max_length = 10, min_length = 10)),
    responses(
        (status = 200, description = "Successfully retrieved task and sub-taks", body = [SubTask], content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to access this task"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = [])),
)]
async fn get_sub_tasks(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskId>,
    TaskMember(membership): TaskMember,
) -> Result<Json<Vec<SubTask>>> {
    membership.check_permissions(Permissions::READ_PROJECT)?;

    SubTask::get_from_task(id, &ctx.pool)
        .await
        .map(|sub_tasks| Json(sub_tasks))
        .map_err(|error| error.into())
}

/// Creates a new sub task on the given task, with default values
/// except for the name which is provided upon creation. Other
/// values such as the task group and project id are extrapolated
/// and the are therefor not required, any optional fields such
/// as assignments will be empty. The position will default to
/// the last available position in the task group. An audit entry
/// will also be created
///
/// This endpoint always requires authentication even if the project 
/// is public and for the given member to have permission to manage 
/// tasks
///
#[utoipa::path(
    post,
    path = "/tasks/{id}/sub-tasks",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = SubTaskBuilder, description = "The data of the new sub task", content_type = "application/json"),
    params(("id" = String, Path, description = "The id of the paretn task", max_length = 10, min_length = 10)),
    responses(
        (status = 200, description = "Successfully created a sub sub task", body = SubTask, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to edit this task"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn create_sub_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskId>,
    TaskMember(membership): TaskMember,
    Json(form): Json<SubTaskBuilder>,
) -> Result<Json<SubTask>> {
    let mut transaction = ctx.pool.begin().await?;

    membership.check_permissions(Permissions::CREATE_TASKS)?;

    let sub_task = SubTask::create(id, membership.project_id, form, &mut transaction).await?;
    transaction.commit().await?;

    Ok(Json(sub_task))
}
