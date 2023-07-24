use sqlx;

use crate::database::{DatabaseError, Database};
use super::ids::{NotifcationId, UserId, ProjectId, TaskId};

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
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), super::DatabaseError> {
        self.create_many(vec![recipient], &mut *transaction).await
    }

    pub async fn create_many(
        &self,
        recipients: Vec<UserId>,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), super::DatabaseError> {
        for recipient in recipients {
            let id = NotifcationId::generate(&mut *transaction).await?;

            Notification {
                id,
                recipient: recipient.clone(),
                body: self.body.clone(),
                notification_type: self.notification_type.clone()
            }
            .insert(&mut *transaction)
            .await?;
        }

        Ok(())
    }
}

impl Notification {
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
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
            self.id,
            self.recipient,
            self.body,
            notification_type
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }

    pub async fn get(
        notification_id: NotifcationId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Option<Self>, DatabaseError> {
        let query = sqlx::query!(
            "
            SELECT id, recipient, body, 
                   notification_type
            FROM NOTIFICATIONS
            WHERE id = $1
            ",
            notification_id
        )
        .fetch_optional(&mut *transaction)
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