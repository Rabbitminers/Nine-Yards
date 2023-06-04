use bcrypt::{DEFAULT_COST, hash};
use sqlx::{SqlitePool, error::DatabaseError};
use uuid::Uuid;
use crate::{constants, models::ids::generate_user_id};

use super::ids::UserId;

pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: String,
    pub email: String,
    pub login_session: Option<String>
}

pub struct Register {
    pub username: String,
    pub password: String,
    pub email: String
}

impl User {
    pub async fn register(
        data: Register, 
        conn: &SqlitePool
    ) -> Result<String, String> {
        match Self::find_by_username(&data.username, conn).await {
            Ok(None) => {
                let hashed_pwd = hash(&data.password, DEFAULT_COST).unwrap();
                let user_id = generate_user_id(conn).await;
                
                if let Ok(ido) = user_id {
                    User {
                        id: ido,
                        username: data.username,
                        password: hashed_pwd,
                        email: data.email,
                        login_session: None
                    }
                    .insert(conn);

                    Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
                } else {
                    Err(constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string())
                }
            }
            _ => Err(format!("User '{}' is already registered", &data.username))
        }
    }

    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO users (
                id, username, password, email, login_session
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

    pub async fn find_by_username(
        username: &String, 
        conn: &SqlitePool
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT id, username, password, email, login_session
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