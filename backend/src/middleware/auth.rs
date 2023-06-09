use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web::Data,
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use sqlx::SqlitePool;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::{utilities::token_utils, models::response::ResponseBody, constants};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let token = req.headers()
            .get("Authorization") 
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                h.strip_prefix("Bearer ")
                    .map(|stripped_token | stripped_token.to_owned())
            })
            .unwrap_or_else(String::new);

        let pool = req.app_data::<Data<SqlitePool>>();

        let token_validation_future = async move {
            let pool = pool
                .ok_or(actix_web::error::ErrorInternalServerError(
                    "Missing database pool"
                ))?;
            let token_data = token_utils::decode_token(token)
                .map_err(|_| {
                    HttpResponse::Unauthorized()
                        .json(ResponseBody::new(constants::MESSAGE_INVALID_TOKEN, constants::EMPTY))
                })?;

            if !token.is_empty() && token_utils::verify_token(&token_data, pool).await.is_ok() {
                let res = self.service.call(req).await?;
                Ok(res)
            } else {
                Err(HttpResponse::Unauthorized()
                    .json(ResponseBody::new(
                        constants::MESSAGE_INVALID_TOKEN, 
                        constants::EMPTY
                    ))
                    .into()
                )
            }
        };

        Box::pin(token_validation_future)
    }
}
