use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::SqlitePool;
use uuid::Uuid;
use validator::Validate;
use crate::{constants, models::{ids::generate_user_id, login_history::LoginHistory}};

use super::{ids::UserId, user_token::UserToken};

pub const DELETED_USER: &str ="7102222d-b551-46e6-b1cf-44a67a05aace";

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

pub struct Login {
    pub username: String,
    pub password: String
}

pub struct LoginSession {
    pub username: String,
    pub login_session: String,
}

impl User {
    pub async fn register(
        data: Register, 
        conn: &SqlitePool
    ) -> Result<User, String> {        
        // Check if the username is already used
        if let Ok(Some(_)) = Self::find_by_username(&data.username, conn).await {
            return Err(format!("User '{}' is already registered", &data.username));
        }

        // Generate password hash and new user id
        let hashed_pwd = hash(&data.password, DEFAULT_COST).unwrap();
        let user_id = generate_user_id(conn).await;
        
        let user_id = match user_id {
            Ok(generated_id) => generated_id,
            Err(_) => return Err(constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()),
        };
        
        let user: User = User {
            id: user_id,
            username: data.username,
            password: hashed_pwd,
            email: data.email,
            login_session: None,
        };

        user.insert(conn).await;
        
        Ok(user)
    }

    pub async fn logout(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE users
            SET login_session = $1
            where id = $2
            ",
            "",
            self.id.0
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn login(
        login: Login, 
        conn: &SqlitePool
    ) -> Result<Option<LoginSession>, sqlx::error::Error> {
        let user = match login.get_user(conn).await? {
            Some(user) => user,
            None => return Ok(None)
        };

        if !verify(&login.password, &user.password).unwrap()
                || user.password.is_empty()  {
            return Ok(None);
        }   

        if let Some(history) = LoginHistory::create(&user.username, conn).await { 
            sqlx::query!(
                "
                INSERT INTO login_history (
                    id, user_id, login_timestamp
                )
                VALUES (
                    $1, $2, $3
                )
                ",
                history.id.0,
                history.user_id.0,
                history.login_timestamp
            )
            .execute(conn)
            .await?;
        } else {
            return Ok(None)
        };
        
        let session = Self::generate_login_session();

        Ok(Some(LoginSession {
            username: user.username,
            login_session: session
        }))
    }

    pub async fn insert(
        &self,
        conn: &SqlitePool
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
            self.id.0,
            self.username,
            self.password,
            self.email,
            self.login_session
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn remove(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        let deleted_user: UserId = UserId(DELETED_USER.into());

        sqlx::query!(
            "
            UPDATE team_members
            SET user_id = $1
            WHERE user_id = $2
            ",
            deleted_user.0,
            self.id.0
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            DELETE FROM notifications
            WHERE user_id = $1
            ",
            self.id.0
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE creator = $1
            ",
            self.id.0
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            UPDATE tasks
            SET assignee = $1
            WHERE assignee = $2
            ",
            deleted_user.0,
            self.id.0
        )
        .execute(conn)
        .await?;

        sqlx::query!(
            "
            DELETE FROM users
            WHERE id = $1
            ",
            self.id.0
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn get_user_from_login_session(
        token: &UserToken,
        conn: &SqlitePool
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT id, username, password, 
            email, login_session
            FROM users
            WHERE login_session = $1
            ",
            token.login_session
        )
        .fetch_optional(conn)
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
        conn: &SqlitePool
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
        .fetch_optional(conn)
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

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}

impl Login {
    pub async fn get_user(
        &self,
        conn: &SqlitePool
    ) -> Result<Option<User>, sqlx::error::Error> {
        let results = sqlx::query!(
            "
            SELECT id, username, password, 
            email, login_session
            FROM users
            WHERE username = $1
            OR email = $2
            ",
            self.username,
            self.password
        )
        .fetch_optional(conn)
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

    pub async fn is_valid_user(
        &self,
        conn: &SqlitePool
    ) -> bool {
        let result = self.get_user(conn).await;

        if let Ok(Some(_)) = result {
            true
        } else {
            false
        }
    }
}