use bcrypt::DEFAULT_COST;
use serde_derive::{Serialize, Deserialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::error::ApiError;
use crate::database::Database;

use super::id::UserId;
use super::tokens::Token;

pub const DELETED_USER: &str = "03082007";

#[derive(Serialize, ToSchema, FromRow, sqlx::Decode)]
pub struct User {
    /// The user's unqiue id
    /// 
    #[schema(example="03082007", min_length=8, max_length=8)]
    pub id: UserId,
    /// The user's unique username (3 -> 30 chars)
    /// 
    #[schema(example="My username")]
    pub username: String,
    /// The user's hashed password
    /// 
    #[serde(skip_serializing)]
    #[schema(format=Password)]
    pub password: String,
    /// The user's email address
    /// 
    #[schema(example="user@example.com")]
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Register {
    /// The user's username
    /// 
    #[schema(example="My username")]
    pub username: String,
    /// The user's password
    /// 
    #[schema(example="password", format=Password)]
    pub password: String,
    /// The user's email address
    /// 
    #[schema(example="user@example.com")]
    pub email: String
}

#[derive(Serialize, ToSchema, Deserialize)]
pub struct Login {
    /// Either a username or email for validation
    /// 
    #[schema(example="My username")]
    pub username_or_email: String,
    /// The user's password to validated
    /// 
    #[schema(example="password", format=Password)]
    pub password: String
}

#[derive(Serialize, ToSchema)]
pub struct AuthenticatedUser {
    /// The users data to prevent the need
    /// for a following request to fetch this
    /// 
    pub user: User,
    /// The token generated for the new
    /// session
    /// 
    pub token: Token
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
    ) -> Result<Self, ApiError> {        
        let id = UserId::generate(&mut *transaction).await?;

        let password = bcrypt::hash(form.password, DEFAULT_COST).unwrap();

        let user: User = User {
            id,
            username: form.username,
            password,
            email: form.email,
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
    /// This method returns `Result<Token, sqlx::error::Error>`, where:
    /// - `Ok(token)` is returned with an authentication `Token` if the login is successful.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database queries or if the login credentials are invalid.
    ///
    pub async fn login(
        form: Login,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, ApiError> {
        let user = sqlx::query_as!(
            User,
            "
            SELECT id, username, 
            password, email
            FROM users
            WHERE username = $1
            OR email = $1
            ",
            form.username_or_email,
        )
        .fetch_optional(&mut **transaction)
        .await?
        .ok_or(ApiError::NotFound)?;

        if !bcrypt::verify(&form.password, &user.password).unwrap() {
            return Err(ApiError::Unauthorized);
        }   

        Ok(user)
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
                id, username, 
                password, email
            )
            VALUES (
                $1, $2, $3, $4
            )
            ",
            self.id,
            self.username,
            self.password,
            self.email,
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
            SELECT id, username, 
                password, email
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
            SELECT id, username, 
                password, email
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
            WHERE user_id = $1
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