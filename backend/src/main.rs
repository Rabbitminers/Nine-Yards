#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

use actix_web::{web, App};
use database::SqlPool;
use utoipa_swagger_ui::SwaggerUi;

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let sql_pool: SqlPool = database::sql::connect().await?; 

    actix_web::HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::default()
                .allow_any_origin()
                .allow_any_header()
                .allow_any_method()
                .max_age(3600)
                .send_wildcard()
            )
            .wrap(actix_web::middleware::Compress::default())
            .app_data(web::Data::new(sql_pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )   
            .configure(api::v1::config)
    })
    .bind("127.0.0.1:8000")
    .run()
    .await
}
