use axum::body::BoxBody;
use axum::http::{Request, Response, header};
use axum::response::IntoResponse;
use futures_util::future::BoxFuture;
use tower_http::auth::AsyncAuthorizeRequest;

use crate::error::ApiError;
use crate::models::tokens::Token;

#[derive(Clone, Copy)]
struct UserAuthenticationLayer;

impl <B> AsyncAuthorizeRequest<B> for UserAuthenticationLayer
where   
    B: Send + Sync + 'static
{
	type RequestBody = B;
	type ResponseBody = BoxBody;
	type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        Box::pin(async {
            let (mut parts, _body) = request.into_parts();

            let token = parts.headers
                .get(header::AUTHORIZATION)
                .map(|v| v.to_str().unwrap().to_string())
                .map(|t| Token(t))
                .ok_or(ApiError::Unauthorized.into_response())?;

            let claims = token.decode()
                .map_err(|_| ApiError::Unauthorized.into_response())?.claims;

            parts.extensions.insert(claims.user_id);

            Ok(Request::from_parts(parts, _body))
        })
    }
}