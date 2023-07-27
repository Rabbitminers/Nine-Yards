use poem_openapi::ApiResponse;
use poem_openapi::payload::Json;
use poem_openapi::types::ToJSON;

use crate::database::{SqlPool, Database};

pub mod tasks;
pub mod users;

#[derive(ApiResponse)]
pub enum FetchResponse<T: Send + Sync + ToJSON> {
    /// The data was succesfully fetched and returned in a json body
    #[oai(status = 200)]
    Ok(Json<T>),

    /// The project member or user could not be found
    #[oai(status = 401)]
    Unauthorized,

    /// The project member was found but does not have the required permissions 
    /// in the project to access this data or is not in the project
    #[oai(status = 403)]
    Forbidden,

    /// The data being fetched could not be found or does not exist
    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum CreateResponse<T: Send + Sync + ToJSON> {
    /// The data was succesfully created and it's id is returned in a
    /// json body
    #[oai(status = 200)]
    Ok(Json<T>),

    /// The project member or user could not be found
    #[oai(status = 401)]
    Unauthorized,

    /// The project member was found but does not have the required permissions 
    /// in the project to access this data or is not in the project
    #[oai(status = 403)]
    Forbidden,

    /// The data could not be written as it already exists or another entry
    /// with the same name already exists
    #[oai(status = 409)]
    AlreadyExists,
}

#[derive(ApiResponse)]
pub enum UpdateResponse {
    /// The data was succesfully updated
    #[oai(status = 200)]
    Ok,

    /// The project member or user could not be found
    #[oai(status = 401)]
    Unauthorized,

    /// The project member was found but does not have the required permissions 
    /// in the project to access this data or is not in the project
    #[oai(status = 403)]
    Forbidden,
}


#[derive(ApiResponse)]
pub enum DeleteResponse {
    /// The data was succesfully deleted
    #[oai(status = 200)]
    Ok,

    /// The project member or user could not be found
    #[oai(status = 401)]
    Unauthorized,

    /// The project member was found but does not have the required permissions 
    /// in the project to access this data or is not in the project
    #[oai(status = 403)]
    Forbidden,

    /// The data could not be deleted as it doesnt exist or could not be found
    #[oai(status = 404)]
    NotFound,
}

pub async fn begin_transaction(
    pool: &SqlPool
) -> poem::Result<sqlx::Transaction<'_, Database>>  {
    pool.begin()
        .await
        .map_err(poem::error::InternalServerError)
}