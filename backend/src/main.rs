#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

use std::{time::Duration, net::SocketAddr};

use axum::Router;
use axum::http::{Method, header};
use axum::extract::FromRef;
use database::SqlPool;
use tower_http::add_extension::AddExtensionLayer;
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

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: SqlPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");

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
        .layer(AddExtensionLayer::new(pool))
        .layer(TraceLayer::new_for_http())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
