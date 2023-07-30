extern crate sqlx;
#[macro_use]
extern crate serde;

use axum::Router;
use database::SqlPool;

pub mod database;
pub mod models;
pub mod response;
pub mod middleware;
pub mod api;
pub mod services;
pub mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");

    let pool: SqlPool = database::connect().await?;
    
    let app = axum::Router::new()
        .layer(pool);
    
    axum::Server::bind(&"0.0.0.0".parse().unwrap())
        .serve(app)
        .await?;

    Ok(())
}
