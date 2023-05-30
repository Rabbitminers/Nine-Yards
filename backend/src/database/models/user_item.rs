use sqlx::SqlitePool;

use super::ids::UserId;

pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: String,
    pub email: String,
    pub icon_url: String,
    pub login_session: String,
}

impl User {
    pub async fn insert(
        &self, conn: 
        &SqlitePool
    ) -> Result<(), sqlx::error::Error> {
        let user_id = self.id as UserId;

        sqlx::query!(
            "
            INSERT INTO users (
                id, username, password, email, 
                icon_url, login_session
            )
            VALUES (
                $1, $2, $3, $4, $5, $6
            )
            ",
            user_id, self.username, self.password,
            self.email, self.icon_url, self.login_session
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn remove(
        id: UserId,
        conn: &SqlitePool,
    ) -> Result<Option<()>, sqlx::error::Error> {
        let user_id = id as UserId;

        sqlx::query!(
            "
            DELETE FROM users
            WHERE id = $1
            ",
            user_id,
        )
        .execute(conn)
        .await?;

        Ok(Some(()))
    }

    pub async fn find_user_by_username(
        username: &str, 
        conn: &SqlitePool
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let result = sqlx::query!(
            "
            SELECT u.id, u.username, u.password, 
            u.email, u.icon_url, u.login_session
            FROM users u
            WHERE LOWER(username) = LOWER(?)
            ",
            username
        )
        .fetch_optional(conn)
        .await?;

        if let Some(row) = result {
            Ok(Some(User {
                id: UserId(row.id),
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
}

