use std::time::Duration;

use axum::Router;
use axum::http::{Method, header};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::cli::StartCommandArguments;
use crate::database::SqlPool;
use crate::database::sql::connect;

pub mod docs;
pub mod v1;

#[derive(Clone)]
pub struct ApiContext {
    pub pool: SqlPool
}

pub async fn init(
    StartCommandArguments {
        listen_address,
        database_url,
        ..
    }: StartCommandArguments
) -> Result<(), Box<dyn std::error::Error>> {
    let pool: SqlPool = connect(database_url).await?; 

    let app = Router::new()
        .nest("/api/v1", v1::configure())
        .merge(docs::configure())
        .layer(CorsLayer::default()
            .allow_headers([
                header::AUTHORIZATION,
                header::WWW_AUTHENTICATE,
                header::CONTENT_TYPE,
                header::ORIGIN,
                header::COOKIE,
            ])
            .allow_methods([
                Method::GET,
                Method::PUT,
                Method::POST,
                Method::DELETE,
                Method::OPTIONS
            ])
            .allow_origin(Any)
            .max_age(Duration::from_secs(86400))
        )
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .with_state(ApiContext { pool });

    info!("Starting Nine Yards server on http://{}", listen_address);
    #[cfg(feature = "swagger")]
    info!("View the Swagger documentation at http://{}/api-docs/swagger-ui", listen_address);
    #[cfg(feature = "redoc")]
    info!("View the Redoc documentation at http://{}/redoc-ui", listen_address);
    #[cfg(feature = "rapidoc")]
    info!("View the Rapidoc documentation at http://{}/rapidoc-ui", listen_address);

    axum::Server::bind(&listen_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}