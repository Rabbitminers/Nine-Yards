use axum::Router;

use super::ApiContext;

pub mod schema;
pub mod redoc;
pub mod swagger;
pub mod rapidoc;

pub fn configure() -> Router<ApiContext> {
    Router::new()
        .nest("/api-docs", schema::configure())
        .merge(swagger::configure())
        .merge(redoc::configure())
        .merge(rapidoc::configure())
}   

