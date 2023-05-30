use actix_web::http::HeaderMap;
use sqlx::SqlitePool;
use thiserror::Error;

use crate::models::users::User
use crate::database::models;
use crate::utilities::token_utils;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("An unknown database error occurred")]
    Sqlx(#[from] sqlx::Error),
    #[error("Database Error: {0}")]
    Database(#[from] models::DatabaseError),
    #[error("Error while parsing JSON: {0}")]
    SerDe(#[from] serde_json::Error),
    #[error("Error while communicating to GitHub OAuth2: {0}")]
    InvalidCredentials,
}

pub async fn get_user_from_headers(
    headers: &HeaderMap,
    conn: &SqlitePool
) -> Result<User, AuthenticationError> {
    let token = headers
        .get("Authorization")
        .ok_or(AuthenticationError::InvalidCredentials)?
        .to_str()
        .map_err(|_| AuthenticationError::InvalidCredentials)?;

    get_user_from_token(token, conn).await
}

pub async fn get_user_from_token(
    access_token: &str,
    conn: &SqlitePool
) -> Result<User, AuthenticationError> {
    if let Ok(token_data) = token_utils::decode_token(access_token.to_string()) {
        if let Ok(username) = token_utils::verify_token(&token_data, pool) {

        }
    }
}