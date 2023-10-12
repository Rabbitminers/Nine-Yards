use crate::api::ApiContext; 

use axum::Router;

#[cfg(feature = "swagger")]
pub fn configure() -> Router<ApiContext> {
    use utoipa_swagger_ui::SwaggerUi;

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui"))
}

#[cfg(not(feature = "swagger"))]
pub fn configure() -> Router<ApiContext> {
    Router::new()
}