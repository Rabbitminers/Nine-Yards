#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

use std::time::Duration;

use axum::Router;
use axum::http::{Method, header};
use database::SqlPool;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa::openapi::info;
use utoipa_swagger_ui::SwaggerUi;

pub mod database;
pub mod models;
pub mod response;
pub mod middleware;
pub mod api;
pub mod error;
pub mod utilities;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Nine Yards REST API",
        version = "0.0.1",
    ),
    paths(
        api::v1::projects::create_project,
        api::v1::projects::get_memberships_from_user,
        api::v1::projects::get_project_by_id,
        api::v1::projects::update_project,
        api::v1::projects::remove_project,
        api::v1::projects::get_audits,
        api::v1::projects::get_members,
        api::v1::projects::invite_member,
        api::v1::projects::get_task_groups,
        api::v1::projects::create_task_group
    ),
    components(schemas(
        models::id::UserId,
        models::id::ProjectId,
        models::id::ProjectMemberId,
        models::id::TaskGroupId,
        models::id::TaskId,
        models::id::SubTaskId,
        models::id::AuditId,
        models::id::NotificationId,
        models::id::NotificationActionId,
        models::projects::Project,
        models::projects::EditProject,
        models::projects::ProjectBuilder,
        models::projects::Permissions,
        models::projects::ProjectMember,
    ))
)]
pub struct ApiDoc;

#[derive(Clone)]
pub struct ApiContext {
    pool: SqlPool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");

    tracing_subscriber::fmt::init();

    let pool: SqlPool = database::sql::connect().await?; 

    let app = Router::new()
        .layer(CorsLayer::default()
            .allow_headers([
                header::AUTHORIZATION,
                header::WWW_AUTHENTICATE,
                header::CONTENT_TYPE,
                header::ORIGIN
            ])
            .allow_methods([
                Method::GET,
                Method::PUT,
                Method::POST,
                Method::DELETE,
            ])
            .allow_origin(Any)
            .max_age(Duration::from_secs(86400))
        )
        .layer(TraceLayer::new_for_http())
        .nest("/api/v1", api::v1::configure())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(ApiContext { pool });

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
