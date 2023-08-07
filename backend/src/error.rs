use actix_web::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("Environment Error")]
    Env(#[from] dotenv::Error),
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid or expired token")]
    InvalidToken,
    #[error("Database error: {0}")]
    SqlxDatabase(#[from] sqlx::Error),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl actix_web::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ResponseError::Env(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::SqlxDatabase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::MissingToken => StatusCode::UNAUTHORIZED,
            ResponseError::InvalidToken => StatusCode::UNAUTHORIZED,
        }
    }
}
