#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

use axum::Router;
use database::SqlPool;
use tower_http::{cors::{CorsLayer, Any}, trace::TraceLayer};

pub mod database;
pub mod models;
pub mod response;
pub mod middleware;
pub mod api;
pub mod services;
pub mod error;

#[derive(utoipa::OpenApi)]
#[openapi(paths())]
pub struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");

    let sql_pool: SqlPool = database::sql::connect().await?; 

    let app = Router::new()
        .layer(CorsLayer::default()
            .allow_origin(Any)
            .allow_headers(Any)
            .allow_methods(Any)
        )
        .layer(TraceLayer::new_for_http())
        .with_state(sql_pool);

    axum::Server::bind(dotenv::var("BIND_ADDR").unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
