use actix_web::{web, HttpRequest, HttpMessage, Responder};
use shared::responses::errors::AppError;
use shared::models::user::Claims;
use shared::state::app_state::AppState;
use crate::services::evaluation_services::get_evaluations_for_evaluator;

pub async fn get_evaluations_for_evaluator_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    // 1. Extraer claims
    let claims = req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("No se encontraron los claims del usuario".to_string()))?;

    // 2. Verificar que el rol sea evaluador
    if claims.role != "evaluador" {
        return Err(AppError::Unauthorized("Solo los evaluadores pueden ver sus evaluaciones".to_string()));
    }

    // 3. Conexión a la BD
    let client = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Error obteniendo conexión a la base de datos: {}", e))
    })?;

    // 4. Obtener evaluaciones
    let evaluations = get_evaluations_for_evaluator(&client, &claims.user_id).await?;

    // 5. Responder
    Ok(web::Json(evaluations))
}