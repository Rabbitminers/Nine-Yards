use axum::Router;
use axum::extract::{State, Path};
use axum::routing::get;
use axum::Json;

use crate::ApiContext;
use crate::models::id::SubTaskId;
use crate::models::tasks::{SubTask, EditSubTask};
use crate::models::projects::Permissions;
use crate::middleware::extractors::SubTaskMember;
use crate::response::Result;
use crate::error::ApiError;

/// Create a router to be nested on the main api router with
/// endpoints for sub task item endpoints
///
pub (crate) fn configure() -> Router<ApiContext> {
    Router::new()
        .route("/sub-tasks/:id", 
            get(get_subtask_by_id)
            .put(edit_sub_task)
            .delete(remove_sub_task)
        )
}

/// Fetches the sub-task specified by the id path parameter
///
/// If the project is public then this endpoint requires no
/// authentication, if it is private then a membership of the
/// project is required.
///
#[utoipa::path(
    get,
    path = "/sub-tasks/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the sub task", max_length = 12, min_length = 12)),
    responses(
        (status = 200, description = "Successfully retrieved sub-task", body = SubTask, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to access this sub task"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = [])),
)]
async fn get_subtask_by_id(
    State(ctx): State<ApiContext>,
    Path(id): Path<SubTaskId>,
    SubTaskMember(membership): SubTaskMember 
) -> Result<Json<SubTask>> {
    membership.check_permissions(Permissions::READ_PROJECT)?;

    SubTask::get(id, &ctx.pool)
        .await?
        .ok_or(ApiError::Forbidden)
        .map(|task| Json(task))
}

/// Edits the values of a sub task such as it's body or weight
/// 
/// If the position of the sub task is updated all other subtasks
/// in a position equal to or greater than the updated position
/// are moved forwards to make space for the new location
/// 
/// This endpoint always requires authentication even if the
/// project is public and for the given member to have permission
/// to edit tasks
///
#[utoipa::path(
    put,
    path = "/sub-tasks/{id}",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = EditSubTask, description = "The values to update", content_type = "application/json"),
    params(("id" = String, Path, description = "The id of the sub-task", max_length = 12, min_length = 12)),
    responses(
        (status = 200, description = "Successfully edited the sub task", body = SubTask, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to edit this sub-task"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn edit_sub_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<SubTaskId>,
    SubTaskMember(membership): SubTaskMember,
    Json(form): Json<EditSubTask>
) -> Result<Json<SubTask>> {
    let mut transaction = ctx.pool.begin().await?;

    membership.check_permissions(Permissions::EDIT_TASKS)?;

    SubTask::edit(&id, form, &mut transaction).await?;

    let sub_task = SubTask::get(id, &mut *transaction)
        .await?
        .ok_or(ApiError::Forbidden)?;

    transaction.commit().await?;
    Ok(Json(sub_task))
}

/// Deletes a given sub task aswell as any references to it such
/// as assignments and notifications
/// 
/// This endpoint always requires authentication even if the
/// project is public and for the given member to have permission
/// to remove tasks
/// 
#[utoipa::path(
    delete,
    path = "/sub-tasks/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the sub task to remove", max_length = 12, min_length = 12)),
    responses(
        (status = 200, description = "Successfully removed the sub task"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to remove this sub task"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn remove_sub_task(
    State(ctx): State<ApiContext>,
    Path(id): Path<SubTaskId>,
    SubTaskMember(membership): SubTaskMember 
) -> Result<()> {
    let mut transaction = ctx.pool.begin().await?;
    
    membership.permissions.check_contains(Permissions::EDIT_TASKS)?;

    let subtask = SubTask::get(id, &mut *transaction)
        .await?
        .ok_or(ApiError::Forbidden)?;
    
    subtask.remove(&mut transaction).await?;
    transaction.commit().await?;

    Ok(())
}
