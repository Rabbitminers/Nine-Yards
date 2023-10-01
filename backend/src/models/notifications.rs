use chrono::{NaiveDateTime, Utc};

use crate::database::Database;

use super::id::{NotificationId, NotificationActionId, UserId};

#[derive(Serialize)]
pub struct Notification {
    pub id: NotificationId,
    pub user_id: UserId,
    pub body: String,
    pub created: NaiveDateTime,
    pub read: bool
}

pub struct NotificationBuilder {
    pub body: String,
    pub actions: Vec<NotificationActionBuilder>
}

#[derive(Deserialize, Serialize)]
pub struct NotificationAction {
    pub id: NotificationActionId,
    pub notification_id: NotificationId,
    pub title: String,
    pub action_endpoint: String
}

pub struct NotificationActionBuilder {
    pub title: String,
    pub action_endpoint: String
}

#[derive(Serialize)]
pub struct FullNotification {
    pub id: NotificationId,
    pub user_id: UserId,
    pub body: String,
    pub created: NaiveDateTime,
    pub read: bool,
    pub actions: Actions
}

#[derive(Deserialize, Serialize)]
pub struct Actions {
    inner: Vec<NotificationAction>
}

impl From<String> for Actions {
    fn from(value: String) -> Self {
        let actions: Vec<NotificationAction> = serde_json::from_str(&value).unwrap_or_default();
        Self { inner: actions }
    }
}

impl Notification {
    pub async fn send(
        builder: NotificationBuilder,
        user_id: UserId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        let notification_id = NotificationId::generate(transaction).await?;

        Notification {
            id: notification_id.clone(),
            user_id,
            body: builder.body,
            created: Utc::now().naive_utc(),
            read: false, 
        }
        .insert(transaction)
        .await?;

        for action_builder in builder.actions {
            let action_id = NotificationActionId::generate(transaction).await?;

            NotificationAction {
                id: action_id,
                notification_id: notification_id.clone(),
                title: action_builder.title,
                action_endpoint: action_builder.action_endpoint,
            }
            .insert(transaction)
            .await?;
        }

        Ok(())
    }

    pub async fn remove(
        notification_id: NotificationId, 
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM notification_actions
            WHERE notification_id = ?
            ",
            notification_id
        )
        .execute(&mut **transaction)
        .await?;

        sqlx::query!(
            "
            DELETE FROM notifications
            WHERE id = ?
            ",
            notification_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn remove_all(
        user_id: UserId, 
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM notification_actions
            WHERE notification_id = (
                SELECT id 
                FROM notifications 
                WHERE user_id = ?
            )
            ",
            user_id
        )
        .execute(&mut **transaction)
        .await?;

        sqlx::query!(
            "
            DELETE FROM notifications
            WHERE user_id = ?
            ",
            user_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn read(
        notification_id: NotificationId, 
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE notifications
            SET read = TRUE
            WHERE id = ?
            ",
            notification_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn read_all(
        user_id: UserId, 
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE notifications
            SET read = TRUE
            WHERE user_id = ?
            ",
            user_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

impl Notification {
    pub async fn insert(
        &self, 
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO notifications (
                id, user_id, body, created,
                read
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.user_id,
            self.body,
            self.created,
            self.read
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    pub async fn get_many_full<'a, E>(
        user_id: UserId,
        executor: E
    ) -> Result<Vec<FullNotification>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database> + Copy,
    {
        sqlx::query_as!(
            FullNotification,
            "
            SELECT n.id, n.user_id, n.body, 
            n.created, n.read,
            JSON_GROUP_ARRAY(JSON_OBJECT(
                'id', na.id,
                'notification_id', na.notification_id,
                'title', na.title,
                'action_endpoint', na.action_endpoint
            )) AS actions
            FROM notifications n 
            LEFT OUTER JOIN notification_actions na
            ON n.id = na.notification_id
            WHERE n.user_id = ?
            GROUP BY n.id, n.user_id
            ",
            user_id
        )
        .fetch_all(executor)
        .await
    }
}

impl NotificationAction {
    pub async fn insert(
        &self, 
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO notification_actions (
                id, notification_id, title, 
                action_endpoint
            )
            VALUES (
                $1, $2, $3, $4
            )
            ",
            self.id,
            self.notification_id,
            self.title,
            self.action_endpoint
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}