use axum::extract::{State, Path};
use axum::routing::get;
use axum::{Router, Json};

use crate::models::audits::Audit;
use crate::models::projects::{ProjectBuilder, Project, ProjectMember, Permissions, EditProject};
use crate::models::id::{UserId, ProjectId};
use crate::error::ApiError;
use crate::models::tasks::{TaskGroup, TaskGroupBuilder};
use crate::response::Result;
use crate::ApiContext;

/// Create a router to be nested on the main api router with
/// endpoints for creating, updating and retrieving projects
/// aswell as additional infomration such as memberships and
/// audits
/// 
pub fn configure() -> Router<ApiContext> {
    Router::new()
        .route(
            "/projects", 
            get(get_memberships_from_user)
            .post(create_project)
        )
        .route(
            "/projects/:id", 
            get(get_project_by_id)
            .put(update_project)
            .delete(remove_project)
        )
        .route("/projects/:id/audits", 
            get(get_audits)
        )
        .route("/projects/:id/members", 
            get(get_members)
            .post(invite_member)
        )
        .route("/projects/:id/task-groups",
            get(get_task_groups)
            .post(create_task_group)
        )
}

/// Fetches the projects and related membership of that the logged
/// in user is a member of. Even if the user has no memberships
/// the request will still return a success response with an empty
/// array in the body.
/// 
/// This endpoint requires a bearer token in order to retreive a
/// given user's memberships.
/// 
#[utoipa::path(
    get,
    path = "/projects",
    context_path = "/api/v1",
    tag = "v1",
    responses(
        (status = 200, description = "Successfully retrieved project memberships", body = [ProjectMember], content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = []))
)]
async fn get_memberships_from_user(
    State(ctx): State<ApiContext>,
    user_id: UserId,
) -> Result<Json<Vec<ProjectMember>>> {
    ProjectMember::get_many_from_user(user_id, &ctx.pool)
        .await
        .map_err(|error| error.into())
        .map(|memberships| Json(memberships))
}

/// Fetches a project by it's id.
/// 
/// If the project is public then this endpoint requires no
/// authentication, if it is private then a membership of the
/// project is required.
/// 
#[utoipa::path(
    get,
    path = "/projects/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the project", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully retrieved project", body = Project, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to access this project"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = []))
)]
async fn get_project_by_id(
    State(ctx): State<ApiContext>,
    Path(project_id): Path<ProjectId>,
    _membership: ProjectMember,
) -> Result<Json<Project>> {
    Project::get(project_id, &ctx.pool)
        .await?
        .ok_or(ApiError::Forbidden) // Prevent checking weather a task exists in another project
        .map(|project| Json(project))
}

/// Gets all audits on a project within (by default) the last 7
/// days. By default these are in reverse chronological order so
/// the newest audits are shown first.
/// 
/// If the project is public then this endpoint requires no
/// authentication, if it is private then a membership of the
/// project is required. 
/// 
#[utoipa::path(
    get,
    path = "/projects/{id}/audits",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the project", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully retrieved roject audits", body = [Audit], content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to access this project"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = []))
)]
async fn get_audits(
    State(ctx): State<ApiContext>,
    Path(project_id): Path<ProjectId>,
    _membership: ProjectMember,
) -> Result<Json<Vec<Audit>>> {
    Audit::get_many_from_project(project_id, &ctx.pool)
        .await
        .map(|audits| Json(audits))
        .map_err(|error| error.into())
}

#[utoipa::path(
    post,
    path = "/projects",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = ProjectBuilder, description = "Details of the project to be created ", content_type = "application/json"),
    responses(
        (status = 200, description = "Successfully created a project", body = Project, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = []))
)]
async fn create_project(
    State(ctx): State<ApiContext>,
    user_id: UserId,
    Json(form): Json<ProjectBuilder>,
) -> Result<Json<Project>> {
    let mut transaction = ctx.pool.begin().await?;
    
    let project = Project::create(form, user_id, &mut transaction).await?;
    transaction.commit().await?;

    Ok(Json(project))
}

#[utoipa::path(
    get,
    path = "/projects/{id}/audits",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the project", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully retrieved project audits", body = [Audit], content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to access this project's memberships"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = []))
)]
async fn get_members(
    State(ctx): State<ApiContext>,
    Path(project_id): Path<ProjectId>,
    _membership: ProjectMember,
) -> Result<Json<Vec<ProjectMember>>> {
    ProjectMember::get_many_from_project(project_id, &ctx.pool)
        .await
        .map(|members| Json(members))
        .map_err(|error| error.into())
}

#[utoipa::path(
    post,
    path = "/projects/{id}/members",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = [String], description = "The users id's to invite", content_type = "application/json"),
    params(("id" = String, Path, description = "The id of the project to invite a user to", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully invited member and sent invitation notification"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to invite a member to this project"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = []))
)]
async fn invite_member(
    State(ctx): State<ApiContext>,
    Path(project_id): Path<ProjectId>,
    membership: ProjectMember,
    Json(user_ids): Json<Vec<UserId>>,
) -> Result<()> {
    let mut transaction = ctx.pool.begin().await?;

    if !membership.permissions.contains(Permissions::INVITE_MEMBERS) {
        return Err(ApiError::Forbidden);
    }
    
    ProjectMember::invite_users(user_ids, project_id, &mut transaction).await?;

    transaction.commit().await?;

    Ok(())
}

#[utoipa::path(
    delete,
    path = "projects/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the project", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully removed a project"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to invite a member to this project"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = []))
)]
async fn remove_project(
    State(ctx): State<ApiContext>,
    Path(project_id): Path<ProjectId>,
    membership: ProjectMember
) -> Result<()> {
    let mut transaction = ctx.pool.begin().await?;

    if !membership.permissions.contains(Permissions::DELETE_RPOJECT) {
        return Err(ApiError::Forbidden);
    }

    Project::remove(project_id, &mut transaction).await?;
    transaction.commit().await?;

    Ok(())
}

#[utoipa::path(
    put,
    path = "/projects/{id}",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = EditProject, description = "The values to update", content_type = "application/json"),
    params(("id" = String, Path, description = "The id of the project", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully edited the project", body = Project, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to edit this project"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn update_project(
    State(ctx): State<ApiContext>,
    Path(project_id): Path<ProjectId>,
    membership: ProjectMember,
    Json(form): Json<EditProject>,
) -> Result<Json<Project>> {
    let mut transaction = ctx.pool.begin().await?;

    if !membership.permissions.contains(Permissions::EDIT_PROJECT) {
        return Err(ApiError::Forbidden);
    }

    Project::edit(project_id.clone(), form, &mut transaction).await?;
    transaction.commit().await?;

    let project = Project::get(project_id, &ctx.pool)
        .await?
        .ok_or(ApiError::Forbidden)?;

    Ok(Json(project))
}   

#[utoipa::path(
    get,
    path = "/projects/{id}/task-groups",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The id of the project", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully retrieved project's task groups", body = [TaskGroup], content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to access this project's audits"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = []))
)]
async fn get_task_groups(
    State(ctx): State<ApiContext>,
    Path(project_id): Path<ProjectId>,
    membership: ProjectMember,
) -> Result<Json<Vec<TaskGroup>>> {
    if !membership.permissions.contains(Permissions::READ_PROJECT) {
        return Err(ApiError::Forbidden);
    }

    TaskGroup::get_from_project(project_id, &ctx.pool)
        .await
        .map(|task_groups| Json(task_groups))
        .map_err(|error| error.into())
}

#[utoipa::path(
    post,
    path = "/projects/{id}/task-groups",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = [TaskGroupBuilder], description = "Details of the new task group", content_type = "application/json"),
    params(("id" = String, Path, description = "The id of the project", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully created a project"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to invite a member to this project"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = []))
)]
async fn create_task_group(
    State(ctx): State<ApiContext>,
    Path(project_id): Path<ProjectId>,
    membership: ProjectMember,
    Json(form): Json<TaskGroupBuilder>,
) -> Result<Json<TaskGroup>> {
    let mut transaction = ctx.pool.begin().await?;

    if !membership.permissions.contains(Permissions::CREATE_TASKS) {
        return Err(ApiError::Forbidden);
    }

    let task_group = TaskGroup::create(form, project_id, &mut transaction).await?;
    transaction.commit().await?;

    Ok(Json(task_group))
}