use core::task;

use crate::middleware::project::ProjectAuthentication;
use crate::utilities::validation_utils::{validate_future_date, validate_hex_colour, validation_errors_to_string};
use crate::utilities::auth_utils::AuthenticationError::{NotMember, MissingPermissions};
use crate::models::ids::{TaskId, ProjectId, TaskGroupId};
use crate::models::tasks::{Task, TaskBuilder, SubTask, SubTaskBuilder};
use crate::models::projects::{ProjectMember, Permissions};
use crate::models::audit::Audit;
use crate::{response,};
use crate::routes::ApiError;

use actix_web::dev::{ServiceRequest, Service};
use actix_web::{web, HttpResponse, get, post, delete, middleware};
use actix_web::http::StatusCode;

use chrono::NaiveDateTime;
use validator::Validate;

use crate::database::SqlPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
    web::scope("/tasks/{task_id}")
                .wrap(ProjectAuthentication {
                    id_key: "task_id".to_string(),
                    table_name: Some("tasks".to_string()),
                })
                .service(get)   
                .service(edit)
                .service(delete)
            );
}

#[get("/")]
pub async fn get(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, super::ApiError> {
    let task_id = TaskId(path.0.clone());

    let task = Task::get_full(task_id, &**pool)
        .await?
        .ok_or_else(|| ApiError::NotFound("Could not find task".to_string()))?;

    response!(StatusCode::OK, task, "Successfully found task")
}

#[derive(Deserialize, Validate)]
pub struct EditTask {
    pub task_group_id: Option<TaskGroupId>,
    #[validate(length(min = 3, max = 30))]
    pub name: Option<String>,
    pub information: Option<String>,
    #[validate(custom = "validate_future_date")]
    pub due: Option<NaiveDateTime>,
    pub position: Option<i64>,
    #[validate(custom = "validate_hex_colour")]
    pub primary_colour: Option<String>,
    #[validate(custom = "validate_hex_colour")]
    pub accent_colour: Option<String>
}

#[post("/edit")]
pub async fn edit(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>,
    req: actix_web::HttpRequest,
    form: web::Json<EditTask>,
) -> Result<HttpResponse, super::ApiError> {
    let form = form.into_inner();

    let task_id = TaskId(path.0.clone());

    let mut transaction = pool.begin().await?;

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    let task = Task::get(task_id.clone(), &mut transaction).await?
        .ok_or_else(|| super::ApiError::NotFound("Could not find task to edit".to_string()))?;

    if let Some(position) = form.position {
        let task_group = form.task_group_id.clone()
            .unwrap_or(task.task_group_id.clone());

        sqlx::query!(
            "
            UPDATE tasks
            SET position = position + 1
            WHERE position >= $1
            AND task_group_id = $2
            ",
            position,
            task_group
        )
        .execute(&mut transaction)
        .await?;

        // Dont create an audit entry for this :)
    }

    if let Some(task_group_id) = form.task_group_id.clone() {
        if form.position.is_none() {
            Err(super::ApiError::InvalidInput("Cannot move groups without updating position".to_string()))?;
        }

        sqlx::query!(
            "
            UPDATE tasks
            SET task_group_id = $1
            WHERE id = $2
            ",
            task_group_id,
            task_id,
        )
        .execute(&mut transaction)
        .await?;

        // Move tasks from previous group back
        sqlx::query!(
            "
            UPDATE tasks
            SET position = position - 1
            WHERE position >= $1
            AND task_group_id = $2
            ",
            task.position, // Use old position
            task.task_group_id
        )
        .execute(&mut transaction)
        .await?;

        Audit::create(
            &member, 
            format!("Moved task '{}'", task.name),
            &mut transaction
        ).await?;
    }

    if let Some(name) = form.name {
        sqlx::query!(
            "
            UPDATE tasks
            SET name = $1
            WHERE id = $2
            ",
            name,
            task_id
        )
        .execute(&mut transaction)
        .await?;

        Audit::create(
            &member,
            format!("Renamed task from '{}' to '{}'", task.name, name),
            &mut transaction
        ).await?;
    }

    if let Some(information) = form.information {
        sqlx::query!(
            "
            UPDATE tasks
            SET information = $1
            WHERE id = $2
            ",
            information,
            task_id
        )
        .execute(&mut transaction)
        .await?;

        Audit::create(
            &member,
            format!("Updated description of task '{}'", task.name),
            &mut transaction
        ).await?;
    }

    if let Some(due) = form.due {
        sqlx::query!(
            "
            UPDATE tasks
            SET due = $1
            WHERE id = $2
            ",
            due,
            task_id
        )
        .execute(&mut transaction)
        .await?;

        Audit::create(
            &member, 
            format!("Updated due date of task '{}'", task.name), 
            &mut transaction
        ).await?;
    }

    if let Some(primary_colour) = form.primary_colour {
        sqlx::query!(
            "
            UPDATE tasks
            SET primary_colour = $1
            WHERE id = $2
            ",
            primary_colour,
            task_id
        )
        .execute(&mut transaction)
        .await?;

        // Dont create an audit entry for this
    }

    if let Some(accent_colour) = form.accent_colour {
        sqlx::query!(
            "
            UPDATE tasks
            SET accent_colour = $1
            WHERE id = $2
            ",
            accent_colour,
            task_id
        )
        .execute(&mut transaction)
        .await?;
        
        // Dont create an audit entry for this
    }

    transaction.commit().await?;
    response!(StatusCode::OK, "Successfully edited task")
}


#[get("/sub-tasks/")]
pub async fn get_subtasks(
    path: web::Path<(String, String)>,
    pool: web::Data<SqlPool>,
) -> Result<HttpResponse, super::ApiError> {
    let project_id = ProjectId(path.0.clone());
    let task_id = TaskId(path.1.clone());

    let sub_tasks = Task::get_sub_tasks(task_id, &**pool).await?;

    response!(StatusCode::OK, sub_tasks, "Successfully found subtasks")
}

#[post("/sub-tasks/create")]
pub async fn create_subtask(
    path: web::Path<(String, String)>,
    pool: web::Data<SqlPool>,
    form: web::Json<SubTaskBuilder>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let form = form.into_inner();

    let project_id = ProjectId(path.0.clone());
    let task_id = TaskId(path.1.clone());

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    form.create(project_id, task_id, &mut transaction).await?;
    transaction.commit().await?;

    response!(StatusCode::OK, "Successfully created subtask")
}

#[delete("/")]
pub async fn delete(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let task_id = TaskId(path.0.clone());

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    let task = Task::get(task_id.clone(), &mut transaction).await?
        .ok_or_else(|| super::ApiError::NotFound("Could not find task to edit".to_string()))?;

    task.delete(&mut transaction).await?;
    Audit::create(&member, format!("Deleted task '{}'", task.name), &mut transaction);

    transaction.commit().await?;

    response!(StatusCode::OK, "Successfully deleted task")
}