use std::{time::Duration, env};

use super::PoolOptions;

pub async fn connect(database_url: String) -> Result<super::SqlPool, sqlx::Error> {
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
