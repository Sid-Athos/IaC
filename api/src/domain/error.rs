use axum::http::StatusCode;
use crate::database::database_error::DatabaseError;

pub fn internal_error<E>(_: E) -> StatusCode where E: std::error::Error {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn database_error_to_status_code(err: DatabaseError) -> StatusCode {
    match err {
        DatabaseError::NotFound => StatusCode::NOT_FOUND,
        DatabaseError::DuplicateKey => StatusCode::CONFLICT,
        DatabaseError::InvalidInput => StatusCode::BAD_REQUEST,
        DatabaseError::CannotGetConnectionToDatabase => StatusCode::SERVICE_UNAVAILABLE,
    }
}