use chrono::{NaiveDateTime, Utc};

use crate::database::Database;

use super::{ids::{
    UserId, 
    LoginHistoryId,
}, users::User};

pub struct LoginHistory {
    pub id: LoginHistoryId,
    pub user_id: UserId,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub async fn create(
        username: &String,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Option<LoginHistory> {
        if let Ok(Some(user)) = User::find_by_username(username, &mut *transaction).await {
            let history_id = LoginHistoryId::generate(&mut *transaction).await.ok()?;
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
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }
}

