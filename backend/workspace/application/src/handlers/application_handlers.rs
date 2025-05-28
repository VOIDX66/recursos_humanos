use actix_web::{web, HttpRequest, HttpMessage, Responder};
use shared::responses::errors::AppError;
use shared::models::user::Claims;
use shared::models::application::{ ApplicationRequest, DecideApplicationRequest };
use shared::state::app_state::AppState;
use crate::services::application_services::{ apply_to_vacancy, get_applications_for_evaluator, decide_application_and_schedule_evaluation };

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

pub async fn get_applications_for_evaluator_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    // 1. Sacamos los claims del evaluador autenticado
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("No se encontraron los claims del usuario".to_string()))?;

    // 2. Verificamos que tenga el rol correcto
    if claims.role != "evaluador" {
        return Err(AppError::Unauthorized("Solo los evaluadores pueden ver estas postulaciones".to_string()));
    }

    // 3. Obtenemos conexión a la BD
    let client = app_state.pool.get().await
        .map_err(|e| AppError::InternalServerError(format!("Error obteniendo conexión de BD: {}", e)))?;

    // 4. Llamamos al servicio que trae las aplicaciones para el evaluador
    let applications = get_applications_for_evaluator(&client, &claims.user_id).await?;

    // 5. Respondemos con las aplicaciones
    Ok(web::Json(applications))
}

pub async fn decide_application_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    payload: web::Json<DecideApplicationRequest>,
) -> Result<impl Responder, AppError> {
    // 1. Sacamos los claims del evaluador autenticado
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("No se encontraron los claims del usuario".to_string()))?;

    // 2. Verificamos que tenga el rol correcto
    if claims.role != "evaluador" {
        return Err(AppError::Unauthorized("Solo los evaluadores pueden tomar decisiones sobre postulaciones".to_string()));
    }

    // 3. Obtenemos conexión a la BD
    let client = app_state.pool.get().await
        .map_err(|e| AppError::InternalServerError(format!("Error obteniendo conexión de BD: {}", e)))?;

    // 4. Ejecutamos la lógica del servicio
    decide_application_and_schedule_evaluation(&client, payload.into_inner(), &claims.user_id).await?;

    // 5. Respondemos éxito
    Ok(web::Json(serde_json::json!({"message": "Decisión registrada correctamente"})))
}