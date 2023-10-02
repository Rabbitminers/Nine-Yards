use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use utoipa::ToSchema;

use crate::ApiContext;
use crate::error::ApiError;
use crate::models::id::UserId;
use crate::models::tokens::Token;
use crate::models::users::{User, Login, Register};
use crate::response::Result;

/// Create a router to be nested on the main api router with
/// endpoints for fetching, registering and authenticating users.
/// 
pub fn configure() -> Router<ApiContext> {
    Router::new()
        .route("/users", get(get_current_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/register", post(register))
        .route("/users/login", post(login))
}

/// Fetches information about the user provided by the given
/// bearer token. Despite the password hash being stored in the
/// user struct it is skipped during serialization for security.
/// 
/// This endpoint requires a bearer token to be provided in the
/// request headers.
/// 
#[utoipa::path(
    get,
    path = "/users",
    context_path = "/api/v1",
    tag = "v1",
    responses(
        (status = 200, description = "Successfully retrieved user", body = User, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 500, description = "Internal server error")
    ),
    security(("Bearer" = [])),
)]
async fn get_current_user(
    State(ctx): State<ApiContext>,
    user_id: UserId,
) -> Result<Json<User>> {
    User::get(user_id, &ctx.pool)
        .await?
        .ok_or(ApiError::Unauthorized)
        .map(|user| Json(user))
}

/// Fetches information about a user given their id. If the user
/// does not exist 403 forbidden will be retured instead of 404
/// not found for security.
/// 
/// This endpoint may require authentication depending on the
/// privicy level set by the user. 
/// 
#[utoipa::path(
    get,
    path = "/users/{id}",
    context_path = "/api/v1",
    tag = "v1",
    params(("id" = String, Path, description = "The user's id", max_length = 8, min_length = 8)),
    responses(
        (status = 200, description = "Successfully retrieved user", body = User, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 403, description = "Forbidden, you don't have permission to access this user"),
        (status = 500, description = "Internal server error")
    ),
    security((), ("Bearer" = [])),
)]
async fn get_user_by_id(
    State(ctx): State<ApiContext>,
    Path(user_id): Path<UserId>,
) -> Result<Json<User>> {
    User::get(user_id, &ctx.pool)
        .await?
        .ok_or(ApiError::NotFound)
        .map(|user| Json(user))
}

#[derive(Serialize, ToSchema)]
pub struct AuthenticatedUser {
    user: User,
    token: Token
}

/// Registers a new user and returns their information aswell as
/// an authorised bearer token to prevent the need to login with 
/// a subsequent request.
/// 
#[utoipa::path(
    post,
    path = "/users/register",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = Register, description = "A register form", content_type = "application/json"),
    responses(
        (status = 200, description = "Successfully registered", body = AuthenticatedUser, content_type = "application/json"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
async fn register(
    State(ctx): State<ApiContext>,
    Json(form): Json<Register>,
) -> Result<Json<AuthenticatedUser>> {
    let mut transaction = ctx.pool.begin().await?;

    let user = User::register(form, &mut transaction).await?;
    transaction.commit().await?;    

    let token = Token::encode(&user);
    
    Ok(Json(AuthenticatedUser { user, token }))
}

/// Logs in a user given their credentials and returns an
/// authorised bearer token which can be used to authenticate
/// 
/// This token should be placed in subsequent request headers
/// like so 
/// 
/// Authorization: Bearer <token>
/// 
#[utoipa::path(
    get,
    path = "/users/login",
    context_path = "/api/v1",
    tag = "v1",
    request_body(content = Login, description = "A login form", content_type = "application/json"),
    responses(
        (status = 200, description = "Successfully retrieved user", body = AuthenticatedUser, content_type = "application/json"),
        (status = 401, description = "Unauthorized, provide a bearer token"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login(
    State(ctx): State<ApiContext>,
    Json(form): Json<Login>,
) -> Result<Json<AuthenticatedUser>> {
    let mut transaction = ctx.pool.begin().await?;

    let user = User::login(form, &mut transaction).await?;
    transaction.commit().await?;
    
    let token = Token::encode(&user);
 
    Ok(Json(AuthenticatedUser { user, token }))
}