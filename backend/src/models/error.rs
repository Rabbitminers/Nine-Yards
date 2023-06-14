use actix_web::{
    http::StatusCode, 
    HttpResponse
};

#[derive(Debug)]
pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

#[macro_export]
macro_rules! service_error {
    ($status:expr, $fmt:expr $(, $arg:expr)*) => {
        ServiceError::new($status, format!($fmt $(, $arg)*))
    };
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.body.data)
    }
}

#[macro_export]
macro_rules! response {
    ($status:expr, $data:expr, $fmt:expr $(, $arg:expr)*) => {
        Ok(HttpResponse::build($status).json(ResponseBody::new($fmt $(, $arg)*, $data)))
    };

    ($status:expr, $fmt:expr $(, $arg:expr)*) => {
        Ok(HttpResponse::build($status).json(ResponseBody::new($fmt $(, $arg)*, String::new())))
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

impl From<sqlx::error::Error> for ServiceError {
    fn from(error: sqlx::error::Error ) -> ServiceError { 
        Self::new(
    StatusCode::INTERNAL_SERVER_ERROR, 
    format!("Error while interacting with database: {}", error)
        )   
    }
}

impl From<super::DatabaseError> for ServiceError {
    fn from(error: super::DatabaseError) -> ServiceError {
        Self::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("{}", error)
        )
    }
}