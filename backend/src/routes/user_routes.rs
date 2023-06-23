use crate::models::users::{Register, User, Login};
use crate::models::ids::UserId;
use crate::response;
use crate::constants;
use crate::database::SqlPool;
use crate::utilities::validation_utils::validation_errors_to_string; 
use crate::utilities::auth_utils::AuthenticationError;

use super::ApiError;

use actix_web::{web, HttpResponse, post, HttpRequest, get};
use actix_web::http::StatusCode;

use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
    web::scope("/account")
                .wrap(crate::middleware::auth::Authenticator)
                .service(logout)
                .service(login)
                .service(register)
        )
        .service( 
    web::scope("/user/{id}")
                .service(get)
        );
}

#[get("/")]
pub async fn get(
    user_id: web::Path<(String,)>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let mut transaction = pool.begin().await?;
    let user_id = UserId(user_id.into_inner().0);

    let user = User::find_by_id(user_id, &mut transaction).await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    response!(StatusCode::OK, user, "Successfully found user")
}

#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError>{
    let mut transaction = pool.begin().await?;
    let user = User::from_request(req, &mut transaction).await?;

    user.logout(&mut transaction).await?;
    transaction.commit().await?;

    response!(StatusCode::OK, constants::MESSAGE_LOGOUT_SUCCESS)
}

#[post("/login")]
pub async fn login(
    form: web::Json<Login>,
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let form = form.into_inner();

    let mut transaction = pool.begin().await?;

    let session = form.login(&mut transaction).await?;
    transaction.commit().await?;

    response!(StatusCode::OK, session, constants::MESSAGE_LOGIN_SUCCESS)
}

#[post("/register")]
pub async fn register(
    req: HttpRequest, 
    form: web::Json<Register>, 
    pool: web::Data<SqlPool>
) -> Result<HttpResponse, super::ApiError> {
    let form = form.into_inner();

    let mut transaction = pool.begin().await?;

    form.validate().map_err(|err| 
        super::ApiError::Validation(validation_errors_to_string(err, None)))?;

    if User::from_request(req, &mut transaction).await.is_ok() {
        return Err(AuthenticationError::AlreadyLoggedIn.into())
    }

    let user = form.register(&mut transaction).await?;
    transaction.commit().await?;

    response!(StatusCode::OK, user.id, "Successfully registered user")
}



