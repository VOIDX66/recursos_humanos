use deadpool_postgres::Client;
use shared::responses::errors::AppError;
use shared::models::notification::Notification;

pub async fn mark_notification_as_read(
    client: &Client,
    notification_id: &str,
) -> Result<(), AppError> {
    // 1. Preparar la consulta
    let stmt = client.prepare(
        "UPDATE notifications
         SET is_read = TRUE
         WHERE id = $1"
    ).await.map_err(|e| {
        AppError::DatabaseError(format!("Error preparando UPDATE de notificación: {}", e))
    })?;

    // 2. Ejecutar la actualización
    let rows_affected = client.execute(&stmt, &[&notification_id]).await.map_err(|e| {
        AppError::DatabaseError(format!("Error ejecutando UPDATE de notificación: {}", e))
    })?;

    // 3. Validar si se actualizó alguna fila
    if rows_affected == 0 {
        return Err(AppError::NotFoundError(format!(
            "No se encontró la notificación con ID {}",
            notification_id
        )));
    }

    Ok(())
}

pub async fn get_notifications_for_user(
    client: &Client,
    user_id: &str,
) -> Result<Vec<Notification>, AppError> {
    let stmt = client
        .prepare(
            "SELECT id, title, message, is_read, created_at, updated_at
             FROM notifications
             WHERE user_id = $1
             ORDER BY created_at DESC",
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Error preparando consulta de notificaciones: {}", e)))?;

    let rows = client
        .query(&stmt, &[&user_id])
        .await
        .map_err(|e| AppError::DatabaseError(format!("Error ejecutando consulta de notificaciones: {}", e)))?;

    let notifications = rows
        .into_iter()
        .map(|row| Notification {
            id: row.get("id"),
            title: row.get("title"),
            message: row.get("message"),
            is_read: row.get("is_read"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(notifications)
}