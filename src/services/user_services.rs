use crate::models::user::{NewUser, UserResponse, Claims, LoginData};
use deadpool_postgres::Client;
use crate::responses::errors::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono;

// La función para crear un usuario, que solo permite que un aplicante se registre por sí mismo
pub async fn create_user(
    conn: &Client,
    new_user: NewUser,
    admin_role: Option<String>,  // Este parámetro representaría el rol del usuario autenticado
) -> Result<UserResponse, AppError> {
    // 1. Verificar si el id_number ya existe
    let check_id_stmt = conn
        .prepare("SELECT 1 FROM users WHERE id_number = $1")
        .await
        .map_err(|_| AppError::DatabaseError("Failed to prepare ID check query".into()))?;

    let id_exists = conn
        .query_opt(&check_id_stmt, &[&new_user.id_number])
        .await
        .map_err(|_| AppError::DatabaseError("Failed to execute ID check query".into()))?;

    if id_exists.is_some() {
        return Err(AppError::ValidationError("ID number already exists".into()));
    }

    // 1.2 Verificar si el email ya existe
    let check_email_stmt = conn
        .prepare("SELECT 1 FROM users WHERE email = $1")
        .await
        .map_err(|_| AppError::DatabaseError("Failed to prepare email check query".into()))?;

    let email_exists = conn
        .query_opt(&check_email_stmt, &[&new_user.email])
        .await
        .map_err(|_| AppError::DatabaseError("Failed to execute email check query".into()))?;

    if email_exists.is_some() {
        return Err(AppError::ValidationError("Email already exists".into()));
    }

    // 2. Si el rol es uno de los roles restringidos (analista, evaluador, instructor),
    // se requiere que el usuario tenga el rol de administrador para poder crear el usuario.
    let restricted_roles = ["analista", "evaluador", "instructor"];
    if restricted_roles.contains(&new_user.rol.as_str()) {
        if admin_role.is_none() {
            return Err(AppError::ValidationError(
                "Only admins can create users with the specified role".into(),
            ));
        }
    }

    // 3. Validar que el rol es uno permitido
    let valid_roles = ["postulante", "analista", "evaluador", "instructor"];
    if !valid_roles.contains(&new_user.rol.as_str()) {
        return Err(AppError::ValidationError("Invalid role".into()));
    }

    // 4. Hashear la contraseña con bcrypt
    let hashed_password = hash(&new_user.password, DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError("Password hashing failed".into()))?;

    // 5. Insertar el nuevo usuario en la base de datos
    let stmt = conn
        .prepare(
            "INSERT INTO users (id_number, name, lastname, email, password, rol)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING user_id, id_number, name, lastname, email, rol",
        )
        .await
        .map_err(|_| AppError::UserCreationError("Failed to prepare query".into()))?;

    let rows = conn
        .query(
            &stmt,
            &[
                &new_user.id_number,
                &new_user.name,
                &new_user.lastname,
                &new_user.email,
                &hashed_password,
                &new_user.rol,
            ],
        )
        .await
        .map_err(|_| AppError::UserCreationError("Failed to execute query".into()))?;

    if let Some(row) = rows.get(0) {
        Ok(UserResponse {
            user_id: row.get(0),
            id_number: row.get(1),
            name: row.get(2),
            lastname: row.get(3),
            email: row.get(4),
            rol: row.get(5),
        })
    } else {
        Err(AppError::UserCreationError("Failed to insert user".into()))
    }
}

pub async fn register_user(
    conn: &Client,
    new_user: NewUser,
) -> Result<UserResponse, AppError> {
    // Validar que el rol sea exclusivamente "aplicante"
    if new_user.rol != "postulante" {
        return Err(AppError::ValidationError("Only applicants can self-register".into()));
    }

    // Verificar si el id_number ya existe
    let check_stmt = conn
        .prepare("SELECT 1 FROM users WHERE id_number = $1")
        .await
        .map_err(|_| AppError::DatabaseError("Failed to prepare check query".into()))?;

    let exists = conn
        .query_opt(&check_stmt, &[&new_user.id_number])
        .await
        .map_err(|_| AppError::DatabaseError("Failed to execute check query".into()))?;

    if exists.is_some() {
        return Err(AppError::ValidationError("ID number already exists".into()));
    }

    // Hashear la contraseña
    let hashed_password = hash(&new_user.password, DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError("Password hashing failed".into()))?;

    // Insertar el nuevo usuario
    let stmt = conn
        .prepare(
            "INSERT INTO users (id_number, name, lastname, email, password, rol)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING user_id, id_number, name, lastname, email, rol",
        )
        .await
        .map_err(|_| AppError::UserCreationError("Failed to prepare insert query".into()))?;

    let rows = conn
        .query(
            &stmt,
            &[
                &new_user.id_number,
                &new_user.name,
                &new_user.lastname,
                &new_user.email,
                &hashed_password,
                &new_user.rol,
            ],
        )
        .await
        .map_err(|_| AppError::UserCreationError("Failed to execute insert query".into()))?;

    if let Some(row) = rows.get(0) {
        Ok(UserResponse {
            user_id: row.get("user_id"),
            id_number: row.get("id_number"),
            name: row.get("name"),
            lastname: row.get("lastname"),
            email: row.get("email"),
            rol: row.get("rol"),
        })
    } else {
        Err(AppError::UserCreationError("Failed to insert user".into()))
    }
}


// Función para iniciar sesión
pub async fn login_user(
    conn: &Client,
    login_data: LoginData,
    jwt_secret: &str,
) -> Result<String, AppError> {
    // Buscar al usuario por correo
    let stmt = conn
        .prepare("SELECT user_id, email, password, rol FROM users WHERE email = $1")
        .await
        .map_err(|_| AppError::DatabaseError("Failed to prepare login query".into()))?;

    let row = conn
        .query_opt(&stmt, &[&login_data.email])
        .await
        .map_err(|_| AppError::DatabaseError("Failed to execute login query".into()))?
        .ok_or_else(|| AppError::AuthenticationError("Invalid email or password".into()))?;

    // Verificar la contraseña
    let password_hash: String = row.get("password");
    if !verify(&login_data.password, &password_hash)
        .map_err(|_| AppError::InternalServerError("Password verification failed".into()))?
    {
        return Err(AppError::AuthenticationError("Invalid email or password".into()));
    }

    // Construir las claims del token
    let claims = Claims {
        sub: row.get("email"),
        user_id: row.get("user_id"),
        rol: row.get("rol"),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    // Generar el token JWT
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError("Token generation failed".into()))?;

    Ok(token)
}

pub async fn get_user_profile(
    conn: &Client, 
    email: &str
) -> Result<UserResponse, AppError> {
    // Preparar la consulta para obtener los datos del usuario
    let stmt = conn
        .prepare(
            "SELECT user_id, id_number, name, lastname, email, rol 
             FROM users 
             WHERE email = $1"
        )
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to prepare profile query: {}", e)))?;

    // Ejecutar la consulta
    let row = conn
        .query_opt(&stmt, &[&email])
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to execute profile query: {}", e)))?
        .ok_or_else(|| AppError::NotFoundError(format!("User with email {} not found", email)))?;

    // Construir y devolver el perfil usando UserResponse
    let user_response = UserResponse {
        user_id: row.get("user_id"),
        id_number: row.get("id_number"),
        name: row.get("name"),
        lastname: row.get("lastname"),
        email: row.get("email"),
        rol: row.get("rol"),
    };

    Ok(user_response)
}