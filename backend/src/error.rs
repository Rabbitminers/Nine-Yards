use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] crate::models::DatabaseError),

    #[error("Database error: {0}")]
    SqlxDatabase(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Json model error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid Input: {0}")]
    InvalidInput(String),

    #[error("Error while validating input: {0}")]
    Validation(String),

    #[error("Could not find {0}")]
    NotFound(String),

    #[error("Authentication Error: {0}")]
    Unauthorized(#[from] self::AuthenticationError),
}

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("An unknown database error occurred")]
    Sqlx(#[from] sqlx::Error),

    #[error("Database error: {0}")]
    Database(#[from] crate::models::DatabaseError),

    #[error("Error while parsing JSON: {0}")]
    SerDe(#[from] serde_json::Error),

    #[error("Invalid Authentication Credentials")]
    InvalidCredentials,

    #[error("You are not authorized to perform this action")]
    Unauthorized,

    #[error("You do not have sufficient permissions to perform this action")]
    MissingPermissions,

    #[error("Invalid or missing token. Please login again")]
    InvalidToken,

    #[error("You need to be member of this project in order to perform this action")]
    NotMember,

    #[error("You are already logged in")]
    AlreadyLoggedIn
}