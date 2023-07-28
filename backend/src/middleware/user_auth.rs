use poem::{Endpoint, async_trait, Request, Middleware, Response, IntoResponse};

use crate::{models::{tokens::{Token, TokenClaims}, users::User}, database::SqlPool};

use super::AuthenticationError;

#[derive(Default)]
pub struct UserAuthentication;

pub struct UserAuthenticationEndpoint<E: Endpoint> {
    inner: E,
}

impl UserAuthentication {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
}

impl<E: Endpoint> Middleware<E> for UserAuthentication {
    type Output = UserAuthenticationEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        UserAuthenticationEndpoint {
            inner: ep,
        }
    }
}

#[async_trait]
impl<E: Endpoint> Endpoint for UserAuthenticationEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        let token = req.header("Authorization")
            .and_then(|h| h.strip_prefix("bearer"))
            .map(|h| Token(h.to_string()))
            .ok_or(AuthenticationError::Unauthorized)?;

        let token_data = TokenClaims::decode(token)
            .map_err(|_| AuthenticationError::Unauthorized)?;

        req.extensions().insert(token_data.claims.user_id);

        let mut resp = self.inner.call(req).await?.into_response();
        Ok(resp)
    }
}