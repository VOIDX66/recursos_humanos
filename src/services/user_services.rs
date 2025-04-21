use crate::models::user::{NewUser, User};
use tokio_postgres::Client;  // Eliminamos "Error as PgError" porque no lo usamos directamente
use crate::responses::errors::AppError;

pub async fn create_user(conn: &Client, new_user: NewUser) -> Result<User, tokio_postgres::Error> {
    // Preparación de la consulta
    let stmt = conn.prepare(
        "INSERT INTO users (name, lastname, email, password, rol)
        VALUES ($1, $2, $3, $4, $5) 
        RETURNING id, name, lastname, email, password, rol"
    ).await?;
    
    // Ejecución de la consulta
    let rows = conn.query(&stmt, &[
        &new_user.name, 
        &new_user.lastname, 
        &new_user.email, 
        &new_user.password, 
        &new_user.rol
    ]).await?;
    
    // Si se obtienen resultados, se construye el usuario
    if let Some(row) = rows.get(0) {
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            lastname: row.get(2),
            email: row.get(3),
            password: row.get(4),
            rol: row.get(5),
        })
    } else {
        // Usamos nuestro error personalizado
        Err(AppError::UserCreationError("Failed to insert user".to_string()).into())
    }
}