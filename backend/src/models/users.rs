use sqlx::{SqlitePool, FromRow};
use uuid::Uuid;

use super::ids::{Base62Id};
use super::user_token::UserToken;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "Base62Id")]
#[serde(into = "Base62Id")]
pub struct UserId(pub u64);

pub const DELETED_USER: UserId = UserId(710324008456789);

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: String,
    pub email: String,
    pub icon_url: String,
    pub login_session: String,
}

pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub icon_url: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}


impl User {
    pub async fn signup(user: UserDTO, conn: &SqlitePool) -> Result<(), sqlx::Error> {
        let passh = user.passhash();
        let id = generate_user_id();

        sqlx::query!(
            r#"INSERT INTO users (
                id, username, password, email, icon_url
            )
            VALUES (
                $1 ,$2 ,$3, $4, $5
            )"#,
            id as UserId, 
            user.username, 
            passh, 
            user.email, 
            "default"
        )
        .execute(conn)
        .await
        .map(|d| d.rows_affected());

        Ok(())
    }

    pub async fn logout(user_id: UserId, conn: &SqlitePool) -> Result<(), sqlx::Error> {
        unimplemented!()
    }

    pub async fn is_valid_login_session(user_token: &UserToken, conn: &SqlitePool) -> bool {
        let query = sqlx::query!(
            "
            SELECT * FROM users
            WHERE username = ? 
            AND login_session = ?
            ",
            user_token.username,
            user_token.login_session,
        );
    
        if let Ok(user) = query.fetch_optional(conn).await {
            user.is_some()
        } else {
            false
        }
    }

    pub async fn find_user_by_username(un: &str, conn: &SqlitePool) -> Result<Option<Self>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT * 
            FROM users 
            WHERE LOWER(username) = LOWER(?)
            ",
            un
        )
        .fetch_optional(conn)
        .await?;

        if let Some(row) = result {
            Ok(Some(User {
                id: UserId(row.id.unwrap()),
                username: row.username.unwrap(),
                password: row.password.unwrap(),
                email: row.email.unwrap(),
                icon_url: row.icon_url.unwrap(),
                login_session: row.login_session.unwrap()
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_login_session_to_db(
        un: &str,
        login_session_str: &str,
        conn: &SqlitePool,
    ) -> bool {

        if let Ok(Some(user)) = User::find_user_by_username(un, conn).await {
            let result = sqlx::query!(
                "UPDATE users SET login_session = ? WHERE id = ?",
                login_session_str,
                user.id
            )
            .execute(conn)
            .await;

            result.is_ok()
        } else {
            false
        }
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}

impl UserDTO {
    pub fn passhash(&self) -> String {
        passhash(&self.username, &self.password)
    }
}

use crate::database::models::user_item::User as DBUser;
impl From<DBUser> for User {
    fn from(data: DBUser) -> Self {
        Self {
            id: data.id.into(),
            username: data.username,
            password: data.password,
            email: data.email,
            icon_url: data.icon_url,
            login_session: data.login_session,
        }
    }
}

fn passhash(name: &str, pass: &str) -> String {
    let namedpass = format!("{}{}", name, pass);
    let hash = bcrypt::hash(namedpass.as_bytes(), bcrypt::DEFAULT_COST).unwrap();
    hash
}

fn passhash_verify(name: &str, pass: &str, hash: &str) -> bool {
    let namedpass = format!("{}{}", name, pass);
    bcrypt::verify(namedpass.as_bytes(), hash).unwrap()
}
