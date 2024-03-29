use actix_web::{HttpRequest, HttpMessage};
use bcrypt::{DEFAULT_COST, hash, verify};
use uuid::Uuid;
use validator::Validate;
use crate::{models::login_history::LoginHistory, utilities::auth_utils::AuthenticationError, routes::ApiError, database::{Database, SqlPool}, response};

use super::{ids::{UserId, ProjectId}, user_token::UserToken};

pub const DELETED_USER: &str ="03082007";

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub login_session: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Register {
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    pub password: String,
    #[validate(email)]
    pub email: String
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username_or_email: String,
    pub password: String
}


#[derive(Serialize, Deserialize)]
pub struct LoginSession {
    pub user_id: UserId,
    pub login_session: String,
}

impl User {
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
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO users (
                id, username, password, email, 
                login_session
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
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn remove(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
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
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            "
            DELETE FROM notifications
            WHERE recipient = $1
            ",
            self.id
        )
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE creator = $1
            ",
            self.id
        )
        .execute(&mut *transaction)
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
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            "
            DELETE FROM users
            WHERE id = $1
            ",
            self.id
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn is_member_of(
        &self,
        project: ProjectId,
        conn: &SqlPool,
    ) -> Result<bool, ApiError>{
        let query = sqlx::query!(
            "
            SELECT EXISTS(
                SELECT 1
                FROM project_members
                WHERE user_id = $1
                AND project_id = $2
            )
            AS member_exists
            ",
            self.id,
            project.0
        )
        .fetch_one(conn)
        .await?;
    
        Ok(query.member_exists.is_positive())
    }

    pub async fn find_by_login_session<'a, E>(
        token: &UserToken,
        transaction: E,
    ) -> Result<Option<Self>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        let result = sqlx::query!(
            "
            SELECT id, username, password, 
            email, login_session
            FROM users
            WHERE login_session = $1
            ",
            token.login_session
        )
        .fetch_optional(transaction)
        .await?;

        if let Some(row) = result {
            Ok(Some(Self {
                id: UserId(row.id),
                username: row.username,
                password: row.password,
                email: row.email,
                login_session: row.login_session
            }))
        } else {
            Ok(None)
        }
    }
    
    pub async fn find_by_username(
        username: &String, 
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT id, username, password, 
                   email, login_session
            FROM users
            WHERE username = $1
            ",
            username
        )
        .fetch_optional(&mut *transaction)
        .await?;

        if let Some(row) = result {
            Ok(Some(Self {
                id: UserId(row.id),
                username: row.username,
                password: row.password,
                email: row.email,
                login_session: row.login_session
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_id(
        user_id: UserId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT id, username, password, 
                   email, login_session
            FROM users
            WHERE id = $1
            ",
            user_id
        )
        .fetch_optional(&mut *transaction)
        .await?;

        if let Some(row) = result {
            Ok(Some(Self {
                id: UserId(row.id),
                username: row.username,
                password: row.password,
                email: row.email,
                login_session: row.login_session
            }))
        } else {
            Ok(None)
        }
    }
    

    pub async fn from_request(
        req: HttpRequest,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, AuthenticationError> {
        if let Some(user_id) = req.extensions().get::<UserId>() {
            if let Some(user) = Self::find_by_id(user_id.clone(), &mut *transaction).await? {
                return Ok(user);
            }
        } 

        Err(AuthenticationError::InvalidToken)
    }

    pub fn generate_login_session(user_id: UserId) -> LoginSession {
        LoginSession {
            user_id,
            login_session: Uuid::new_v4().to_simple().to_string()
        } 
    }
}

impl Login {
    pub async fn get_user(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Option<User>, sqlx::error::Error> {
        let results = sqlx::query!(
            "
            SELECT id, username, password, 
            email, login_session
            FROM users
            WHERE username = $1
            OR email = $2
            ",
            self.username_or_email,
            self.password
        )
        .fetch_optional(&mut *transaction)
        .await?;

        if let Some(row) = results {
            Ok(Some(User {
                id: UserId(row.id),
                username: row.username,
                password: row.password,
                email: row.email,
                login_session: row.login_session
            }))   
        } else {
            Ok(None)
        }
    }

    pub async fn login(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<LoginSession, ApiError> {
        let user = self
            .get_user(&mut *transaction)
            .await?
            .ok_or(ApiError::NotFound("Could not find user".to_string()))?;

        if user.login_session.is_some() {
            return Err(AuthenticationError::AlreadyLoggedIn.into());
        }

        if !bcrypt::verify(&self.password, &user.password).unwrap() {
            return Err(AuthenticationError::InvalidCredentials.into());
        }   

        let history = LoginHistory::create(&user.username, &mut *transaction)
            .await
            .ok_or_else(|| ApiError::NotFound("Could not find user".to_string()))?;

        sqlx::query!(
            "
            INSERT INTO login_history (
                id, user_id, login_timestamp
            )
            VALUES (
                $1, $2, $3
            )
            ",
            history.id,
            history.user_id,
            history.login_timestamp
        )
        .execute(&mut *transaction)
        .await?;

        let session = User::generate_login_session(user.id);

        sqlx::query!(
            "
            UPDATE users
            SET login_session = $1
            WHERE id = $2
            ",
            session.login_session,
            session.user_id
        )
        .execute(&mut *transaction)
        .await?;
        
        Ok(session)
    }
}

impl Register {
    pub async fn register(
        self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<User, ApiError> {        
        if User::find_by_username(&self.username, &mut *transaction).await?.is_some() {
            return Err(ApiError::InvalidInput(format!("User '{}' already exists", self.username)))
        }

        let hashed_pwd = hash(&self.password, DEFAULT_COST).unwrap();
        let user_id = UserId::generate(&mut *transaction).await?;
        
        let user: User = User {
            id: user_id,
            username: self.username,
            password: hashed_pwd,
            email: self.email,
            login_session: None,
        };

        user.insert(&mut *transaction).await?;
        
        Ok(user)
    }
}
