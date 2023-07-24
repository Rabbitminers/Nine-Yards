extern crate sqlx;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate validator;

use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, OpenApiService};
use poem::listener::TcpListener;
use poem::{Route, EndpointExt};

use database::SqlPool;

pub mod database;
pub mod models;
pub mod error;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path="/", method="get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello world!")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().expect("Failed to read .env file");

    let pool: SqlPool = database::connect().await?;

    let api_service =
        OpenApiService::new(Api, "Todos", "1.0.0")
        .server("http://localhost:8000");

    let ui = api_service.openapi_explorer();
    let route = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .data(pool);

    poem::Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(route)
        .await?;

    Ok(())
}
