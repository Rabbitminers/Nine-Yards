use actix_web::{web, HttpResponse, get, http::StatusCode, post, delete, dev::ServiceRequest};
use log::info;
use validator::Validate;

use crate::{database::SqlPool, models::{ids::{TaskGroupId}, tasks::{TaskGroup, TaskBuilder}, projects::{ProjectMember, Permissions}, audit::Audit}, response, utilities::{validation_utils::validation_errors_to_string, auth_utils::AuthenticationError::{NotMember, MissingPermissions}}, middleware::project::ProjectAuthentication};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
    web::scope("/task-groups/{task_group_id}")
                .wrap(ProjectAuthentication {
                    id_key: "task_group_id".to_string(),
                    table_name: Some("task_groups".to_string())
                })
                .service(get)
                .service(get_tasks)
                .service(edit)
                .service(remove)
                .service(
            web::scope("/tasks")        
                        .service(get_tasks)
                        .service(create_task)
                )
        );
}

// Id-ed Task group routes

#[get("/")]
pub async fn get(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let group_id = TaskGroupId(path.0.clone());

    let task_group = TaskGroup::get(group_id, &**pool).await?
        .ok_or_else(|| super::ApiError::NotFound("Task group not found".to_string()))?;

    response!(StatusCode::OK, task_group, "Successfully retrieved task group")
}


#[derive(Deserialize, Validate)]
pub struct TaskGroupEdit {
    #[validate(length(min = 3, max = 30))]
    pub name: Option<String>,
    pub position: Option<i64>
}

#[post("/edit")]
pub async fn edit(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>,
    form: web::Json<TaskGroupEdit>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, super::ApiError> {
    let form = form.into_inner();
    let project_id = super::project_id(&req)?;

    form.validate().map_err(|err| 
        super::ApiError::Validation(validation_errors_to_string(err, None)))?;
        
    let group_id = TaskGroupId(path.0.clone());

    let mut transaction = pool.begin().await?;

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    let group = TaskGroup::get(group_id.clone(), &mut transaction).await?
        .ok_or_else(|| super::ApiError::NotFound("Could not find task group to edit".to_string()))?;

    if let Some(name) = form.name {
        sqlx::query!(
            "
            UPDATE task_groups
            SET name = $1
            WHERE id = $2
            AND project_id = $3
            ",
            name,
            group_id,
            project_id
        )
        .execute(&mut transaction)
        .await?;

        Audit::create(
            &member, 
            format!("Changed task group name from '{}' to '{}'", group.name, name), 
            &mut transaction
        ).await?;
    }

    if let Some(position) = form.position {
        // Move existing groups out of the ways
        sqlx::query!(
            "
            UPDATE task_groups
            SET position = position + 1
            WHERE position >= $1
            AND project_id = $2
            ",
            position,
            project_id
        )
        .execute(&mut transaction)
        .await?;

        // Insert into now empty slot
        sqlx::query!(
            "
            UPDATE task_groups
            SET position = $1
            WHERE id = $2
            AND project_id = $3
            ",
            position,
            group_id,
            project_id
        )
        .execute(&mut transaction)
        .await?;

        Audit::create(
            &member,
            format!("Moved task group '{}'", group.name),
            &mut transaction
        ).await?;
    }

    transaction.commit().await?;

    response!(StatusCode::OK, "Successfully edited task group")
}

#[delete("/")]
pub async fn remove(
    path: web::Path<(String,)>,
    req: actix_web::HttpRequest,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;

    let group_id = TaskGroupId(path.0.clone());

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    let group = TaskGroup::get(group_id.clone(), &mut transaction).await?
        .ok_or_else(|| super::ApiError::NotFound("Could not find task group to remove".to_string()))?;
    
    TaskGroup::remove(group_id, &mut transaction).await?;
    Audit::create(&member, format!("Removed task group '{}'", group.name), &mut transaction).await?;

    transaction.commit().await?;

    response!(StatusCode::OK, "Successfully removed task group")
}

// Generic Task routes

#[post("/")]
pub async fn create_task(
    path: web::Path<(String,)>,
    req: actix_web::HttpRequest,
    pool: web::Data<SqlPool>,
    form: web::Json<TaskBuilder>,
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let form = form.into_inner();

    let project_id = super::project_id(&req)?;
    let task_group_id = TaskGroupId(path.0.clone());

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    form.validate().map_err(|err| 
            super::ApiError::Validation(validation_errors_to_string(err, None)))?;
    let task = form.create(project_id, task_group_id, &mut transaction).await?;
    

    response!(StatusCode::OK, task, "Successfully created task")
}

#[get("/")]
pub async fn get_tasks(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>,
) -> Result<HttpResponse, super::ApiError> {
    let group_id = TaskGroupId(path.0.clone());
    let tasks = TaskGroup::get_tasks_full(group_id, &**pool).await?;
    response!(StatusCode::OK, tasks, "Successfully retrieved tasks")
}