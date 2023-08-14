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
use utoipa_swagger_ui::SwaggerUi;

pub mod database;
pub mod models;
pub mod response;
pub mod middleware;
pub mod api;
pub mod error;

#[derive(OpenApi)]
#[openapi(paths())]
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
