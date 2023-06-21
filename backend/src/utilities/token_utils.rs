use crate::{models::{
    user_token::{
        UserToken, 
        KEY
    }, users::User
}, routes::ApiError};
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use sqlx::SqlitePool;

use super::auth_utils::AuthenticationError;

pub fn decode_token(
    token: String
) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub async fn verify_token(
    token_data: &TokenData<UserToken>,
    pool: &SqlitePool,
) -> Result<User, ApiError> {
    let mut transaction = pool.begin().await?;
    if let Ok(Some(user)) = User::find_by_login_session(&token_data.claims, &mut transaction).await {
        transaction.commit().await?;
        Ok(user)
    } else {
        transaction.rollback().await?;
        Err(AuthenticationError::InvalidToken.into())
    }
}

pub async fn is_valid_token(
    token: String,
    pool: &SqlitePool,
) -> Option<User> {
    if let Ok(token_data) = decode_token(token) {
        match verify_token(&token_data, &pool).await {
            Ok(user) => Some(user),
            Err(_) => None
        }
    } else {
        None
    }
}