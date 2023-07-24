use actix_web::{HttpResponse, body::BoxBody, HttpRequest};

use crate::models::ids::ProjectId;

pub mod user_routes;
pub mod project_routes;
pub mod task_routes;
pub mod task_group_routes;
pub mod sub_task_routes;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api")
            .configure(user_routes::config)
            .configure(project_routes::config)
            .configure(task_group_routes::config)
            .configure(task_routes::config)
            .configure(sub_task_routes::config)
    );
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] crate::models::DatabaseError),

    #[error("Database error: {0}")]
    SqlxDatabase(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Json model error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid Input: {0}")]
    InvalidInput(String),

    #[error("Error while validating input: {0}")]
    Validation(String),

    #[error("Could not find: {0}")]
    NotFound(String),

    #[error("Authentication Error: {0}")]
    Unauthorized(#[from] crate::utilities::auth_utils::AuthenticationError),
}

impl actix_web::error::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Database(_) => 
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SqlxDatabase(_) => 
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Json(_) => 
            actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::InvalidInput(..) => 
            actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::Validation(..) => 
            actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::NotFound(..) =>
            actix_web::http::StatusCode::NOT_FOUND,
            ApiError::Unauthorized(..) =>
            actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError(..) =>
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(ResponseBody::new(&self.to_string(), String::new()))
    }
}

#[macro_export]
macro_rules! response {
    ($status:expr, $data:expr, $fmt:expr $(, $arg:expr)*) => {
        Ok(actix_web::HttpResponse::build($status)
            .json(crate::routes::ResponseBody::new($fmt $(, $arg)*, $data)))
    };

    ($status:expr, $fmt:expr $(, $arg:expr)*) => {
        Ok(actix_web::HttpResponse::build($status)
            .json(crate::routes::ResponseBody::new($fmt $(, $arg)*, String::new())))
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}

pub fn project_id(req: &HttpRequest) -> Result<ProjectId, ApiError> {
    ProjectId::from_request(req).ok_or(ApiError::NotFound("Project not found".to_string()))
}
