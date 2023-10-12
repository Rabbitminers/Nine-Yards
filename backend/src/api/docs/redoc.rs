use crate::api::ApiContext; 

use axum::Router;

#[cfg(feature = "redoc")]
pub fn configure() -> Router<ApiContext> {
    use crate::openapi::ApiDoc;

    use utoipa_redoc::{Redoc, Servable};
    use utoipa::OpenApi;

    Router::new()
        .merge(Redoc::with_url("/redoc-ui", ApiDoc::openapi()))
}

#[cfg(not(feature = "redoc"))]
pub fn configure() -> Router<ApiContext> {
    Router::new()
}