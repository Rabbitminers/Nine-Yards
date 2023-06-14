use actix_web::{HttpResponse, body::BoxBody};

use crate::models::error::ResponseBody;

pub mod user_routes;
pub mod project_routes;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/api")
            .configure(user_routes::config)
            .configure(project_routes::config)
    );
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] crate::models::DatabaseError),

    #[error("Database error: {0}")]
    SqlxDatabase(#[from] sqlx::Error),

    #[error("Json model error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid Input: {0}")]
    InvalidInput(String),

    #[error("Error while validating input: {0}")]
    Validation(String),
}

impl actix_web::ResponseError for ApiError {
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
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(ResponseBody::new(&self.to_string(), String::new()))
    }
}