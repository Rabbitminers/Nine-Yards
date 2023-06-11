use crate::{models::{users::{
    Register, 
    User, Login
}, ids::UserId, user_token::UserToken, error::ResponseBody}, response, constants};
use actix_web::{
    web,
    HttpResponse, 
    Result, post, HttpRequest, get, http::StatusCode
};
use log::info;
use sqlx::SqlitePool;
use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
    web::scope("/auth")
                .service(logout)
                .service(login)
                .service(register)
        )
        .service( 
    web::scope("/user")
                .service(get)
        );
}

#[get("/get/{id}")]
pub async fn get(
    user_id: web::Path<(String,)>,
    pool: web::Data<SqlitePool>
) -> Result<HttpResponse> {
    let user_id = UserId(user_id.into_inner().0);
    match User::find_by_id(user_id, &pool).await {
        Ok(Some(user)) => {
            info!("Found user: {:?}", user.username);
            response!(StatusCode::OK, user, constants::MESSAGE_FIND_USER_SUCCESS)
        },
        Ok(None) => {
            info!("Could not find user");
            response!(StatusCode::NOT_FOUND, constants::MESSAGE_FIND_USER_FAIL)
        }
        Err(e) => {
            info!("Error finding user: {:?}", e);
            response!(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_FIND_USER_FAIL)
        }
    }
}

#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    pool: web::Data<SqlitePool>
) -> Result<HttpResponse> {
    let user = match User::from_request(req, &pool).await {
        Ok(Some(user)) => user,
        _ => {
            info!("Invalid logout attempt");
            return response!(StatusCode::UNAUTHORIZED, constants::MESSAGE_LOGOUT_FAIL);
        }
    };

    match user.logout(&pool).await {
        Ok(_) => {
            info!("Logged out user: {:?}", user.username);
            response!(StatusCode::OK, constants::MESSAGE_LOGOUT_SUCCESS)
        },
        Err(e) => {
            info!("Error logging out user: {:?}", e);
            response!(StatusCode::BAD_REQUEST, constants::MESSAGE_LOGOUT_FAIL)
        }
    }
}

#[post("/login")]
pub async fn login(
    form: web::Json<Login>, 
    pool: web::Data<SqlitePool>
) -> Result<HttpResponse> {
    match User::login(form.into_inner(), &pool).await {
        Ok(Some(session)) => {
            info!("Logged in user: {:?}", session.username);
            let token = UserToken::generate_token(&session);
            response!(StatusCode::OK, token, constants::MESSAGE_LOGIN_SUCCESS)
        }
        Ok(None) => {
            info!("Invalid login attempt");
            response!(StatusCode::UNAUTHORIZED, constants::MESSAGE_LOGIN_FAIL)
        }
        Err(e) => {
            info!("Error logging in user: {:?}", e);
            response!(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_LOGIN_FAIL)
        }
    }
}

#[post("/register")]
pub async fn register(
    form: web::Json<Register>, 
    pool: web::Data<SqlitePool>
) -> Result<HttpResponse> {
    let form = form.into_inner();

    if let Err(e) = form.validate() {
        info!("Invalid registry: {:?}, error: {:?}", form, e);
        return response!(StatusCode::BAD_REQUEST, "Invalid sign up form");
    };
    
    match User::register(form, &pool).await {
        Ok(user) => {
            info!("Registered new user: {:?}", user.username);
            response!(StatusCode::OK, user, constants::MESSAGE_CREATE_USER_SUCCESS)
        }
        Err(e) => {
            info!("Error creating new user: {:?}", e);
            response!(StatusCode::BAD_REQUEST, constants::MESSAGE_CREATE_USER_FAIL)
        }
    }
}
