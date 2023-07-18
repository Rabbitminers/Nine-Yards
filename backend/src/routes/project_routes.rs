use crate::database::SqlPool;
use crate::middleware::auth::Authenticator;
use crate::middleware::project::ProjectAuthentication;
use crate::models::audit::Audit;
use crate::models::projects::{ProjectBuilder, Project, ProjectMember, Permissions};
use crate::models::tasks::TaskGroupBuilder;
use crate::models::users::User;
use crate::models::ids::ProjectId;
use crate::utilities::auth_utils::AuthenticationError::{NotMember, MissingPermissions};
use crate::utilities::validation_utils::validation_errors_to_string;
use crate::response;
use actix_web::{get, delete};
use actix_web::{web, HttpResponse, post, http::StatusCode, HttpRequest};
use log::info;
use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
    web::scope("/project")
                .service(web::scope("/create") // Scope to apply middleware
                    .wrap(Authenticator)
                    .service(create)
                )
                .service(web::scope("/{project_id}")
                    .wrap(ProjectAuthentication {
                        id_key: "project_id".to_string(),
                        table_name: None
                    })
                    .service(get)
                    .service(get_members)
                    .service(web::scope("/task-groups")
                        .service(create_task_group)
                        .service(get_task_groups)
                    )
                )
        );
}


#[get("/")]
pub async fn get(
    project_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {    
    let project_id = ProjectId(project_id.into_inner().0);
    let project = Project::get(project_id.clone(), &**pool).await?;

    if let Some(project) = project {
        info!("Successfully retrieved project: {:?}", project.name);
        response!(StatusCode::OK, project, "Successfully retrieved project")
    } else {
        info!("Project not found: {:?}", project_id.0);
        Err(super::ApiError::NotFound("Could not find project".to_string()))
    }
}

#[get("/members")]
pub async fn get_members(
    project_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let id = ProjectId(project_id.into_inner().0);

    let members= Project::get_members(id, &**pool).await?;

    response!(StatusCode::OK, members, "Successfully retrieved members")
}

#[get("/audit-log")]
pub async fn get_audit_log(
    project_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let id = ProjectId(project_id.into_inner().0);

    let members= Audit::get_many(id, &**pool).await?;

    response!(StatusCode::OK, members, "Successfully collected audit log")
}

#[post("/")]
pub async fn create(
    req: HttpRequest,
    form: web::Json<ProjectBuilder>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let form = form.into_inner();
    
    form.validate().map_err(|err| 
            super::ApiError::Validation(validation_errors_to_string(err, None)))?;
        
    let user = User::from_request(req, &mut transaction).await?;
    let project = form.create(user.id, &mut transaction).await?;

    transaction.commit().await?;
    response!(StatusCode::OK, project, "Successfully created project")
}

#[delete("/")]
pub async fn remove(
    req: HttpRequest,
    project_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let id = ProjectId(project_id.into_inner().0);

    let member = ProjectMember::from_request(req, &mut transaction)
        .await?
        .ok_or_else(|| super::ApiError::Unauthorized(NotMember))?;

    if !member.permissions.contains(Permissions::MANAGE_TASKS) {
        return Err(super::ApiError::Unauthorized(MissingPermissions));
    }

    Project::remove(id, &mut transaction).await?;
    transaction.commit().await?;

    response!(StatusCode::OK, "Successfully removed project")
}

// Generic task group routes

#[get("/")]
pub async fn get_task_groups(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let project_id = ProjectId(path.0.clone());

    let groups = Project::get_task_groups(project_id, &**pool).await?;
    
    response!(StatusCode::OK, groups, "Successfully retrieved task groups")
}

#[post("/")]
pub async fn create_task_group(
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

// Generic task routes

#[get("/")]
pub async fn get_tasks(
    path: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let project_id = ProjectId(path.0.clone());

    let groups = Project::get_tasks(project_id, &**pool).await?;

    response!(StatusCode::OK, groups, "Successfully retrieved task groups")
}