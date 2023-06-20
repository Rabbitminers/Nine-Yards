extern crate sqlx;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate validator;

use std::env;

use actix_cors::Cors;
use actix_web::{ 
    App, 
    HttpServer, 
    http, web
};
use database::sqlite;

pub mod config;
pub mod routes;
pub mod models;
pub mod constants;
pub mod middleware;
pub mod utilities;
pub mod database;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    let pool = sqlite::connect()
        .await
        .expect("Database connection failed");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() 
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            // .wrap(crate::middleware::auth::Authenticator)
            .configure(config::app::config_services)
            .configure(routes::config)
    })
    .bind(&app_url)?
    .run()
    .await
}
