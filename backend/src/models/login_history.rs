use chrono::{NaiveDateTime, Utc};
use sqlx::SqlitePool;

use super::{ids::{
    UserId, 
    LoginHistoryId, generate_login_history_id
}, users::User};

pub struct LoginHistory {
    pub id: LoginHistoryId,
    pub user_id: UserId,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub async fn create(
        username: &String,
        conn: &SqlitePool
    ) -> Option<LoginHistory> {
        if let Ok(Some(user)) = User::find_by_username(username, conn).await {
            let history_id = match generate_login_history_id(conn).await {
                Ok(history_id) => history_id,
                Err(_) => return None
            };
            let now = Utc::now();
            Some(LoginHistory {
                id: history_id,
                user_id: user.id,
                login_timestamp: now.naive_utc(),
            })
        } else {
            None
        }
    }

    pub async fn save_login_history(
        &self,
        conn: &SqlitePool
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
            self.id.0,
            self.user_id.0,
            self.login_timestamp
        )
        .execute(conn)
        .await?;

        Ok(())
    }
}

