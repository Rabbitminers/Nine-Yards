use actix_web::{web, HttpResponse, get, post, delete};

use crate::database::SqlPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
    web::scope("/tasks")
                .service(
                    web::scope("/{task_id}")
                        .service(get)   
                        .service(create)
                        .service(edit)
                        .service(delete)
                )
        );
}

#[get("/")]
pub async fn get(
    task_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    unimplemented!()
}

#[post("/create")]
pub async fn create(
    task_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    unimplemented!()
}

pub struct EditTask {
    
}

#[post("/edit")]
pub async fn edit(
    task_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    unimplemented!()
}

#[delete("/")]
pub async fn delete(
    task_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    unimplemented!()
}