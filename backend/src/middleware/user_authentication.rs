use axum::http::{Request, Response};
use axum::body::Body;
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
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        Box::pin(async {
            let token: Token = request.headers()
                .get("Authorization")
                .map(|v| v.to_str().unwrap().to_string())
                .ok_or(ApiError::Unauthorized)?;

            let claims = token.decode()
                .map_err(|_| ApiError::Unauthorized)?.claims;

            request.extensions().insert(claims.user_id);

            Ok(request)
        })
    }
}