use poem::{Endpoint, async_trait, Request, Middleware, Response, IntoResponse};

use crate::{models::{id::ProjectId, tokens::{TokenClaims, Token}, users::User, projects::ProjectMember}, database::SqlPool};

use super::AuthenticationError;


#[derive(Default)]
pub struct ProjectAuthentication {
    table: String,
}

pub struct ProjectAuthenticationEndpoint<E> {
    inner: E,
    table: String,
}

impl ProjectAuthentication {
    #[must_use]
    pub fn new(table: String) -> Self {
        Self { table }
    }
}

impl<E: Endpoint> Middleware<E> for ProjectAuthentication {
    type Output = ProjectAuthenticationEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ProjectAuthenticationEndpoint {
            inner: ep,
            table: self.table
        }
    }
}

#[async_trait]
impl<E: Endpoint> Endpoint for ProjectAuthenticationEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        let pool = req.data::<SqlPool>().unwrap();

        let token = req.header("Authorization")
            .and_then(|h| h.strip_prefix("bearer"))
            .map(|h| Token(h.to_string()))
            .ok_or(AuthenticationError::Unauthorized)?;

        let user_id = TokenClaims::decode(token)
            .map_err(|_| AuthenticationError::Unauthorized)?
            .claims
            .user_id;

        let id = req.raw_path_param("id")
            .ok_or(AuthenticationError::Unauthorized)?;

        let sql = format!(
            "
            SELECT project_id
            FROM {}
            WHERE id = $1
            ",
            self.table
        );

        let project_id = sqlx::query_as::<_, ProjectId>(&sql)
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(poem::error::InternalServerError)?
            .ok_or(AuthenticationError::Forbidden)?;

        let member = ProjectMember::get_from_user(user_id, project_id, pool)
            .await
            .map_err(poem::error::InternalServerError)?
            .ok_or(AuthenticationError::Forbidden);

        req.extensions_mut().insert(member);

        let mut resp = self.inner.call(req).await?.into_response();
        Ok(resp)
    }
}