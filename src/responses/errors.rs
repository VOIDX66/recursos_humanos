use actix_web::{HttpResponse, ResponseError};
//use diesel::dsl::Update;
use std::error::Error as StdError;
use std::fmt;
use tokio_postgres::Error as PgError;
use serde_json::json;  // Necesario para poder serializar a JSON

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    UserCreationError(String),
    UserNotFoundError(String),
    ValidationError(String),
    InternalServerError(String),
    AuthenticationError(String),
    NotFoundError(String),
    Unauthorized(String),
    UpdateError(String)
    // Puedes añadir más tipos de errores según necesites
}

// Implementación de la conversión del error a string para el `fmt::Display`
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::UserCreationError(msg) => write!(f, "User creation error: {}", msg),
            AppError::UserNotFoundError(msg) => write!(f, "User not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Server error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
            AppError::NotFoundError(msg) => write!(f, "Not Found Error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::UpdateError(msg) => write!(f, "UpdateError: {}", msg)

        }
    }
}

// Implementación de StdError
impl StdError for AppError {}

// Implementación para convertir AppError a PgError
impl From<AppError> for PgError {
    fn from(_: AppError) -> Self {
        tokio_postgres::Error::__private_api_timeout()
    }
}

// También podemos implementar la conversión inversa
impl From<PgError> for AppError {
    fn from(error: PgError) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}

// Implementación del trait ResponseError para convertir AppError en un Error de Actix Web
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        // Devolver un JSON con estructura más clara en lugar de solo texto plano
        match self {
            AppError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "DatabaseError",
                    "message": msg
                }))
            }
            AppError::UserCreationError(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "UserCreationError",
                    "message": msg
                }))
            }
            AppError::UserNotFoundError(msg) => {
                HttpResponse::NotFound().json(json!({
                    "error": "UserNotFoundError",
                    "message": msg
                }))
            }
            AppError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "ValidationError",
                    "message": msg
                }))
            }
            AppError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "InternalServerError",
                    "message": msg
                }))
            }
            AppError::AuthenticationError(msg) => {
                // Aquí devolvemos el error de autenticación con un HTTP 401 Unauthorized
                HttpResponse::Unauthorized().json(json!({
                    "error": "AuthenticationError",
                    "message": msg
                }))
            }
            AppError::NotFoundError(msg) => {
                HttpResponse::NotFound().json(json!({
                    "error": "NotFoundError",
                    "message": msg
                }))
            }
            AppError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Unauthorized",
                    "message": msg
                }))
            }
            AppError::UpdateError(msg) => {
                HttpResponse::Conflict().json(json!({
                    "error": "UpdateError",
                    "message": msg
                }))
            } 
                
        }
    }
}
