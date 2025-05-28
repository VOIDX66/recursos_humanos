use deadpool_postgres::Client;
use shared::responses::errors::AppError;
use shared::models::evaluations::EvaluationResponse;

pub async fn get_evaluations_for_evaluator(
    client: &Client,
    evaluator_id: &str,
) -> Result<Vec<EvaluationResponse>, AppError> {
    let stmt = client.prepare("
        SELECT id, vacancy_id, candidate_id, evaluation_date, feedback, score, status
        FROM evaluations
        WHERE evaluator_id = $1
        ORDER BY evaluation_date DESC
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
        evaluation_date: row.get::<_, chrono::DateTime<chrono::Utc>>("evaluation_date").to_rfc3339(),
        feedback: row.get("feedback"),
        score: row.get("score"),
        status: row.get("status"),
    }).collect();

    Ok(evaluations)
}