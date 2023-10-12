use axum::{Router, routing::get};
use utoipa::OpenApi;

use crate::api::ApiContext;
use crate::openapi::ApiDoc;
use crate::response::Result;
use crate::error::ApiError;

pub fn configure() -> Router<ApiContext> {
    Router::new()
        .route("/openapi.json", get(json))
        .route("/openapi.yaml", get(yaml))
}

async fn json() -> Result<String> {
    ApiDoc::openapi()
        .to_json()
        .map_err(|e| ApiError::Anyhow(e.into()))
}

async fn yaml() -> Result<String> {
    ApiDoc::openapi()
        .to_yaml()
        .map_err(|e| ApiError::Anyhow(e.into()))
}