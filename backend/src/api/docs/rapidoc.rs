use crate::api::ApiContext; 

use axum::Router;

#[cfg(feature = "rapidoc")]
pub fn configure() -> Router<ApiContext> {
    use utoipa_rapidoc::RapiDoc;

    Router::new()
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}

#[cfg(not(feature = "rapidoc"))]
pub fn configure() -> Router<ApiContext> {
    Router::new()
}