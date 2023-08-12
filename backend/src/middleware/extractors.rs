use std::marker::PhantomData;

use axum::{async_trait, RequestPartsExt, Extension};
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::http::header;
use futures_util::TryFutureExt;

use crate::models::projects::ProjectMember;
use crate::models::id::{UserId, ProjectId, DatabaseId};
use crate::models::tasks::{TaskGroup, Task, SubTask};
use crate::models::tokens::Token;
use crate::models::users::User;
use crate::error::ApiError;
use crate::database::SqlPool;

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = parts.headers
            .get(header::AUTHORIZATION)
            .map(|v| v.to_str().unwrap().to_string())
            .map(|t| Token(t))
            .ok_or(ApiError::Unauthorized)?;

        let claims = token.decode()
            .map_err(|_| ApiError::Unauthorized)?.claims;

        Ok(claims.user_id)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_id = UserId::from_request_parts(parts, state).await?;

        let Extension(pool) = parts.extensions
            .get::<Extension<SqlPool>>()
            .ok_or(ApiError::Internal("Missing application state".to_string()))?;

        let user = User::get(user_id, pool)
            .await?
            .ok_or(ApiError::Unauthorized)?;  

        Ok(user)
    }
}

pub struct Membership<I> 
where
    I: DatabaseId
{
    pub membership: ProjectMember,
    __id_type: PhantomData<I>
}

#[async_trait]
impl<I, S> FromRequestParts<S> for Membership<I>
where
    I: DatabaseId,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_id = UserId::from_request_parts(parts, state).await?;
        
        let Extension(pool) = parts.extensions
            .get::<Extension<SqlPool>>()
            .ok_or(ApiError::Internal("Missing application state".to_string()))?;

        let Path(path_id) = parts.extract::<Path<String>>()
            .await
            .map_err(|_| ApiError::Forbidden)?;

        let table_name = I::table_name();
        
        let sql = format!("SELECT id FROM {} WHERE id = $1", table_name);

        let project_id = sqlx::query_as::<_, ProjectId>(&sql)
            .bind(path_id)
            .fetch_optional(pool)
            .await?
            .ok_or(ApiError::Forbidden)?;

        sqlx::query_as!(    
            ProjectMember,
            "
            SELECT id, project_id, user_id,
            permissions, accepted
            FROM project_members
            WHERE user_id = $1
            AND project_id = $2
            ",
            user_id,
            project_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ApiError::Forbidden)
        .map(|membership| Self { membership, __id_type: PhantomData {} })
    }
} 

impl<I> Membership<I> 
where
    I: DatabaseId
{
    pub fn inner(&self) -> ProjectMember {
        self.membership
    } 
}