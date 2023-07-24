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

#[cfg(any(feature = "sqlite"))]
pub type SqlPool = sqlx::SqlitePool;
#[cfg(any(feature = "sqlite"))]
pub type PoolOptions = sqlx::sqlite::SqlitePoolOptions;
#[cfg(any(feature = "sqlite"))]
pub type Database = sqlx::Sqlite;

#[cfg(any(feature = "postgres"))]
pub type SqlPool = sqlx::PgPool;
#[cfg(any(feature = "postgres"))]
pub type PoolOptions = sqlx::postgres::PgPoolOptions;
#[cfg(any(feature = "postgres"))]
pub type Database = sqlx::Postgres;