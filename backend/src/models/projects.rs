use utoipa::ToSchema;

use crate::database::Database;
use crate::error::ApiError;

use super::id::{ProjectId, UserId, ProjectMemberId};

#[derive(Serialize, ToSchema)]
pub struct Project {
    // The project's id
    pub id: ProjectId,
    // The project's name (3 -> 30 charachters)
    pub name: String,
    // The project owner's id
    pub owner: UserId,
    // The project's icon's url
    pub icon_url: String,
    // The permissions of non-members.
    // by default this is none
    pub public_permissions: Permissions
}

#[derive(Deserialize, ToSchema)]
pub struct EditProject {
    // The project's new name (3 -> 30 charachters)
    #[schema(example = "My project", min_length = 3, max_length = 30)]
    pub name: Option<String>,
    // The project's new icon
    #[schema(example = "https://example.com/icon.png")]
    pub icon_url: Option<String>,
    // The project's new visibility
    pub public_permissions: Option<Permissions>
}

#[derive(Deserialize, ToSchema)]
pub struct ProjectBuilder {
    // The project's name (3 -> 30 charachters)
    #[schema(example = "My project", min_length = 3, max_length = 30)]
    name: String,
    // The project's icon's url
    #[schema(example = "https://example.com/icon.png")]
    icon_url: String,
    // The project's visibility
    public_permissions: Permissions
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
            public_permissions: form.public_permissions
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

    pub async fn edit(
        project_id: ProjectId,
        form: EditProject,
        transaction: &mut sqlx::Transaction<'_, Database>
    ) -> Result<(), sqlx::error::Error> {
        let permissions = form.public_permissions.map(|p| p.bits() as i64);

        sqlx::query!(
            "
            UPDATE projects
            SET name = coalesce($1, name),
                icon_url = coalesce($2, icon_url),
                public_permissions = coalesce($3, public_permissions)
            WHERE id = $4
            ",
            form.name,
            form.icon_url,
            permissions,
            project_id
        )
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }

    /// Removes the project and associated data from the database.
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
        let permssions = self.public_permissions.bits() as i64;

        sqlx::query!(
            "
            INSERT INTO projects (
                id, name, owner, icon_url,
                public_permissions
            )
            VALUES (
                $1, $2, $3, $4, $5
            )
            ",
            self.id,
            self.name,
            self.owner,
            self.icon_url,
            permssions
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
            SELECT id, name, owner, icon_url, 
                   public_permissions
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
            SELECT id, name, owner, icon_url, 
                   public_permissions
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
    #[derive(Serialize, Deserialize, ToSchema)]
    #[serde(transparent)]
    pub struct Permissions: u64 {
        // Permission to read any information about
        // the project such as it's description title,
        // icon, tasks, sub tasks, members etc. 
        //
        const READ_PROJECT = 1 << 0;
        // Permission to create tasks aswell as add
        // sub tasks to them.
        //
        const CREATE_TASKS = 1 << 1;
        // Permission to edit any task or sub task
        // even those created by other members
        //
        const EDIT_TASKS = 1 << 2;
        // Permission to remove any task even on
        // tasks that where not created by the user
        //
        const DELETE_TASKS = 1 << 3;
        // Permission to create and edit task groups 
        // in the project aswell as their position. 
        //
        const CREATE_TASK_GROUPS = 1 << 4;
        // Permission to remove any task group from
        // the project. Aswell as any attached data
        // such as tasks. 
        // 
        const DELETE_TASK_GROUPS = 1 << 5;
        // Permission to upload files to be stored
        // in the project's shared storage. 
        // 
        const UPLOAD_FILES = 1 << 6;
        // Permission to remove files from the 
        // project's shared storage. Including ones
        // that the user did not upload themselves
        //
        const REMOVE_FILES = 1 << 7;
        // Permission to invite users to the project
        // and add them as members and assign them
        // permissions up to their own.
        //
        const INVITE_MEMBERS = 1 << 7;
        // Permission to remove a user from the 
        // project's team aswell as any related data 
        // to the user such as assignments and 
        // optionally any tasks they created if the 
        // member removing them from the team has 
        // the `DELETE_TASKS` permission.
        //
        const REMOVE_MEMBERS = 1 << 8;
        // Permission to edit information about the
        // project such as it's name, description,
        // icon etc.
        //
        const EDIT_PROJECT = 1 << 9;
        // Permission to remove the project entirely
        // including all tasks, memberships, files,
        // etc. 
        //
        const DELETE_RPOJECT = 1 << 10;
    }
}

impl Permissions {
    pub fn check_contains(&self, permissions: Permissions) -> Result<(), ApiError> {
        if self.contains(permissions) {
            Ok(())
        } else {
            Err(ApiError::Forbidden)
        }
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
    /// Returns the default permissions of a new project
    /// member. This should not be used as a substitute if 
    /// the members permissions cannot be read as it may
    /// allow users to perform actions they should not be
    /// able to.
    /// 
    /// By default members have permission to read from the
    /// project and it's tasks files etc. Aswell as create
    /// remove and update tasks, sub-tasks and task groups.
    /// In addition they can upload and remove files from
    /// project storage. But cannot modify core information 
    /// about the project such as it's title, icon or 
    /// description. They also cannot manage other members 
    /// permissions, or invite new members to the project.
    ///
    fn default() -> Permissions {
        Permissions::READ_PROJECT | Permissions::CREATE_TASKS 
        | Permissions::EDIT_TASKS | Permissions::DELETE_TASKS 
        | Permissions::CREATE_TASK_GROUPS | Permissions::DELETE_TASK_GROUPS
    }
}

#[derive(Serialize, ToSchema)]
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
    pub async fn invite_users(
        user_ids: Vec<UserId>,
        project_id: ProjectId,
        transaction: &mut sqlx::Transaction<'_, Database>,
    ) -> Result<(), sqlx::error::Error> {
        for user_id in user_ids {
            let member = ProjectMember {
                id: ProjectMemberId::generate(&mut *transaction).await?,
                project_id: project_id.clone(),
                user_id,
                permissions: Permissions::all(),
                accepted: true,
            };

            // TODO: Send notification

            member.insert(&mut *transaction).await?;
        }

        Ok(())
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

    pub fn check_permissions(
        &self, 
        permissions: Permissions
    ) -> Result<(), ApiError> {
        self.permissions.check_contains(permissions)
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
        project_id: ProjectId,
        executor: E,
    ) -> Result<Option<Self>, sqlx::error::Error>
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
            AND project_id = $2
            ",
            user_id,
            project_id
        )
        .fetch_optional(executor)
        .await
    }

    pub async fn get_accepted_from_user<'a, E>(
        user_id: UserId,
        project_id: ProjectId,
        executor: E,
    ) -> Result<Option<Self>, sqlx::error::Error>
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
            AND project_id = $2
            AND accepted = true
            ",
            user_id,
            project_id
        )
        .fetch_optional(executor)
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

    pub async fn get_many_from_user<'a, E>(
        user_id: UserId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        Self::get_many("user_id", user_id.0, executor).await
    }

    pub async fn get_many_from_project<'a, E>(
        project_id: ProjectId,
        executor: E,
    ) -> Result<Vec<Self>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        Self::get_many("project_id", project_id.0, executor).await
    }

    pub async fn get_project<'a, E>(
        &self,
        executor: E,
    ) -> Result<Option<Project>, sqlx::error::Error>
    where
        E: sqlx::Executor<'a, Database = Database>
    {
        Project::get(self.project_id.clone(), executor).await
    }
}