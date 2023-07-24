
use std::rc::Rc;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{
    Error, HttpMessage
};
use futures::{FutureExt};
use futures::future::{ready, LocalBoxFuture, Ready};
use sqlx::Row;

use crate::database::{SqlPool};
use crate::models::ids::ProjectId;
use crate::models::projects::Project;
use crate::routes::{ApiError};
use crate::utilities::token_utils;

pub struct ProjectAuthentication {
    pub id_key: String,
    pub table_name: Option<String>
}

pub struct ProjectAuthenticationMiddleware<S> {
    service: Rc<S>,
    id_key: String,
    table_name: Option<String>
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
            service: Rc::new(service),
            id_key: self.id_key.clone(),
            table_name: self.table_name.clone()
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
        let token = super::get_token_from_headers(req.headers());

        let pool = req.app_data::<Data<SqlPool>>()
            .unwrap()
            .clone();
        
        let srv = self.service.clone();
        let id_key = self.id_key.clone();
        let table_name = self.table_name.clone();
        
        async move {
            let id = req.match_info().get(&id_key)
                .ok_or(ApiError::InvalidInput("Missing 'task_id' in url".to_string()))?;

            let project_id = self::get_project_id(table_name, id,  &pool).await?;

            let is_public = Project::is_public(project_id.clone(), &**pool)
                .await
                .map_err(|e| ApiError::Database(e))?;
            req.extensions_mut().insert(project_id.clone());

            if !(is_public && req.method() == Method::GET) {
                let member = token_utils::get_project_member(token, project_id, &pool).await?;
                req.extensions_mut().insert(member.id);
            }

            let res = srv.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}

pub async fn get_project_id(
    table_name: Option<String>,
    id: &str,
    pool: &SqlPool
) -> Result<ProjectId, ApiError> {
    if let Some(table_name) = table_name {
        let sql = format!(
            "
            SELECT project_id
            FROM {}
            WHERE id = $1
            ",
            table_name
        );
        
        let query = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(pool)
            .await?;


        if let Some(row) = query {
            let id = row.get(0);
            Ok(ProjectId(id))
        } else {
            Err(ApiError::NotFound("Project not found".to_string()))
        }
    } else {
        Ok(ProjectId(id.to_string()))
    }
}