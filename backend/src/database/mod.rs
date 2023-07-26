use std::{env, time::Duration};

pub async fn connect() -> Result<super::SqlPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL not found.");
        
    let pool = PoolOptions::new()
        .min_connections(
            env::var("DATABASE_MIN_CONNECTIONS")
                .ok()
                .and_then( |x| x.parse().ok() )
                .unwrap_or(0)
        )
        .max_connections(
            env::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then( |x| x.parse().ok() )
                .unwrap_or(16)
        )
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .connect(&database_url)
        .await?;
    
    Ok(pool)
}

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