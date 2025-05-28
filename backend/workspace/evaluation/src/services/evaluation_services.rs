use deadpool_postgres::Client;
use shared::responses::errors::AppError;
use shared::models::evaluations::{ EvaluationResponse, GradeEvaluationInput };

pub async fn get_evaluations_for_evaluator(
    client: &Client,
    evaluator_id: &str,
) -> Result<Vec<EvaluationResponse>, AppError> {
    let stmt = client.prepare("
        SELECT 
            e.id, 
            e.vacancy_id, 
            e.candidate_id, 
            u.id_number, 
            e.evaluation_date, 
            e.feedback, 
            e.score, 
            e.status
        FROM evaluations e
        JOIN users u ON u.user_id = e.candidate_id
        WHERE e.evaluator_id = $1
        ORDER BY e.evaluation_date DESC;
    ").await.map_err(|e| {
        AppError::DatabaseError(format!("Error preparando consulta de evaluaciones: {}", e))
    })?;

    let rows = client.query(&stmt, &[&evaluator_id]).await.map_err(|e| {
        AppError::DatabaseError(format!("Error obteniendo evaluaciones: {}", e))
    })?;

    let evaluations = rows.into_iter().map(|row| EvaluationResponse {
        id: row.get("id"),
        vacancy_id: row.get("vacancy_id"),
        candidate_id: row.get("candidate_id"),
        id_number : row.get("id_number"),
        evaluation_date: row.get::<_, chrono::DateTime<chrono::Utc>>("evaluation_date").to_rfc3339(),
        feedback: row.get("feedback"),
        score: row.get("score"),
        status: row.get("status"),
    }).collect();

    Ok(evaluations)
}

pub async fn grade_evaluation(
    client: &mut Client,
    input: GradeEvaluationInput,
) -> Result<(), AppError> {
    // 1. Iniciar transacción para garantizar consistencia
    let tx = client
        .transaction()
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Error iniciando transacción: {}", e))
        })?;

    // 2. Determinar estado basado en el puntaje (score)
    let evaluation_status = if input.score >= 70.0 {
        "approved"
    } else {
        "rejected"
    };

    // 3. Preparar la consulta para actualizar la evaluación
    let update_eval_stmt = tx
        .prepare(
            "UPDATE evaluations
             SET feedback = $1, score = $2, status = $3, updated_at = CURRENT_TIMESTAMP
             WHERE id = $4
             RETURNING candidate_id, vacancy_id",
        )
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Error preparando UPDATE de evaluación: {}", e))
        })?;

    // 4. Ejecutar la actualización de la evaluación
    let row = tx
        .query_opt(&update_eval_stmt, &[&input.feedback, &input.score, &evaluation_status, &input.evaluation_id])
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Error ejecutando UPDATE de evaluación: {}", e))
        })?
        .ok_or_else(|| {
            AppError::NotFoundError(format!("Evaluación con ID {} no encontrada", &input.evaluation_id))
        })?;

    // 5. Obtener los IDs necesarios para actualizar la aplicación
    let candidate_id: String = row.get("candidate_id");
    let vacancy_id: String = row.get("vacancy_id");

    // 6. Actualizar estado y comentario en la aplicación
    let update_app_stmt = tx
        .prepare(
            "UPDATE applications
            SET comment = $1, status = $2
            WHERE user_id = $3 AND vacancy_id = $4",
        )
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Error preparando UPDATE de aplicación: {}", e))
        })?;

    let updated = tx
        .execute(&update_app_stmt, &[&input.feedback, &evaluation_status, &candidate_id, &vacancy_id])
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Error actualizando estado de aplicación: {}", e))
        })?;

    if updated == 0 {
        return Err(AppError::NotFoundError(format!(
            "Aplicación no encontrada para usuario {} y vacante {}",
            candidate_id, vacancy_id
        )));
    }


    // 7. Crear notificación para el candidato
    let title = "Resultado de tu evaluación";
    let message = if evaluation_status == "approved" {
        "¡Felicidades! Has aprobado la evaluación y estás un paso más cerca de unirte al equipo."
    } else {
        "Gracias por participar. No has aprobado esta vez, pero sigue intentándolo. ¡Ánimo!"
    };

    let insert_notification_stmt = tx
        .prepare(
            "INSERT INTO notifications (user_id, title, message)
             VALUES ($1, $2, $3)",
        )
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Error preparando INSERT de notificación: {}", e))
        })?;

    tx.execute(&insert_notification_stmt, &[&candidate_id, &title, &message])
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Error insertando notificación: {}", e))
        })?;

    // 8. Confirmar transacción
    tx.commit()
        .await
        .map_err(|e| {
            AppError::DatabaseError(format!("Error al confirmar transacción: {}", e))
        })?;

    Ok(())
}