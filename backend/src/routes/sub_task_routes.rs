use actix_web::{web, HttpResponse, http::StatusCode, post, delete, get};

use crate::{middleware::project, database::SqlPool, models::{tasks::{SubTask, Task}, ids::{SubTaskId, TaskId, ProjectMemberId}, projects::{ProjectMember, Permissions}, audit::Audit}, routes::ApiError, response, utilities::auth_utils::AuthenticationError::{NotMember, MissingPermissions}};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
web::scope("/sub-tasks/{sub_task_id}")
            .wrap(project::ProjectAuthentication {
                id_key: "sub_task_id".to_string(),
                table_name: Some("tasks".to_string())
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
) -> Result<HttpResponse, super::ApiError> {
    let sub_task_id = SubTaskId(path.0.clone());

    let task = SubTask::get(sub_task_id, &**pool)
        .await?
        .ok_or_else(|| ApiError::NotFound("Could not find sub task".to_string()))?;

    response!(StatusCode::OK, task, "Successfully found task")
}

#[derive(Deserialize, Validate)]
pub struct EditSubTask {
    pub assignee: Option<String>,
    #[validate(length(min = 3, max = 90))]
    pub body: Option<String>,
    pub position: Option<i64>,
    pub completed: Option<bool>,
}

#[post("/edit")]
pub async fn edit(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>,
    req: actix_web::HttpRequest,
    form: web::Json<EditSubTask>,
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let form = form.into_inner();

    let sub_task_id = SubTaskId(path.0.clone());

    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    let sub_task = SubTask::get(sub_task_id.clone(), &mut transaction).await?
        .ok_or_else(|| super::ApiError::NotFound("Could not find sub task to edit".to_string()))?;

    if let Some(assignee) = form.assignee {
        let member_id = ProjectMemberId(assignee);
        let assignee = ProjectMember::get(member_id, &mut transaction).await?
            .ok_or_else(|| super::ApiError::NotFound("Could not find assignee".to_string()))?;

        sqlx::query!(
            "
            UPDATE sub_tasks
            SET assignee = $1
            WHERE id = $2
            ",
            assignee.id,
            sub_task_id
        )
        .execute(&mut transaction)
        .await?;
    }

    if let Some(body) = form.body {
        sqlx::query!(
            "
            UPDATE sub_tasks
            SET body = $1
            WHERE id = $2
            ",
            body,
            sub_task_id
        )
        .execute(&mut transaction)
        .await?;
    }

    if let Some(position) = form.position {
        sqlx::query!(
            "
            UPDATE sub_tasks
            SET position = position + 1
            WHERE position >= $1
            AND task_id = $2
            ",
            position,
            sub_task.task_id
        )
        .execute(&mut transaction)
        .await?;

        sqlx::query!(
            "
            UPDATE sub_tasks
            SET position = $1
            WHERE id = $2
            ",
            position,
            sub_task_id
        )
        .execute(&mut transaction)
        .await?;
    }

    if let Some(completed) = form.completed {
        sqlx::query!(
            "
            UPDATE sub_tasks
            SET completed = $1
            WHERE id = $2
            ",
            completed,
            sub_task_id
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;
    response!(StatusCode::OK, "Successfully edited sub task")
}

#[delete("/")]
pub async fn delete(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let sub_task_id = SubTaskId(path.0.clone());


    let member = ProjectMember::from_request(req, &mut transaction).await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    let sub_task = SubTask::get(sub_task_id.clone(), &mut transaction).await?
        .ok_or_else(|| super::ApiError::NotFound("Could not find sub task to remove".to_string()))?;

    sub_task.remove(&mut transaction).await?;
    Audit::create(&member, format!("Deleted sub task '{}'", sub_task.body), &mut transaction).await?;

    transaction.commit().await?;

    response!(StatusCode::OK, "Successfully deleted task")
}


