use chrono::{NaiveDateTime, Utc};
use poem_openapi::Object;

use crate::database::Database;

use super::{id::{ProjectMemberId, ProjectId, AuditId}, projects::ProjectMember};

#[derive(Object, Serialize)]
pub struct Audit {
    // The audit's id
    pub id: AuditId,
    // The auditor's membership's id
    pub auditor: ProjectMemberId,
    // The parent project's id
    pub project_id: ProjectId,
    // The body of the audit
    pub body: String,
    // The datetime the audit was created
    pub timestamp: NaiveDateTime
}

impl Audit {
    /// Creates a new `Audit` record in the database.
    /// 
    /// This method is used to create a new audit record with the provided details and inserts it into the database within the specified transaction.
    /// 
    /// # Arguments
    /// 
    /// * `auditor`: A reference to a `ProjectMember` representing the auditor associated with the audit.
    /// * `body`: A `String` containing the details or body of the audit.
    /// * `transaction`: A mutable reference to an `sqlx::Transaction` representing the database transaction to perform the insertion.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing the newly created `Audit` if the insertion was successful, or an `sqlx::error::Error` if there was an error executing the insertion query.
    /// 
    pub async fn create(
        auditor: &ProjectMember,
        body: String,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<Self, sqlx::error::Error> {
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
}

impl Audit {
    /// Retrieves an `Audit` record from the database based on the given `id`.
    /// 
    /// # Arguments
    /// 
    /// * `id`: An `AuditId` representing the unique identifier of the audit record to retrieve.
    /// * `executor`: An SQLx Executor implementing the `Executor` trait for the specific database (`Database`) and lifetime `'a`.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing an `Option<Audit>` if the record exists, or an `sqlx::error::Error` if there was an error executing the query.
    /// If the record with the provided `id` exists, it will be wrapped in `Some`, otherwise, it will be `None`.
    /// 
    pub async fn get<'a, E>(
        id: AuditId,
        executor: E,
    ) -> Result<Option<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        sqlx::query_as!(
            Audit,
            "
            SELECT id, auditor, project_id, 
                   body, timestamp
            FROM audit_log
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(executor)
        .await
    }

    /// Retrieves multiple `Audit` records from the database based on the provided column name and value.
    /// 
    /// # Arguments
    /// 
    /// * `column`: A reference to a `&str` representing the column name in the database to filter the query by.
    /// * `value`: A `String` representing the value to compare against the specified column in the database.
    /// * `executor`: An SQLx Executor implementing the `Executor` trait for the specific database (`Database`) and lifetime `'a`.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing a `Vec<Audit>` with all matching records if any, or an `sqlx::error::Error` if there was an error executing the query.
    /// If no records are found, the vector will be empty.
    /// 
    pub async fn get_many<'a, E>(
        column: &str,
        value: String,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        sqlx::query_as!(
            Audit,
            "
            SELECT id, auditor, project_id, 
                   body, timestamp
            FROM audit_log
            WHERE $1 = $2
            ",
            column,
            value
        )
        .fetch_all(executor)
        .await
    }

    /// Retrieves multiple `Audit` records from the database that belong to a specific project based on the given `project_id`.
    /// 
    /// This method is a convenience function that internally calls the `get_many` method, filtering the audits by the `project_id` column.
    /// 
    /// # Arguments
    /// 
    /// * `project_id`: A `ProjectId` representing the unique identifier of the project whose audits are to be retrieved.
    /// * `executor`: An SQLx Executor implementing the `Executor` trait for the specific database (`Database`) and lifetime `'a`.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing a `Vec<Audit>` with all matching records if any, or an `sqlx::error::Error` if there was an error executing the query.
    /// If no records are found, the vector will be empty.
    /// 
    pub async fn get_many_from_project<'a, E>(
        project_id: ProjectId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error> 
    where
        E: sqlx::Executor<'a, Database = Database>,
    {
        Self::get_many("project_id", project_id.0, executor).await
    }

    /// Inserts an `Audit` record into the database.
    /// 
    /// # Arguments
    /// 
    /// * `transaction`: A mutable reference to an `sqlx::Transaction` representing the database transaction to perform the insertion.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing `()` if the insertion was successful, or an `sqlx::error::Error` if there was an error executing the insertion query.
    /// 
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>
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
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}