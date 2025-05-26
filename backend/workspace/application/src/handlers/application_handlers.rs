use actix_web::{web, HttpRequest, HttpMessage, Responder};
use shared::responses::errors::AppError;
use shared::models::user::Claims;
use shared::models::application::ApplicationRequest;
use shared::state::app_state::AppState;
use crate::services::application_services::apply_to_vacancy;

pub async fn apply_to_vacancy_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    payload: web::Json<ApplicationRequest>,
) -> Result<impl Responder, AppError> {
    // 1. Sacamos los claims del usuario autenticado
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("No se encontraron los claims del usuario".to_string()))?;

    // 2. Verificamos que tenga el rol correcto
    if claims.role != "postulante" {
        return Err(AppError::Unauthorized("Solo los postulantes pueden aplicar a vacantes".to_string()));
    }

    // 3. Construimos el struct con el user_id desde claims
    let application = ApplicationRequest {
        user_id: claims.user_id.to_string(),
        vacancy_id: payload.vacancy_id.clone(),
    };

    // 4. Conectamos a la base de datos
    let client = app_state.pool.get().await
        .map_err(|e| AppError::InternalServerError(format!("Error obteniendo conexión de BD: {}", e)))?;

    // 5. Ejecutamos la lógica del servicio
    apply_to_vacancy(&client, application).await?;

    // 6. Respondemos éxito
    Ok(web::Json(serde_json::json!({"message": "Postulación enviada correctamente"})))
}