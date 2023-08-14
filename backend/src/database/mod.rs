pub mod sql;

#[cfg(any(feature = "sqlite"))]
pub type SqlPool = sqlx::SqlitePool;
#[cfg(any(feature = "sqlite"))]
pub type PoolOptions = sqlx::sqlite::SqlitePoolOptions;
#[cfg(any(feature = "sqlite"))]
pub type Database = sqlx::Sqlite;
#[cfg(any(feature = "sqlite"))]
pub type TypeInfo = sqlx::sqlite::SqliteTypeInfo;

#[cfg(any(feature = "postgres"))]
pub type SqlPool = sqlx::PgPool;
#[cfg(any(feature = "postgres"))]
pub type PoolOptions = sqlx::postgres::PgPoolOptions;
#[cfg(any(feature = "postgres"))]
pub type Database = sqlx::Postgres;
#[cfg(any(feature = "postgres"))]
pub type TypeInfo = sqlx::postgres::PostgresTypeInfo;