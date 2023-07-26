use poem::{Endpoint, async_trait, Request, Middleware, Response, IntoResponse};

use crate::models::tokens::{TokenClaims, Token};
use crate::models::users::User;
use crate::database::SqlPool;

use super::AuthenticationError;

#[derive(Default)]
pub struct UserAuthentication;

pub struct UserAuthenticationEndpoint<E> {
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
            .and_then(|h| h.strip_prefix("Bearer"))
            .map(|h| Token(h.to_string()))
            .ok_or(AuthenticationError::MissingToken)?;

        let pool = req.data::<SqlPool>().unwrap();

        let claims = TokenClaims::decode(token)
            .map_err(|e| AuthenticationError::InvalidToken(e))?;

        let user = User::from_token(&claims, pool)
            .await
            .map_err(|e| AuthenticationError::InternalServerError(e))?;

        req.extensions_mut().insert(user);

        let mut resp = self.inner.call(req).await?.into_response();
        Ok(resp)
    }
}