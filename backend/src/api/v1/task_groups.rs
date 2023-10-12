use axum::routing::get;
use axum::{Json, Router};
use axum::extract::{State, Path};

use crate::error::ApiError;
use crate::middleware::extractors::TaskGroupMember;
use crate::models::id::TaskGroupId;
use crate::api::ApiContext;
use crate::models::projects::Permissions;
use crate::models::tasks::{TaskGroup, EditTaskGroup, Task, FullTask};
use crate::response::Result;

pub (crate) fn configure() -> Router<ApiContext> {
   Router::new() 
        .route("/task-groups/:id", 
            get(get_task_group_by_id)
            .put(edit_task_group)
            .delete(remove_task_group)
        )
        .route("/task-groups/:id/tasks", get(get_tasks))
}

#[utoipa::path(
    get,
    path = "/task-groups/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the task group", max_length = 10, min_length = 10)),
    responses(
        (status = 200, description = "Successfully fetched task group", body = TaskGroup, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to read from this project"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = [])), // Optional bearer token
)]
async fn get_task_group_by_id(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskGroupId>,
    TaskGroupMember(membership): TaskGroupMember,
) -> Result<Json<TaskGroup>> {
    membership.check_permissions(Permissions::READ_PROJECT)?;

    TaskGroup::get(id, &ctx.pool)
        .await?
        .ok_or(ApiError::Forbidden)
        .map(|task_group| Json(task_group))
}

#[utoipa::path(
    put,
    path = "/task-groups/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the task group", max_length = 10, min_length = 10)),
    request_body(content = EditTaskGroup, description = "The values to update", content_type = "application/json"),
    responses(
        (status = 200, description = "Successfully edited the task group", body = TaskGroup, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to edit this task group"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn edit_task_group(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskGroupId>,
    TaskGroupMember(membership): TaskGroupMember,
    Json(form): Json<EditTaskGroup>,
) -> Result<Json<TaskGroup>> {
    let mut transaction = ctx.pool.begin().await?;

    membership.check_permissions(Permissions::CREATE_TASK_GROUPS)?;

    TaskGroup::edit(id.clone(), form, &mut transaction).await?;

    let task_group = TaskGroup::get(id, &mut *transaction)
        .await?
        .ok_or(ApiError::Forbidden)?; // Should be impossible to reach
    
    transaction.commit().await?;

    Ok(Json(task_group))
}

#[utoipa::path(
    delete,
    path = "/task-groups/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the task group to remove", max_length = 10, min_length = 10)),
    responses(
        (status = 200, description = "Successfully removed the task group", body = TaskGroup, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to remove this task group"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn remove_task_group(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskGroupId>,
    TaskGroupMember(membership): TaskGroupMember,
) -> Result<()> {
    let mut transaction = ctx.pool.begin().await?;

    membership.check_permissions(Permissions::DELETE_TASK_GROUPS)?;

    let task_group = TaskGroup::get(id, &mut *transaction)
        .await?
        .ok_or(ApiError::Forbidden)?;
    task_group.remove(&mut transaction).await?;

    transaction.commit().await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/task-groups/{id}/tasks",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the task group to fetch the tasks from", max_length = 10, min_length = 10)),
    responses(
        (status = 200, description = "Successfully fetched task group", body = [TaskGroup], content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to read from this project"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = [])), // Optional bearer token
)]
async fn get_tasks(
    State(ctx): State<ApiContext>,
    Path(id): Path<TaskGroupId>,
    TaskGroupMember(membership): TaskGroupMember,
 ) -> Result<Json<Vec<FullTask>>> {
    membership.check_permissions(Permissions::READ_PROJECT)?;

    Task::get_many_from_task_group(id, &ctx.pool)
        .await
        .map(|tasks| Json(tasks))
        .map_err(|error| error.into())
}