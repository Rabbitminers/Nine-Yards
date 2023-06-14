use chrono::{NaiveDateTime, Utc};
use sqlx::SqlitePool;
use sqlx;

use crate::database::DatabaseError;

use super::error::ServiceError;
use super::ids::{NotifcationId, UserId, generate_notification_id, ProjectId, TaskId};

#[derive(Serialize, Deserialize)]
pub struct Notification {
    pub id: NotifcationId,
    pub recipient: UserId,
    pub body: String,
    pub notification_type: NotificationType,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NotificationType {
    ProjectInvite {
        project_id: ProjectId,
        invited_by: UserId,  
    },
    TaskAdded {
        project_id: ProjectId,
        task_id: TaskId,
    },
    TaskAssigned {
        project_id: ProjectId,
        task_id: TaskId,
    },
    MilestoneCompleted {
        project_id: ProjectId,
        task_id: TaskId,
        user_id: UserId,
    },
    Unknown
}

#[derive(Serialize, Deserialize)]
pub struct NotificationCallback {
    pub action_name: String,
    pub description: String,
    pub endpoint: String,
}

pub struct NotificationBuilder {
    pub body: String,
    pub notification_type: NotificationType
}

impl NotificationBuilder {
    pub async fn create(
        &self,
        recipient: UserId,
        conn: &SqlitePool
    ) -> Result<(), ServiceError> {
        self.create_many(vec![recipient], conn).await
    }

    pub async fn create_many(
        &self,
        recipients: Vec<UserId>,
        conn: &SqlitePool
    ) -> Result<(), ServiceError> {
        for recipient in recipients {
            let id = generate_notification_id(conn).await?;

            Notification {
                id,
                recipient: recipient.clone(),
                body: self.body.clone(),
                notification_type: self.notification_type.clone()
            }
            .insert(conn)
            .await?;
        }

        Ok(())
    }
}

impl Notification {
    pub async fn insert(
        &self,
        conn: &SqlitePool
    ) -> Result<(), super::DatabaseError> {
        let notification_type = serde_json::to_string(&self.notification_type)?;

        sqlx::query!(
            "
            INSERT INTO notifications (
                id, recipient, body,
                notification_type
            ) VALUES (
                $1, $2, $3, $4
            )
            ",
            self.id.0,
            self.recipient.0,
            self.body,
            notification_type
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn get(
        notification_id: NotifcationId,
        conn: &SqlitePool
    ) -> Result<Option<Self>, DatabaseError> {
        let query = sqlx::query!(
            "
            SELECT id, recipient, body, 
                   notification_type
            FROM NOTIFICATIONS
            WHERE id = $1
            ",
            notification_id.0
        )
        .fetch_optional(conn)
        .await?;
        
        if let Some(row) = query {
            Ok(Some(Self {
                id: NotifcationId(row.id),
                recipient: UserId(row.recipient),
                body: row.body,
                notification_type: serde_json::from_str(&row.notification_type)?
            }))
        } else {
            Ok(None)
        }
    }
}