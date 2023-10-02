use chrono::{NaiveDateTime, Utc};
use utoipa::ToSchema;

use crate::database::Database;

use super::id::{NotificationId, NotificationActionId, UserId};

#[derive(Serialize, ToSchema)]
pub struct Notification {
    /// The id of the notification
    /// 
    #[schema(example="1234567890", min_length=10, max_length=10)]
    pub id: NotificationId,
    /// The id of the user set to recieve the notification
    /// 
    #[schema(example="12345678", min_length=8, max_length=8)]
    pub user_id: UserId,
    /// The body (message) of the notification
    /// 
    #[schema(example="Hello World")]
    pub body: String,
    /// The time the notification was created and sent
    /// 
    pub created: NaiveDateTime,
    /// Whether the notification has been read or not by the
    /// recipient.
    /// 
    #[schema(example=false)]
    pub read: bool
}

pub struct NotificationBuilder {
    /// The body (message) of the notification
    pub body: String,
    /// The actions that will be available to the recipient
    pub actions: Vec<NotificationActionBuilder>
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct NotificationAction {
    /// The id of the action
    /// 
    #[schema(example="1234567890", min_length=10, max_length=10)]
    pub id: NotificationActionId,
    /// The id of the notification this action belongs to
    /// 
    #[schema(example="1234567890", min_length=10, max_length=10)]
    pub notification_id: NotificationId,
    /// The title (description) of the action
    /// 
    #[schema(example="Hello World")]
    pub title: String,
    /// The endpoint to perform the action
    /// 
    #[schema(example="https://example.com/action")]
    pub action_endpoint: String
}

pub struct NotificationActionBuilder {
    /// The title (description) of the action
    /// 
    pub title: String,
    /// The endpoint to perform the action
    /// 
    pub action_endpoint: String
}

#[derive(Serialize, ToSchema)]
pub struct FullNotification {
    /// The id of the notification
    ///
    #[schema(example="1234567890", min_length=10, max_length=10)]
    pub id: NotificationId,
    /// The id of the user set to recieve the notification
    /// 
    #[schema(example="12345678", min_length=8, max_length=8)]
    pub user_id: UserId,
    /// The body (message) of the notification
    /// 
    #[schema(example="Hello World")]
    pub body: String,
    /// The datetime the notification was created and sent
    /// 
    pub created: NaiveDateTime,
    /// Weather or not the recipient has read the notification
    /// 
    #[schema(example=false)]
    pub read: bool,
    /// The actions that will be available to the recipient
    /// 
    pub actions: Actions
}

/// Additional struct in order to be able to directly
/// deserialze the actions field of the notification
///
#[derive(Deserialize, Serialize, ToSchema)]
pub struct Actions(pub Vec<NotificationAction>);

impl From<String> for Actions {
    /// In order to read multiple notification actions
    /// at once from one notification we use a json
    /// aggregator which then results in a string which
    /// we can then parse into our actions object
    /// 
    fn from(value: String) -> Self {
        Self(serde_json::from_str(&value).unwrap_or_default())
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