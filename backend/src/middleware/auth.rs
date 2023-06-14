use std::rc::Rc;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{
    Error, HttpMessage
};
use futures::FutureExt;
use futures::future::{ready, LocalBoxFuture, Ready};
use sqlx::SqlitePool;

use crate::utilities::token_utils;

pub struct Authenticator;

pub struct AuthenticatorMiddleware<S> {
    service: Rc<S>
}

impl<S, B> Transform<S, ServiceRequest> for Authenticator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticatorMiddleware {
            service: Rc::new(service)
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticatorMiddleware<S>
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

        let pool = req.app_data::<Data<SqlitePool>>()
            .unwrap()
            .clone();

        let srv = self.service.clone();

        async move {
            if let Some(user) = token_utils::is_valid_token(token, &pool).await {
                req.extensions_mut().insert(user.id);
            }
            let res = srv.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}