use sqlx::{
    sqlite::SqlitePoolOptions, 
    SqlitePool
};
use std::{
    time::Duration,
    env
};
use log::info;


pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    info!("Initializing database connection");
    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL not found.");

    let pool = SqlitePoolOptions::new()
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