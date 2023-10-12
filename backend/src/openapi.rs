use std::{fs::File, io::Write, path::Path};

use utoipa::OpenApi;

use crate::{api, models, cli::OpenApiSchemaArguements};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Nine Yards REST API",
        version = "0.0.1",
    ),
    paths(
        api::v1::users::get_current_user,
        api::v1::users::get_user_by_id,
        api::v1::users::register,
        api::v1::users::login,

        api::v1::projects::create_project,
        api::v1::projects::get_memberships_from_user,
        api::v1::projects::get_project_by_id,
        api::v1::projects::update_project,
        api::v1::projects::remove_project,
        api::v1::projects::get_audits,
        api::v1::projects::get_members,
        api::v1::projects::invite_member,
        api::v1::projects::get_task_groups,
        api::v1::projects::create_task_group,

        api::v1::task_groups::get_task_group_by_id,
        api::v1::task_groups::edit_task_group,
        api::v1::task_groups::remove_task_group,
        api::v1::task_groups::get_tasks,

        api::v1::tasks::get_task,
        api::v1::tasks::edit_task,
        api::v1::tasks::remove_task,
        api::v1::tasks::get_sub_tasks,
        api::v1::tasks::create_sub_task,

        api::v1::sub_tasks::get_sub_task_by_id,
        api::v1::sub_tasks::edit_sub_task,
        api::v1::sub_tasks::remove_sub_task
    ),
    components(schemas(
        models::id::UserId,
        models::id::ProjectId,
        models::id::ProjectMemberId,
        models::id::TaskGroupId,
        models::id::TaskId,
        models::id::SubTaskId,
        models::id::AuditId,
        models::id::NotificationId,
        models::id::NotificationActionId,

        models::users::User,
        models::users::Register,
        models::users::Login,

        models::audits::Audit,

        models::tokens::Token,

        models::notifications::Notification,
        models::notifications::NotificationAction,
        models::notifications::FullNotification,
        models::notifications::Actions,

        models::projects::Project,
        models::projects::EditProject,
        models::projects::ProjectBuilder,
        models::projects::ProjectMember,
        
        models::tasks::TaskGroup,
        models::tasks::EditTaskGroup,
        models::tasks::TaskGroupBuilder,
        models::tasks::Task,
        models::tasks::SubTasks,
        models::tasks::FullTask,
        models::tasks::TaskBuilder,
        models::tasks::EditTask,
        models::tasks::SubTask,
        models::tasks::EditSubTask,
        models::tasks::SubTaskBuilder,
    ))
)]
pub struct ApiDoc;

#[derive(thiserror::Error, Debug)]
enum SchemaGenerationError {
    #[error("Invalid file type, must be .json or .yaml / .yml")]
    InvalidFileType,
    #[error("The specified path does not point to a file")]
    InvalidLocation
}

pub async fn write(
    OpenApiSchemaArguements {
        output_location,
        ..
    }: OpenApiSchemaArguements
) -> Result<(), Box<dyn std::error::Error>> {
    let openapi = ApiDoc::openapi();

    let schema = match Path::new(&output_location)
        .extension()
        .ok_or(SchemaGenerationError::InvalidLocation)?
        .to_str() {
        Some("json") => openapi.to_pretty_json()?,
        Some("yaml") | Some("yml") => openapi.to_yaml()?,
        _ => return Err(SchemaGenerationError::InvalidFileType.into())
    };

    let mut output = File::create(output_location)?;
    
    output.write_all(schema.as_bytes())?;

    Ok(())
}