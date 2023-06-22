use super::projects::ProjectMember;
use super::ids::{ProjectId, ProjectMemberId, AuditId};
use crate::database::Database;

use chrono::{Utc, NaiveDateTime};
use futures::TryStreamExt;

#[derive(Serialize)]
pub struct Audit {
    pub id: AuditId,
    pub auditor: ProjectMemberId,
    pub project_id: ProjectId,
    pub body: String,
    pub timestamp: NaiveDateTime,
}

impl Audit {
    pub async fn create(
        auditor: &ProjectMember,
        body: String,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<Self, super::DatabaseError> {
        let id = AuditId::generate(&mut *transaction).await?;

        let audit = Self {
            id,
            auditor: auditor.id.clone(),
            project_id: auditor.project_id.clone(),
            body,
            timestamp: Utc::now().naive_utc(),
        };

        audit.insert(&mut *transaction).await?;

        Ok(audit)
    }

    pub async fn get(
        id: AuditId,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<Option<Self>, sqlx::error::Error> {
        let query = sqlx::query!(
            "
            SELECT id, auditor, project_id, 
                   body, timestamp
            FROM audit_log
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(&mut *transaction)
        .await?;

        if let Some(row) = query {
            Ok(Some(Self {
                id: AuditId(row.id),
                auditor: ProjectMemberId(row.auditor),
                project_id: ProjectId(row.project_id),
                body: row.body,
                timestamp: row.timestamp,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_many<'a, E>(
        project_id: ProjectId,
        transaction: E
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        let audit_log = sqlx::query!(
            "
            SELECT id, auditor, project_id, 
                   body, timestamp
            FROM audit_log
            WHERE project_id = $1
            ",
            project_id
        )
        .fetch_many(transaction)
        .try_filter_map(|e| async {
            Ok(e.right().map(|m| Self {
                id: AuditId(m.id),
                auditor: ProjectMemberId(m.auditor),
                project_id: ProjectId(m.project_id),
                body: m.body,
                timestamp: m.timestamp,
            }))
        })
        .try_collect::<Vec<Self>>()
        .await?;

        Ok(audit_log)
    }

    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO audit_log (
                id, auditor, project_id, 
                body, timestamp
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.auditor,
            self.project_id,
            self.body,
            self.timestamp
        )
        .execute(&mut *transaction)
        .await?;

        Ok(())
    }
}