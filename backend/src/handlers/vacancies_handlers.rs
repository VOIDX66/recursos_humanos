use actix_web::{web, HttpRequest, HttpMessage, Responder, Result};
use crate::models::vacancy::NewVacancy;
use crate::responses::errors::AppError;
use crate::services::vacancies_services;
use crate::state::app_state::AppState;
use crate::models::user::Claims;

pub async fn create_vacancy_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    payload: web::Json<NewVacancy>,
) -> Result<impl Responder> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("User claims not found".to_string()))?;

    let conn = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("DB connection error: {}", e))
    })?;

    let created_by = claims.user_id;

    // Pasa created_by al servicio
    let vacancy = vacancies_services::create_vacancy(&conn, payload.into_inner(), &created_by)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Error creating vacancy: {}", e)))?;

    Ok(web::Json(vacancy))
}

pub async fn get_all_vacancies_handler(
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let client = app_state
        .pool
        .get()
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let vacancies = vacancies_services::get_all_vacancies(&client).await?;
    Ok(web::Json(vacancies))
}
