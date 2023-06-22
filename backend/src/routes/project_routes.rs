use crate::database::SqlPool;
use crate::models::audit::Audit;
use crate::models::projects::{ProjectBuilder, Project};
use crate::models::users::User;
use crate::models::ids::ProjectId;
use crate::utilities::validation_utils::validation_errors_to_string;
use crate::response;

use actix_web::get;
use actix_web::{web, HttpResponse, post, http::StatusCode, HttpRequest};
use log::info;
use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
    web::scope("/project")
                .service(web::scope("/create") // Scope to apply middleware
                    .wrap(crate::middleware::auth::Authenticator)
                    .service(create)
                )
                .service(web::scope("/{project_id}")
                    .wrap(crate::middleware::project::ProjectAuthentication)
                    .service(get)
                    .service(get_members)
                    .configure(super::task_routes::config)
                    .configure(super::task_group_routes::config)
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