use axum::extract::{FromRequestParts, Path, FromRef};
use axum::http::header;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt};

use crate::error::ApiError;
use crate::models::id::{ProjectId, UserId};
use crate::models::projects::ProjectMember;
use crate::models::tokens::Token;
use crate::models::users::User;
use crate::ApiContext;

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .map(|v| v.to_str().unwrap().strip_prefix("Bearer "))
            .ok_or(ApiError::Unauthorized)?
            .map(|t| Token(t.to_string()))
            .ok_or(ApiError::Unauthorized)?;

        let claims = token.decode().map_err(|_| ApiError::Unauthorized)?.claims;

        Ok(UserId(claims.user_id))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    ApiContext: FromRef<S>
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_id = UserId::from_request_parts(parts, state).await?;

        let ctx = ApiContext::from_ref(state);

        let user = User::get(user_id, &ctx.pool)
            .await?
            .ok_or(ApiError::Unauthorized)?;

        Ok(user)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ProjectMember
where
    ApiContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user_id = UserId::from_request_parts(parts, state).await?;

        let project_id = parts
            .extract::<Path<String>>()
            .await
            .map(|p| ProjectId(p.0))
            .map_err(|_| ApiError::Forbidden)?;

        let ctx = ApiContext::from_ref(state);

        let project_member = ProjectMember::get_from_user(user_id, project_id, &ctx.pool)
            .await?
            .ok_or(ApiError::Forbidden)?;

        if !project_member.accepted {
            return Err(ApiError::Forbidden);
        }

        Ok(project_member)
    }
}


async fn extract_id(parts: &mut Parts) -> Result<Path<String>, ApiError> {
    parts.extract::<Path<String>>()
        .await
        .map_err(|_| ApiError::Forbidden)
}

macro_rules! impl_from_request_parts {
    ($(($struct:ident, $table_name:literal);)*) => {$(
        pub struct $struct(pub crate::models::projects::ProjectMember);

        impl From<$struct> for crate::models::projects::ProjectMember {
            fn from(value: $struct) -> Self {
                value.0
            }
        }

        #[axum::async_trait]
        impl <S> axum::extract::FromRequestParts<S> for $struct
        where
            S: Send + Sync,
            crate::ApiContext: FromRef<S>
        {
            type Rejection = crate::error::ApiError;

            async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
                let user_id = crate::models::id::UserId::from_request_parts(parts, state).await?;
        
                let Path(path_id) = extract_id(parts).await?;
                
                let ctx = ApiContext::from_ref(state);
                
                let sql = format!("SELECT id FROM {} WHERE id = $1", $table_name);
        
                let project_id = sqlx::query_as::<_, ProjectId>(&sql)
                    .bind(path_id)
                    .fetch_optional(&ctx.pool)
                    .await?
                    .ok_or(ApiError::Forbidden)?;
        
                let member = ProjectMember::get_accepted_from_user(user_id, project_id, &ctx.pool)
                    .await?
                    .ok_or(ApiError::Forbidden)?;

                Ok($struct(member))
            }
        }
    )*};
}

impl_from_request_parts! {
    (TaskGroupMember, "task_groups");
    (TaskMember, "tasks");
    (SubTaskMember, "sub_tasks");
}