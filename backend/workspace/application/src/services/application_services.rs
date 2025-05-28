// modules/applications/services.rs
use chrono::{Utc, Duration};
use deadpool_postgres::Client;
use shared::responses::errors::AppError;
use shared::models::application::{ ApplicationRequest, ApplicationView, DecideApplicationRequest };

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

pub async fn get_applications_for_evaluator(
    client: &Client,
    evaluator_id: &str,
) -> Result<Vec<ApplicationView>, AppError> {
    let stmt = client.prepare("
        SELECT a.id, a.application_date, a.comment, a.status, u.name, u.lastname, u.email, v.title
        FROM applications a
        JOIN vacancies v ON a.vacancy_id = v.id
        JOIN users u ON a.user_id = u.user_id
        WHERE v.created_by = $1
    ").await.map_err(|e| {
        AppError::InternalServerError(format!("Error preparando la consulta: {}", e))
    })?;

    let rows = client.query(&stmt, &[&evaluator_id]).await.map_err(|e| {
        AppError::InternalServerError(format!("Error ejecutando la consulta: {}", e))
    })?;

    let applications = rows.into_iter().map(|row| ApplicationView {
        application_id: row.get("id"),
        applied_at: row.get("application_date"),
        application_status: row.get("status"),
        comment: row.get("comment"),
        applicant_name: format!("{} {}", row.get::<_, String>("name"), row.get::<_, String>("lastname")),
        applicant_email: row.get("email"),
        vacancy_title: row.get("title"),
    }).collect();

    Ok(applications)
}

pub async fn decide_application_and_schedule_evaluation(
    client: &Client,
    data: DecideApplicationRequest,
    evaluator_id: &str,
) -> Result<(), AppError> {
    if data.decision != "pending_evaluation" && data.decision != "rejected" {
        return Err(AppError::DatabaseError("Decision must be 'pending_evaluation' or 'rejected'".to_string()));
    }

    let query = "
        SELECT a.vacancy_id, a.user_id
        FROM applications a
        JOIN vacancies v ON a.vacancy_id = v.id
        WHERE a.id = $1 AND v.created_by = $2
    ";

    let stmt = client.prepare(query).await.map_err(|e| {
        AppError::DatabaseError(format!("Error preparando consulta de aplicación: {}", e))
    })?;

    let row = client.query_opt(&stmt, &[&data.application_id, &evaluator_id]).await.map_err(|e| {
        AppError::DatabaseError(format!("Error ejecutando consulta de aplicación: {}", e))
    })?.ok_or_else(|| AppError::NotFoundError("Aplicación no encontrada o sin autorización".to_string()))?;

    let vacancy_id: String = row.get("vacancy_id");
    let candidate_id: String = row.get("user_id");

    let update_stmt = client.prepare("UPDATE applications SET status = $1 WHERE id = $2").await.map_err(|e| {
        AppError::DatabaseError(format!("Error preparando actualización de aplicación: {}", e))
    })?;

    client.execute(&update_stmt, &[&data.decision, &data.application_id]).await.map_err(|e| {
        AppError::DatabaseError(format!("Error actualizando estado de aplicación: {}", e))
    })?;

    let (title, message): (String, String);

    if data.decision == "pending_evaluation" {
        let evaluation_date = Utc::now() + Duration::days(5);

        let insert_stmt = client.prepare("
            INSERT INTO evaluations (
                vacancy_id, candidate_id, evaluator_id, evaluation_date, status
            ) VALUES ($1, $2, $3, $4, 'pending')
        ").await.map_err(|e| {
            AppError::DatabaseError(format!("Error preparando inserción de evaluación: {}", e))
        })?;

        client.execute(&insert_stmt, &[
            &vacancy_id,
            &candidate_id,
            &evaluator_id,
            &evaluation_date,
        ]).await.map_err(|e| {
            AppError::DatabaseError(format!("Error insertando evaluación: {}", e))
        })?;

        title = "¡Has sido seleccionado para evaluación!".to_string();
        message = format!(
            "Tu aplicación ha sido preseleccionada. Tienes una evaluación programada para el día {}.",
            evaluation_date.format("%d/%m/%Y a las %H:%M")
        );
    } else {
        title = "Aplicación rechazada".to_string();
        message = "Gracias por postularte. En esta ocasión no fuiste seleccionado, pero sigue intentándolo.".to_string();
    }

    let insert_notif_stmt = client.prepare("
        INSERT INTO notifications (user_id, title, message)
        VALUES ($1, $2, $3)
    ").await.map_err(|e| {
        AppError::DatabaseError(format!("Error preparando notificación: {}", e))
    })?;

    client.execute(&insert_notif_stmt, &[&candidate_id, &title, &message]).await.map_err(|e| {
        AppError::DatabaseError(format!("Error insertando notificación: {}", e))
    })?;

    Ok(())
}
