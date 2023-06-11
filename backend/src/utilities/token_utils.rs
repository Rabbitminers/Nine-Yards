use crate::models::{
    user_token::{
        UserToken, 
        KEY
    }, users::User
};
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use sqlx::SqlitePool;

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
) -> Result<User, String> {
    if let Ok(Some(user)) = User::find_by_login_session(&token_data.claims, &pool).await {
        Ok(user)
    } else {
        Err("Invalid token".to_string())
    }
}

pub async fn is_valid_token(
    token: String,
    pool: &SqlitePool,
) -> Option<User> {
    if let Ok(token_data) = decode_token(token) {
        verify_token(&token_data, &pool).await.ok()
    } else {
        None
    }
}