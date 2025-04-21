use crate::models::user::{NewUser, UserResponse};
use tokio_postgres::Client;
use crate::responses::errors::AppError;
use bcrypt::{hash, DEFAULT_COST};
use regex::Regex;

pub async fn create_user(conn: &Client, new_user: NewUser) -> Result<UserResponse, AppError> {
    // Validación de correo
    let email_re = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    if !email_re.is_match(&new_user.email) {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }

    // Validación del número de identificación
    if new_user.id_number.len() < 6 || new_user.id_number.len() > 12 {
        return Err(AppError::ValidationError("ID number length must be between 6 and 12".to_string()));
    }

    // Validar si el id_number ya existe
    let check_stmt = conn.prepare("SELECT 1 FROM users WHERE id_number = $1").await
        .map_err(|_| AppError::DatabaseError("Failed to prepare check query".to_string()))?;

    let exists = conn.query_opt(&check_stmt, &[&new_user.id_number]).await
        .map_err(|_| AppError::DatabaseError("Failed to execute check query".to_string()))?;

    if exists.is_some() {
        return Err(AppError::ValidationError("ID number already exists".to_string()));
    }

    // Hashear la contraseña con bcrypt
    let hashed_password = hash(new_user.password, DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError("Password hashing failed".to_string()))?;

    // Preparación de la consulta SQL para insertar el usuario
    let stmt = conn.prepare(
        "INSERT INTO users (id_number, name, lastname, email, password, rol)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING user_id, id_number, name, lastname, email, password, rol"
    ).await.map_err(|_| AppError::UserCreationError("Failed to prepare query".to_string()))?;

    // Ejecución de la consulta
    let rows = conn.query(&stmt, &[
        &new_user.id_number,
        &new_user.name,
        &new_user.lastname,
        &new_user.email,
        &hashed_password,
        &new_user.rol
    ]).await.map_err(|_| AppError::UserCreationError("Failed to execute query".to_string()))?;

    // Si se obtienen resultados, se construye el usuario
    if let Some(row) = rows.get(0) {
        Ok(UserResponse {
            user_id: row.get(0),
            id_number: row.get(1),
            name: row.get(2),
            lastname: row.get(3),
            email: row.get(4),
            rol: row.get(6),
        })
    } else {
        // Usamos nuestro error personalizado
        Err(AppError::UserCreationError("Failed to insert user".to_string()))
    }
}
