use thiserror::Error;

pub mod sqlite;

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
}
