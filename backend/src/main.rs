#![forbid(unsafe_code)]

extern crate sqlx;
#[macro_use]
extern crate serde;

use std::process::ExitCode;

pub mod database;
pub mod models;
pub mod response;
pub mod middleware;
pub mod api;
pub mod error;
pub mod utilities;
pub mod cli;
pub mod openapi;

#[tokio::main]
async fn main() -> ExitCode {
    dotenv::dotenv().expect("Failed to read .env file");

    tracing_subscriber::fmt::init();

    cli::init().await
}
