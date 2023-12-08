
pub enum DatabaseError {
    DuplicateKey,
    InvalidInput,
    NotFound,
    CannotGetConnectionToDatabase,
}

pub fn database_error_duplicate_key<E>(_: E) -> DatabaseError where E: std::error::Error {
    DatabaseError::DuplicateKey
}


pub fn database_error_invalid_input<E>(_: E) -> DatabaseError where E: std::error::Error {
    DatabaseError::InvalidInput
}


pub fn database_error_not_found<E>(_: E) -> DatabaseError where E: std::error::Error {
    DatabaseError::NotFound
}

pub fn database_error_cannot_get_connection_to_database<E>(_: E) -> DatabaseError where E: std::error::Error {
    DatabaseError::CannotGetConnectionToDatabase
}