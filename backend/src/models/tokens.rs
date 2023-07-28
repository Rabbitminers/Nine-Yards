use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{TokenData, DecodingKey, Validation, Header, EncodingKey};
use poem_openapi::NewType;

use crate::database::Database;

use super::{id::{UserId, LoginHistoryEntryId, LoginSessionId}, users::User};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

pub struct LoginHistoryEntry {
    // The login session's id
    pub id: LoginHistoryEntryId,
    // The session's user's id
    pub user_id: UserId,
    // The timestamp when the user logged in and the sesssion was created
    pub login_timestamp: NaiveDateTime
}

#[derive(NewType, Debug, Serialize, Deserialize)]
pub struct Token(pub String);

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    // The time the token was issues at (s)
    pub iat: i64,
    // The time the token will expire (issue time + one week)
    pub exp: i64,
    // The user's id
    pub user_id: UserId,
    // The user's login session
    pub login_session: LoginSessionId,
}

impl TokenClaims {
    /// Decodes the token into the base claims so they can be evaluated
    /// 
    /// # Arguements
    /// 
    /// * `token` The token string to be decoded
    /// 
    /// # Returns
    /// 
    /// This method returns `Result<Self, jsonwebtoken::errors::Error>`, where:
    /// - `Ok(claims)` is returned with the decoded claims if successful.
    /// - `jsonwebtoken::errors::Error` is returned if the token was unable to be decoded, this may indicate an invalid token
    pub fn decode(
        token: Token,
    ) -> jsonwebtoken::errors::Result<TokenData<Self>> {
        jsonwebtoken::decode::<TokenClaims>(
            &token.0,
            &DecodingKey::from_secret(&KEY),
            &Validation::default(),
        )
    }
}

impl LoginHistoryEntry {
    /// Creates a new login history entry for a user and inserts it into the database.
    ///
    /// # Arguments
    ///
    /// * `user`: A reference to the `User` for whom the login history entry is being created.
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Self, sqlx::error::Error>`, where:
    /// - `Ok(entry)` is returned with the newly created `LoginHistoryEntry` if the creation and insertion are successful.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database queries or generating the login history entry ID.
    ///
    pub async fn create(
        user: &User,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, sqlx::error::Error> {
        let id = LoginHistoryEntryId::generate(&mut *transaction).await?;
        let now = Utc::now();

        let history = LoginHistoryEntry {
            id,
            user_id: user.id.clone(),
            login_timestamp: now.naive_local()
        };

        history.insert(&mut *transaction).await?;

        Ok(history)
    }

    /// Generates a bearer token for the login history entry and updates the user's `login_session` in the database.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Token, sqlx::error::Error>`, where:
    /// - `Ok(token)` is returned with the newly generated authentication `Token` if successful.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database queries or generating the token.
    ///
    pub async fn generate_token(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Token, sqlx::error::Error> {
        let session = LoginSessionId::generate(&mut *transaction).await?;

        sqlx::query!(
            "
            UPDATE users
            SET login_session = $1
            WHERE id = $2
            ",
            session,
            self.user_id
        )
        .execute(&mut **transaction)
        .await?;

        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second

        let payload = TokenClaims {
            iat: now,
            exp: now + ONE_WEEK,
            user_id: self.user_id.clone(),
            login_session: session
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )
        .unwrap();

        Ok(Token(token))
    }
}

impl LoginHistoryEntry {
    /// Inserts the login history entry into the database using the provided transaction.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the insertion is successful.
    /// - An `sqlx::error::Error` is returned if there is an error executing the insertion query.
    ///
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO login_history (
                id, user_id, login_timestamp
            )
            VALUES (
                $1, $2, $3
            )
            ",
            self.id,
            self.user_id,
            self.login_timestamp
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

