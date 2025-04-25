use serde::{Serialize, Deserialize};
use validator::{ValidationError, Validate};

fn validate_role(rol: &str) -> Result<(), ValidationError> {
    let allowed_roles = ["admin", "analista", "postulante", "evaluador", "instructor"];
    
    if allowed_roles.contains(&rol) {
        Ok(())  // Validación exitosa
    } else {
        let mut error = ValidationError::new("invalid_role");
        error.message = Some("Role must be one of: admin, analista, postulante, evaluador, instructor".into());
        Err(error)  // Si no es un rol válido, retornamos un error
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: i32,
    pub id_number: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub rol: String,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct NewUser {
    #[validate(length(min = 6, max = 12, message = "ID number must be between 6 and 12 characters"))]
    pub id_number: String,

    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(length(min = 1, message = "Lastname is required"))]
    pub lastname: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[serde(skip_serializing)]
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,

    // Usamos un cierre que pasa el campo 'rol' a la función de validación
    #[validate(custom(function = "validate_role"))]
    pub rol: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub id_number: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub rol: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginData {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // Asunto del token: aquí ponemos el email del usuario
    pub user_id: i32,     // ID del usuario (para identificarlo luego)
    pub rol: String,      // Rol del usuario (analista, evaluador, etc.)
    pub exp: usize,       // Fecha de expiración (timestamp UNIX en segundos)
}

