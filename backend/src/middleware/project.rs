use std::rc::Rc;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{
    Error, HttpMessage
};
use futures::FutureExt;
use futures::future::{ready, LocalBoxFuture, Ready};

use crate::database::{SqlPool};
use crate::models::DatabaseError;
use crate::models::ids::{ProjectId, ProjectMemberId, UserId};
use crate::models::projects::{ProjectMember, Permissions, Project};
use crate::routes::ApiError;
use crate::utilities::auth_utils::AuthenticationError::{Unauthorized, NotMember};
use crate::utilities::token_utils;

pub struct ProjectAuthentication;

pub struct ProjectAuthenticationMiddleware<S> {
    service: Rc<S>
}

impl<S, B> Transform<S, ServiceRequest> for ProjectAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ProjectAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ProjectAuthenticationMiddleware {
            service: Rc::new(service)
        }))
    }
}

impl<S, B> Service<ServiceRequest> for ProjectAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers()
            .get("Authorization") 
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                h.strip_prefix("Bearer ")
                    .map(|stripped_token | stripped_token.to_owned())
            })
            .unwrap_or_else(String::new);

        let pool = req.app_data::<Data<SqlPool>>()
            .unwrap()
            .clone();
        
        let srv = self.service.clone();

        async move {
            let project_id = get_project_id(&req)?;
            let is_public = Project::is_public(project_id.clone(), &**pool)
                .await
                .map_err(|e| ApiError::Database(e))?;

            if !(is_public && req.method() == Method::GET) {
                let member = get_project_member(token, project_id, &pool).await?;
                req.extensions_mut().insert(member.id);
            }

            let res = srv.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}

pub async fn get_project_member(
    token: String, 
    project: ProjectId,
    pool: &SqlPool,
) -> Result<ProjectMember, ApiError> {
    let user = token_utils::is_valid_token(token, &pool).await
        .ok_or(ApiError::Unauthorized(Unauthorized))?;

    let query = sqlx::query!(
        "
        SELECT id, project_id,
            user_id, permissions, 
            accepted
        FROM project_members
        WHERE user_id = $1
        AND project_id = $2
        ",
        user.id,
        project
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some(row) = query {
        Ok(ProjectMember {
            id: ProjectMemberId(row.id),
            project_id: ProjectId(row.project_id),
            user_id: UserId(row.user_id),
            permissions: Permissions::from_bits(row.permissions as u64).unwrap_or_default(),
            accepted: row.accepted,                
        })
    } else {
        Err(ApiError::Unauthorized(NotMember))
    }
}

pub fn get_project_id(
    req: &ServiceRequest,
) -> Result<ProjectId, ApiError> {
    req.match_info()
        .get("project_id")
        .map(|id| ProjectId(id.to_owned()))
        .ok_or_else(|| ApiError::InvalidInput("Missing project id".to_string()))
}