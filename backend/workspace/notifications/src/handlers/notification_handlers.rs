use actix_web::{web, HttpRequest, HttpMessage, Responder};
use shared::responses::errors::AppError;
use shared::models::user::Claims;
use shared::state::app_state::AppState;
use crate::services::notification_services::mark_notification_as_read;

pub async fn mark_notification_as_read_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    path: web::Path<String>,  // Aquí recibimos el id de la notificación como String
) -> Result<impl Responder, AppError> {
    // 1. Extraer claims (para validar que haya usuario autenticado)
    let _ = req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("No se encontraron los claims del usuario".to_string()))?;

    // 2. Obtener el id de la notificación del path
    let notification_id = path.into_inner();

    // 3. Obtener conexión a la base de datos
    let client = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Error obteniendo conexión a la base de datos: {}", e))
    })?;

    // 4. Llamar al servicio para marcar como leída
    mark_notification_as_read(&client, &notification_id).await?;

    // 5. Responder con éxito simple
    Ok(web::Json(serde_json::json!({"message": "Notificación marcada como leída"})))
}

use crate::services::notification_services::get_notifications_for_user;

pub async fn get_notifications_for_user_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    // 1. Extraer claims del usuario autenticado
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("No se encontraron los claims del usuario".into()))?;

    // 2. Obtener conexión a la base de datos
    let client = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Error obteniendo conexión a la base de datos: {}", e))
    })?;

    // 3. Consultar notificaciones
    let notifications = get_notifications_for_user(&client, &claims.user_id).await?;

    // 4. Responder con JSON
    Ok(web::Json(notifications))
}
