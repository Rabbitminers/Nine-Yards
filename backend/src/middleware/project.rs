use std::rc::Rc;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{
    Error, HttpMessage
};
use futures::FutureExt;
use futures::future::{ready, LocalBoxFuture, Ready};

use crate::database::SqlPool;
use crate::models::ids::ProjectId;
use crate::models::users::User;
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
            let user = get_authenticated_user(token, &pool).await?;

            let is_public = is_project_public(project_id.clone(), &pool).await?;

            if user.is_member_of(project_id.clone(), &pool).await?
                    || (is_public && req.method() == Method::GET) {
                req.extensions_mut().insert(user.id);
                let res = srv.call(req).await?;
                Ok(res)
            } else {
                Err(ApiError::Unauthorized(NotMember).into())
            }
        }
        .boxed_local()
    }
}

pub async fn is_project_public(
    project_id: ProjectId,
    conn: &SqlPool,
) -> Result<bool, ApiError> {
    let query = sqlx::query!(
        "
        SELECT public
        FROM projects
        WHERE id = $1
        ",
        project_id
    )
    .fetch_one(conn)
    .await?;

    Ok(query.public)
}

pub async fn get_authenticated_user(
    token: String, 
    pool: &SqlPool,
) -> Result<User, ApiError> {
    if let Some(user) = token_utils::is_valid_token(token, &pool).await {
        Ok(user)
    } else {
        Err(ApiError::Unauthorized(Unauthorized))
    }
}

pub fn get_project_id(
    req: &ServiceRequest,
) -> Result<ProjectId, ApiError> {
    req.match_info()
        .get("id")
        .map(|id| ProjectId(id.to_owned()))
        .ok_or_else(|| ApiError::InvalidInput("Missing project id".to_string()))
}