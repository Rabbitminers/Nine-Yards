use crate::{models::{
    user_token::{
        UserToken, 
        KEY
    }, users::User
}, routes::ApiError};
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use crate::database::SqlPool;

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
    pool: &SqlPool,
) -> Result<User, ApiError> {
    if let Ok(Some(user)) = User::find_by_login_session(&token_data.claims, pool).await {
        Ok(user)
    } else {
        Err(AuthenticationError::InvalidToken.into())
    }
}

pub async fn is_valid_token(
    token: String,
    pool: &SqlPool,
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