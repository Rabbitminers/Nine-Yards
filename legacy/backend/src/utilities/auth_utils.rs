use thiserror::Error;

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