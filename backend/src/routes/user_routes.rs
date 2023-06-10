use crate::models::users::{
    Register, 
    User
};
use actix_web::{
    web,
    HttpResponse, 
    Result, post
};
use log::info;
use sqlx::SqlitePool;
use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}

#[post("/auth/register")]
pub async fn register(
    form: web::Json<Register>, 
    pool: web::Data<SqlitePool>
) -> Result<HttpResponse> {
    let form = form.into_inner();

    if let Err(e) = form.validate() {
        info!("Invalid registry: {:?}, error: {:?}", form, e);
        return Ok(HttpResponse::BadRequest().body(e.to_string()));
    };
    
    match User::register(form, &pool).await {
        Ok(user) => {
            info!("Registered new user: {:?}", user.username);
            Ok(HttpResponse::Ok().json(user))
        }
        Err(e) => {
            info!("Error creating new user: {:?}", e);
            Ok(HttpResponse::BadRequest().body(e))
        }
    }
}
