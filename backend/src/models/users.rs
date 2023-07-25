use bcrypt::DEFAULT_COST;
use serde_derive::{Serialize, Deserialize};
use jsonwebtoken::{Header, EncodingKey};
use sqlx::FromRow;
use chrono::{Utc, NaiveDateTime};

use crate::error::{self, AuthenticationError};
use crate::database::Database;

use super::id::{UserId, LoginSessionId, LoginHistoryEntryId};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

pub const DELETED_USER: &str = "03082007";

#[derive(Serialize, Deserialize, FromRow, sqlx::Decode)]
pub struct User {
    // The user's unqiue id
    pub id: UserId,
    // The user's unique username (3 -> 30 chars)
    pub username: String,
    // The user's hashed password
    #[serde(skip_serializing)]
    pub password: String,
    // The user's email address
    pub email: String,
    // The user's current login session id
    #[serde(skip_serializing)]
    pub login_session: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Register {
    // The user's username
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    // The user's password
    pub password: String,
    // The user's email address
    #[validate(email)]
    pub email: String
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    // Either a username or email for validation
    pub username_or_email: String,
    // The user's password to validated
    pub password: String
}

impl User {
    /// Registers a new user with the provided registration form and inserts it into the database.
    ///
    /// # Arguments
    ///
    /// * `form`: A `Register` struct containing the registration form data, including `username`, `password`, and `email`.
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Self, super::DatabaseError>`, where:
    /// - `Ok(user)` is returned with the newly registered `User` instance if the registration is successful.
    /// - An `Err(super::DatabaseError::AlreadyExists)` is returned if a user with the same `username` already exists in the database.
    /// - An `Err(super::DatabaseError)` is returned if there is an error executing the database queries or generating the user ID.
    ///
    pub async fn register(
        form: Register,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, error::ApiError> {
        if User::get_by_username(form.username.clone(), &mut **transaction).await?.is_some() {
            return Err(super::DatabaseError::AlreadyExists.into());
        }
        
        let password = bcrypt::hash(form.password, DEFAULT_COST).unwrap();
        let id = UserId::generate(&mut *transaction).await?;

        let user: User = User {
            id,
            username: form.username,
            password,
            email: form.email,
            login_session: None
        };

        user.insert(&mut *transaction).await?;

        Ok(user)
    }

    /// Attempts to log in a user with the provided login form and returns an authentication token on success.
    ///
    /// # Arguments
    ///
    /// * `form`: A `Login` struct containing the login form data, including `username_or_email` and `password`.
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Token, error::ApiError>`, where:
    /// - `Ok(token)` is returned with an authentication `Token` if the login is successful.
    /// - An `error::ApiError` is returned if there is an error executing the database queries or if the login credentials are invalid.
    ///
    pub async fn login(
        form: Login,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Token, error::ApiError> {
        let user = sqlx::query_as!(
            User,
            "
            SELECT id, username, password, 
            email, login_session
            FROM users
            WHERE username = $1
            OR email = $1
            ",
            form.username_or_email,
        )
        .fetch_optional(&mut **transaction)
        .await?
        .ok_or(super::DatabaseError::NotFound("User".to_string()))?;

        if user.login_session.is_some() {
            return Err(AuthenticationError::AlreadyLoggedIn.into());
        }

        if !bcrypt::verify(&form.password, &user.password).unwrap() {
            return Err(AuthenticationError::InvalidCredentials.into());
        }   

        let entry = LoginHistoryEntry::create(&user, transaction).await?;

        entry.generate_token(&mut *transaction).await
    }

    /// Logs out the user by setting the `login_session` to `None` in the database.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the logout is successful.
    /// - An `sqlx::error::Error` is returned if there is an error executing the update query.
    ///
    pub async fn logout(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE users
            SET login_session = NULL
            where id = $1
            ",
            self.id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

impl User {
    /// Inserts the current `User` instance into the database using the given transaction.
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
            INSERT INTO users (
                id, username, password,
                email, login_session
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.username,
            self.password,
            self.email,
            self.login_session
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// Retrieves a user record from the database based on the provided `id`.
    ///
    /// # Arguments
    ///
    /// * `id`: A `UserId` representing the unique identifier of the user to retrieve.
    /// * `executor`: An implementation of `sqlx::Executor` representing the database executor.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Option<Self>, sqlx::error::Error>`, where:
    /// - `Ok(Some(user))` is returned if a user with the provided `id` is found in the database.
    /// - `Ok(None)` is returned if no user with the provided `id` is found in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the retrieval query.
    ///
    pub async fn get<'a, E>(
        id: UserId,
        executor: E
    ) -> Result<Option<Self>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        let result = sqlx::query_as!(
            User,
            "
            SELECT id, username, password, 
                   email, login_session
            FROM users
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(result)
    }

    /// Retrieves a list of user records from the database that match the given column and value.
    ///
    /// # Arguments
    ///
    /// * `column`: A `String` representing the column name to use in the WHERE clause of the query.
    /// * `value`: A `String` representing the value to match in the specified column.
    /// * `executor`: An implementation of `sqlx::Executor` representing the database executor.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Vec<Self>, sqlx::error::Error>`, where:
    /// - `Ok(vec)` is returned with a vector containing `User` instances that match the criteria.
    /// - An empty vector is returned if no records match the given column and value.
    /// - An `sqlx::error::Error` is returned if there is an error executing the retrieval query.
    ///
    pub async fn get_many<'a, E>(
        column: &str,
        value: String,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database>,
    {
        let results = sqlx::query_as!(
            User,
            "
            SELECT id, username, password,
                   email, login_session
            FROM users
            WHERE $1 = $2
            ",
            column,
            value
        )
        .fetch_all(executor)
        .await?;

        Ok(results)
    }

    /// Retrieves a user record from the database based on the provided `username`.
    /// This method internally uses the `get_many` method to fetch the user by their username.
    ///
    /// # Arguments
    ///
    /// * `username`: A `String` representing the username to search for in the database.
    /// * `executor`: An implementation of `sqlx::Executor` representing the database executor.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Option<Self>, sqlx::error::Error>`, where:
    /// - `Ok(Some(user))` is returned if a user with the provided `username` is found in the database.
    /// - `Ok(None)` is returned if no user with the provided `username` is found in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the retrieval query.
    ///
    pub async fn get_by_username<'a, E>(
        username: String,
        executor: E
    ) -> Result<Option<Self>, sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database>,
    {
        Self::get_many("username", username, executor)
            .await
            .map(|x| x.into_iter().next())
    }

    /// Removes the user and associated data from the database.
    ///
    /// # Arguments
    ///
    /// * `executor`: An implementation of `sqlx::Executor` representing the database executor.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the user and associated data are successfully removed from the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database queries.
    ///
    pub async fn remove<'a, E>(
        &self,  
        executor: E
    ) -> Result<(), sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database> + Copy,
    {
        let deleted_user: UserId = UserId(DELETED_USER.into());

        sqlx::query!(
            "
            UPDATE project_members
            SET user_id = $1
            WHERE user_id = $2
            ",
            deleted_user.0,
            self.id
        )
        .execute(executor)
        .await?;

        sqlx::query!(
            "
            DELETE FROM notifications
            WHERE recipient = $1
            ",
            self.id
        )
        .execute(executor)
        .await?;

        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE creator = $1
            ",
            self.id
        )
        .execute(executor)
        .await?;

        sqlx::query!(
            "
            UPDATE sub_tasks
            SET assignee = $1
            WHERE assignee = $2
            ",
            deleted_user.0,
            self.id
        )
        .execute(executor)
        .await?;

        sqlx::query!(
            "
            DELETE FROM users
            WHERE id = $1
            ",
            self.id
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct LoginHistoryEntry {
    // The login session's id
    pub id: LoginHistoryEntryId,
    // The session's user's id
    pub user_id: UserId,
    // The timestamp when the user logged in and the sesssion was created
    pub login_timestamp: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token(pub String);

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    // The time the token was issues at (s)
    pub iat: i64,
    // The time the token will expire (issue time + one week)
    pub exp: i64,
    // The user's id
    pub user_id: UserId,
    // The user's login session
    pub login_session: LoginSessionId,
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
    /// This method returns `Result<Self, error::ApiError>`, where:
    /// - `Ok(entry)` is returned with the newly created `LoginHistoryEntry` if the creation and insertion are successful.
    /// - An `error::ApiError` is returned if there is an error executing the database queries or generating the login history entry ID.
    ///
    pub async fn create(
        user: &User,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, error::ApiError> {
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
    /// This method returns `Result<Token, error::ApiError>`, where:
    /// - `Ok(token)` is returned with the newly generated authentication `Token` if successful.
    /// - An `error::ApiError` is returned if there is an error executing the database queries or generating the token.
    ///
    pub async fn generate_token(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Token, error::ApiError> {
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