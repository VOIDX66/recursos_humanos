// modules/applications/services.rs
use deadpool_postgres::Client;
use shared::responses::errors::AppError;
use shared::models::application::ApplicationRequest;

pub async fn apply_to_vacancy(
    client: &Client,
    data: ApplicationRequest,
) -> Result<(), AppError> {
    // 1. Verificamos que la vacante exista y esté abierta
    let stmt = client
        .prepare("SELECT status FROM vacancies WHERE id = $1")
        .await
        .map_err(|e| AppError::DatabaseError(format!("Error preparando consulta de vacante: {}", e)))?;

    let row = client
        .query_opt(&stmt, &[&data.vacancy_id])
        .await
        .map_err(|e| AppError::DatabaseError(format!("Error ejecutando consulta de vacante: {}", e)))?;

    let Some(row) = row else {
        return Err(AppError::NotFoundError("Vacante no encontrada".to_string()));
    };

    let status: String = row.get("status");
    if status.to_lowercase() != "open" {
        return Err(AppError::DatabaseError("La vacante no está abierta".into()));
    }

    // 2. Insertamos la aplicación con status "pending" y sin comentario
    let insert_stmt = client
        .prepare(
            "INSERT INTO applications (user_id, vacancy_id)
             VALUES ($1, $2)",
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Error preparando inserción de aplicación: {}", e)))?;

    client
        .execute(&insert_stmt, &[&data.user_id, &data.vacancy_id])
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_db_error() {
                if db_err.constraint() == Some("unique_application") {
                    return AppError::DatabaseError("Ya estás postulado a esta vacante".into());
                }
            }
            AppError::DatabaseError(format!("Error al insertar la aplicación: {}", e))
        })?;

    Ok(())
}
