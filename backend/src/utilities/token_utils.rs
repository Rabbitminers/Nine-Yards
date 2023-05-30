use crate::models::{
    users::User,
    user_token::{
        UserToken, 
        KEY
    },
};
use actix_web::web;
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use sqlx::SqlitePool;

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub async fn verify_token(
    token_data: &TokenData<UserToken>,
    pool: &SqlitePool,
) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, pool).await {
        Ok(token_data.claims.username.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}
