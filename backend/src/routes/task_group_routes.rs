use actix_web::{web, HttpResponse, get, http::StatusCode, post, delete};
use log::info;
use validator::Validate;

use crate::{database::SqlPool, models::{ids::{TaskGroupId, ProjectId}, tasks::{TaskGroup, TaskGroupBuilder}, projects::{Project, ProjectMember, Permissions}, audit::Audit}, response, utilities::{validation_utils::validation_errors_to_string, auth_utils::AuthenticationError::{NotMember, Unauthorized, MissingPermissions}}};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
    web::scope("/task-groups")
                .service(get_many)
                .service(create)
                .service(
                    web::scope("/{task_group_id}")
                        .service(get)
                        .service(get_tasks)
                        .service(edit)
                        .service(remove)
                )
        );
}

#[get("/")]
pub async fn get_many(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let project_id = ProjectId(path.0.clone());

    let mut transaction = pool.begin().await?;

    let groups = Project::get_task_groups(project_id, &mut transaction).await?;

    transaction.commit().await?;
    
    response!(StatusCode::OK, groups, "Successfully retrieved task groups")
}

#[post("/create")]
pub async fn create(
    form: web::Json<TaskGroupBuilder>,
    path: web::Path<(String,)>,
    req: actix_web::HttpRequest,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let form = form.into_inner();

    let project_id = ProjectId(path.0.clone());

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    form.validate().map_err(|err| 
            super::ApiError::Validation(validation_errors_to_string(err, None)))?;

    let task_group = form.create(project_id, &mut transaction).await?;

    Audit::create(&member, format!("Created new task group '{}'", task_group.name), &mut transaction).await?;
    transaction.commit().await?;

    response!(StatusCode::CREATED, task_group, "Successfully created task group")
}

#[get("/")]
pub async fn get(
    path: web::Path<(String, String)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let project_id = ProjectId(path.0.clone());
    let group_id = TaskGroupId(path.1.clone());

    let mut transaction = pool.begin().await?;

    if let Some(group) = TaskGroup::get(group_id.clone(), project_id, &mut transaction).await? {        
        transaction.commit().await?;

        info!("Retrieved task group: {}", group.id.0);
        response!(StatusCode::OK, group, "Successfully retrieved task group")
    } else {
        info!("Failed to retrieve task group: {}", group_id.0);
        Err(super::ApiError::NotFound("Could not find task group".to_string()))
    }
}

#[get("/tasks")]
pub async fn get_tasks(
    path: web::Path<(String, String)>,
    pool: web::Data<SqlPool>,
) -> Result<HttpResponse, super::ApiError> {
    let group_id = TaskGroupId(path.1.clone());
    let project_id = ProjectId(path.0.clone());
    
    let tasks = TaskGroup::get_tasks_full(project_id,group_id, &pool).await?;

    info!("Retrieved {} tasks", tasks.len());
    response!(StatusCode::OK, tasks, "Successfully retrieved tasks")
}


#[derive(Deserialize, Validate)]
pub struct TaskGroupEdit {
    #[validate(length(min = 3, max = 30))]
    pub name: Option<String>,
    pub position: Option<i64>
}

#[post("/edit")]
pub async fn edit(
    path: web::Path<(String, String)>,
    pool: web::Data<SqlPool>,
    form: web::Json<TaskGroupEdit>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, super::ApiError> {
    let form = form.into_inner();

    form.validate().map_err(|err| 
        super::ApiError::Validation(validation_errors_to_string(err, None)))?;

    let project_id = ProjectId(path.0.clone());
    let group_id = TaskGroupId(path.1.clone());

    let mut transaction = pool.begin().await?;

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    let group = TaskGroup::get(group_id.clone(), project_id.clone(), &mut transaction).await?
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
    path: web::Path<(String, String)>,
    req: actix_web::HttpRequest,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;

    let project_id = ProjectId(path.0.clone());
    let group_id = TaskGroupId(path.1.clone());

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    let group = TaskGroup::get(group_id.clone(), project_id.clone(), &mut transaction).await?
        .ok_or_else(|| super::ApiError::NotFound("Could not find task group to remove".to_string()))?;
    
    TaskGroup::remove(project_id, group_id, &mut transaction).await?;
    Audit::create(&member, format!("Removed task group '{}'", group.name), &mut transaction).await?;

    transaction.commit().await?;

    response!(StatusCode::OK, "Successfully removed task group")
}