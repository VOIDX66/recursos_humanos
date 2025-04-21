use actix_web::{web, Responder, Result};
use crate::services::user_services;
use crate::models::user::NewUser;
use crate::state::app_state::AppState;
use crate::responses::errors::AppError;

pub async fn create_user_handler(
    new_user: web::Json<NewUser>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    // Obtener la conexión del pool de manera segura
    let conn = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to get database connection: {}", e))
    })?;

    // Validación de los campos antes de procesar la deserialización (opcional, si es necesario)
    if new_user.name.trim().is_empty()
        || new_user.lastname.trim().is_empty()
        || new_user.email.trim().is_empty()
        || new_user.password.trim().is_empty()
        || new_user.rol.trim().is_empty()
        || new_user.id_number.trim().is_empty()
    {
        return Err(AppError::ValidationError("All fields are required".to_string()).into());
    }

    // Llamar al servicio para crear el usuario y manejar el error
    match user_services::create_user(&conn, new_user.into_inner()).await {
        Ok(user) => Ok(web::Json(user)),
        Err(e) => Err(AppError::UserCreationError(format!("Error creating user: {}", e)).into()),
    }
}
