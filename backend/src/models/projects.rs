use sqlx::FromRow;

use crate::database::Database;

use super::id::{ProjectId, UserId, ProjectMemberId};

#[derive(Serialize)]
pub struct Project {
    // The project's id
    pub id: ProjectId,
    // The project's name (3 -> 30 charachters)
    pub name: String,
    // The project owner's id
    pub owner: UserId,
    // The project's icon's url
    pub icon_url: String,
    // The project's visibility
    pub public: bool
}

#[derive(Deserialize)]
pub struct ProjectBuilder {
    // The project's name (3 -> 30 charachters)
    name: String,
    // The project's icon's url
    icon_url: String,
    // The project's visibility
    public: bool
}

impl Project {
    /// Creates a new project and inserts it into the database.
    ///
    /// # Arguments
    ///
    /// * `form`: A `ProjectBuilder` instance containing the project details to be created.
    /// * `creator`: The `UserId` of the user who is creating the project.
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Self, sqlx::error::Error>`, where:
    /// - `Ok(project)` is returned with the created `Project` instance if the creation and insertion are successful.
    /// - An `sqlx::error::Error` is returned if there is an error generating IDs or executing the database queries.
    ///
    pub async fn create(
        form: ProjectBuilder,
        creator: UserId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, sqlx::error::Error> {
        let id = ProjectId::generate(&mut *transaction).await?;

        let project = Self {
            id,
            name: form.name.clone(),
            owner: creator.clone(),
            icon_url: form.icon_url.clone(),
            public: form.public
        };

        project.insert(transaction).await?;

        let id = ProjectMemberId::generate(&mut *transaction).await?;

        let project_member = ProjectMember {
            id,
            project_id: project.id.clone(),
            user_id: creator,
            permissions: Permissions::all(),
            accepted: true
        };

        project_member.insert(&mut *transaction).await?;

        Ok(project)
    }

    /// Removes the project and associated data from the database.
    ///
    /// # Arguments
    ///
    /// * `id`: The `ProjectId` of the project to be removed.
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the project and associated data are successfully removed from the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database queries.
    ///
    pub async fn remove(
        id: ProjectId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        // Remove all associated sub tasks
        sqlx::query!(
            "
            DELETE FROM sub_tasks
            WHERE project_id = $1
            ",
            id
        )
        .execute(&mut **transaction)
        .await?;
        // Remove all associated tasks
        sqlx::query!(
            "
            DELETE FROM tasks
            WHERE project_id = $1
            ",
            id
        )
        .execute(&mut **transaction)
        .await?;
        // Remove all associated task groups
        sqlx::query!(
            "
            DELETE FROM task_groups
            WHERE project_id = $1
            ",
            id
        )
        .execute(&mut **transaction)
        .await?;
        // Remove all memberships
        sqlx::query!(
            "
            DELETE FROM project_members
            WHERE project_id = $1
            ",
            id
        )
        .execute(&mut **transaction)
        .await?;
        // Remove the project itself
        sqlx::query!(
            "
            DELETE FROM projects
            WHERE id = $1
            ",
            id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

impl Project {
    /// Inserts the project into the database.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the insertion is successful.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            INSERT INTO projects (
                id, name, owner, icon_url,
                public
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.name,
            self.owner,
            self.icon_url,
            self.public
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// Retrieves a project from the database by its ID.
    ///
    /// # Arguments
    ///
    /// * `id`: The `ProjectId` of the project to retrieve.
    /// * `executor`: An implementation of `sqlx::Executor`, representing the database executor.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Option<Self>, sqlx::error::Error>`, where:
    /// - `Ok(Some(project))` is returned with the retrieved `Project` if it exists in the database.
    /// - `Ok(None)` is returned if no project is found with the specified ID.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get<'a, E>(
        id: ProjectId,
        executor: E,
    ) -> Result<Option<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        let project = sqlx::query_as!(
            Project,
            "
            SELECT id, name, owner,
                   icon_url, public
            FROM projects
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(project)
    }

    /// Retrieves multiple projects from the database based on the specified column and value.
    ///
    /// # Arguments
    ///
    /// * `column`: The column name to search for the specified value.
    /// * `value`: The value to match in the specified column.
    /// * `executor`: An implementation of `sqlx::Executor`, representing the database executor.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Vec<Self>, sqlx::error::Error>`, where:
    /// - `Ok(projects)` is returned with a vector of retrieved `Project` instances that match the specified column and value.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get_many<'a, E>(
        column: &str,
        value: String,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database>,
    {
        let results = sqlx::query_as!(
            Project,
            "
            SELECT id, name, owner,
                   icon_url, public
            FROM projects
            WHERE $1 = $2
            ",
            column,
            value
        )
        .fetch_all(executor)
        .await?;

        Ok(results)
    }
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Permissions: u64 {
        // Allows for tasks to be created, removed, assigned and editted
        const MANAGE_TASKS = 1 << 0;
        // Allows for the project settings to be changed
        const MANAGE_PROJECT = 1 << 1;
        // Allows for the project members to be added and removed and roles changed
        const MANAGE_TEAM = 1 << 2;
    }
}

impl From<i64> for Permissions {
    /// Converts an integer value into a `Permissions` bitset.
    ///
    /// # Arguments
    ///
    /// * `value`: An `i64` value representing the integer value to convert into the `Permissions` bitset.
    ///
    /// # Returns
    ///
    /// This method returns a `Permissions` instance, which is a bitset representing the permissions.
    /// or a the default permission set if the value given is invalid
    ///
    fn from(value: i64) -> Self {
        Permissions::from_bits(value as u64).unwrap_or_default()
    }
}

impl Default for Permissions {
    /// Returns the default value for the `Permissions` enum.
    ///
    /// The `default` function provides a way to create a default value for the `Permissions` enum.
    /// When no explicit value is provided, this default value is used. For the `Permissions` enum,
    /// the default value returned is `Permissions::MANAGE_TASKS`.
    ///
    /// # Returns
    ///
    /// The default value for the `Permissions` enum, which is `Permissions::MANAGE_TASKS`.
    ///
    fn default() -> Permissions {
        Permissions::MANAGE_TASKS
    }
}

#[derive(Serialize, FromRow)]
pub struct ProjectMember {
    // The project member's id
    pub id: ProjectMemberId,
    // The project the membership is for's id
    pub project_id: ProjectId,
    // The user's id
    pub user_id: UserId,
    // The user's permissions 
    pub permissions: Permissions,
    // Whether the user has accepted the project's invitation
    pub accepted: bool,

}

impl ProjectMember {
    /// Invites a user to the project and inserts the invitation as a `ProjectMember` into the database.
    ///
    /// # Arguments
    ///
    /// * `user`: The `UserId` of the user to be invited to the project.
    /// * `project`: A reference to the `Project` instance to which the user is being invited.
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<ProjectMember, sqlx::error::Error>`, where:
    /// - `Ok(member)` is returned with the created `ProjectMember` instance representing the invitation
    ///    if the insertion is successful and the user is invited to the project.
    /// - An `sqlx::error::Error` is returned if there is an error generating an ID or executing the database query.
    ///
    pub async fn invite_user(
        user: UserId,
        project: &Project,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<Self, sqlx::error::Error> {
        let member_id = ProjectMemberId::generate(&mut *transaction).await?;

        let member = ProjectMember {
            id: member_id,
            project_id: project.id.clone(),
            user_id: user,
            permissions: Permissions::all(),
            accepted: true,
        };

        member.insert(&mut *transaction).await?;

        Ok(member)
    }

    /// Accepts the invitation to the project by setting the "accepted" field to true in the database.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the acceptance is successful and the "accepted" field is updated to true in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn accept_invitation(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            UPDATE project_members
            SET accepted = true
            WHERE id = $1
            ",
            self.id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// Denies the invitation to the project by removing the project member from the database
    /// if the member's "accepted" field is false (not accepted).
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the denial is successful, and the project member is removed from the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn deny_invitation(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
            "
            DELETE FROM project_members
            WHERE id = $1
            AND accepted = false
            ",
            self.id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// Allows the project member to leave the project by removing the membership from the database.
    /// This method is typically used by members who have voluntarily decided to leave the project.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the member successfully leaves the project, and the membership is removed from the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn leave(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        // Remove all task assignments
        sqlx::query!(
            "
            UPDATE sub_tasks
            SET assignee = NULL
            WHERE assignee = $1
            ",
            self.id,
        )
        .execute(&mut **transaction)
        .await?;
        // Remove the membership itself
        sqlx::query!(
            "
            DELETE FROM project_members
            WHERE id = $1
            ",
            self.id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}

impl ProjectMember {
    /// Inserts the `ProjectMember` instance into the `project_members` table in the database.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A mutable reference to a `sqlx::Transaction`, representing a database transaction.
    ///
    /// # Returns
    ///
    /// This method returns `Result<(), sqlx::error::Error>`, where:
    /// - `Ok(())` is returned if the insertion is successful, and the `ProjectMember` is added to the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn insert(
        &self,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        let permssions = self.permissions.bits() as i64;

        sqlx::query!(
            "
            INSERT INTO project_members (
                id, project_id, user_id,
                permissions, accepted
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.project_id,
            self.user_id,
            permssions,
            self.accepted
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// Retrieves a `ProjectMember` from the `project_members` table based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id`: The `ProjectId` of the `ProjectMember` to retrieve.
    /// * `executor`: A type implementing `sqlx::Executor` that represents the database connection.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Option<Self>, sqlx::error::Error>`, where:
    /// - `Ok(Some(member))` is returned with the retrieved `ProjectMember` instance if found in the database.
    /// - `Ok(None)` is returned if no `ProjectMember` with the specified ID is found in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get<'a, E>(
        id: ProjectMemberId,
        executor: E,
    ) -> Result<Option<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        let project = sqlx::query_as!(
            ProjectMember,
            "
            SELECT id, project_id, user_id,
                   permissions, accepted
            FROM project_members
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(project)
    }

    /// Retrieves a `ProjectMember` from the `project_members` table based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id`: The `ProjectId` of the `ProjectMember` to retrieve.
    /// * `executor`: A type implementing `sqlx::Executor` that represents the database connection.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Option<Self>, sqlx::error::Error>`, where:
    /// - `Ok(Some(member))` is returned with the retrieved `ProjectMember` instance if found in the database.
    /// - `Ok(None)` is returned if no `ProjectMember` with the specified ID is found in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get_from_user<'a, E>(
        user_id: UserId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        sqlx::query_as!(
            ProjectMember,
            "
            SELECT id, project_id, user_id,
                   permissions, accepted
            FROM project_members
            WHERE user_id = $1
            ",
            user_id,
        )
        .fetch_all(executor)
        .await
    }

    /// Retrieves a list of `ProjectMember` instances from the `project_members` table
    /// based on the specified column and value.
    ///
    /// # Arguments
    ///
    /// * `column`: The name of the column to use in the WHERE clause of the query.
    /// * `value`: The value to use for filtering the results in the specified column.
    /// * `executor`: A type implementing `sqlx::Executor` that represents the database connection.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Vec<Self>, sqlx::error::Error>`, where:
    /// - `Ok(members)` is returned with the list of `ProjectMember` instances if found in the database.
    /// - An empty vector is returned if no `ProjectMember` with the specified column and value is found in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get_many<'a, E>(
        column: &str,
        value: String,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where 
        E: sqlx::Executor<'a, Database = Database>,
    {
        let results = sqlx::query_as!(
            ProjectMember,
            "
            SELECT id, project_id, user_id,
                   permissions, accepted
            FROM project_members
            WHERE $1 = $2
            ",
            column,
            value
        )
        .fetch_all(executor)
        .await?;

        Ok(results)
    }

    /// Retrieves a list of `ProjectMember` instances for a given `UserId`.
    ///
    /// # Arguments
    ///
    /// * `user`: The `UserId` for which to retrieve the project memberships.
    /// * `executor`: A type implementing `sqlx::Executor` that represents the database connection.
    ///
    /// # Returns
    ///
    /// This method returns `Result<Vec<Self>, sqlx::error::Error>`, where:
    /// - `Ok(members)` is returned with the list of `ProjectMember` instances if found in the database.
    /// - An empty vector is returned if the user does not have any project memberships in the database.
    /// - An `sqlx::error::Error` is returned if there is an error executing the database query.
    ///
    pub async fn get_memberships<'a, E>(
        user: UserId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        Self::get_many("user_id", user.0, executor).await
    }

}