use crate::{models::{
    user_token::{
        UserToken, 
        KEY
    }, users::User, ids::ProjectId, projects::ProjectMember
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

pub async fn user_from_token(
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

pub async fn get_project_member(
    token: String, 
    project: ProjectId,
    pool: &SqlPool,
) -> Result<ProjectMember, ApiError> {
    let user = user_from_token(token, &pool).await
        .ok_or(ApiError::Unauthorized(AuthenticationError::Unauthorized))?;

    let member = ProjectMember::get(user.id, project, pool).await?
        .ok_or(ApiError::Unauthorized(AuthenticationError::NotMember))?;

    Ok(member)
}