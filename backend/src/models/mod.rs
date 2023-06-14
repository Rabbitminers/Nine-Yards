use thiserror::Error;

pub mod users;
pub mod user_token;
pub mod projects;
pub mod ids;
pub mod tasks;
pub mod login_history;
pub mod error;
pub mod notifications;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Error while interacting with the database: {0}")]
    Database(#[from] sqlx::error::Error),

    #[error("Error while trying to generate random ID")]
    RandomId,

    #[error("A database request failed")]
    Other(String),

    #[error("Error while parsing JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Entry already exists")]
    AlreadyExists
}
