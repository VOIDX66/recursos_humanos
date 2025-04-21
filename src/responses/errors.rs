// src/errors.rs

use std::error::Error as StdError;
use std::fmt;
use tokio_postgres::Error as PgError;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    UserCreationError(String),
    UserNotFoundError(String),
    ValidationError(String),
    // Puedes añadir más tipos de errores según necesites
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::UserCreationError(msg) => write!(f, "User creation error: {}", msg),
            AppError::UserNotFoundError(msg) => write!(f, "User not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl StdError for AppError {}

// Implementación para convertir AppError a PgError
impl From<AppError> for PgError {
    fn from(_: AppError) -> Self {
        // Usamos _ en lugar de "error" ya que no usamos esta variable
        tokio_postgres::Error::__private_api_timeout()
    }
}

// También podemos implementar la conversión inversa
impl From<PgError> for AppError {
    fn from(error: PgError) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}