use crate::error::ApiError;

pub type Result<T, E = ApiError> = std::result::Result<T, E>;