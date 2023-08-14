use axum::Router;

use crate::ApiContext;

pub mod tasks;
pub mod users;
pub mod projects;

pub fn configure() -> Router<ApiContext> {
    Router::new()
        .merge(tasks::configure())
        .merge(users::configure())
}

