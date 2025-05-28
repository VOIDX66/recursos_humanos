//use crate::models::user::{Claims, LoginData, NewUser, UpdateRolData};
use shared::models::user::{Claims, LoginData, NewUser, UpdateRolData, DeleteUserPayload};
//use crate::responses::{errors::AppError, json_response::AuthResponse};
use shared::responses::{errors::AppError, json_response::AuthResponse};
use crate::services::auth_services;
//use crate::state::app_state::AppState;
use shared::state::app_state::AppState;
use actix_web::{HttpMessage, HttpRequest, Responder, Result, web};
use validator::Validate;

pub async fn create_user_handler(
    new_user: web::Json<NewUser>,
    app_state: web::Data<AppState>,
    req: HttpRequest, // Recibimos la solicitud para obtener el token
) -> Result<impl Responder, AppError> {
    // Intentamos extraer los claims del middleware de autenticación
    let claims = req.extensions().get::<Claims>().cloned();
    // Si no hay claims (no está logueado), solo podemos crear un aplicante
    if claims.is_none() {
        if new_user.role != "aplicante" {
            return Err(AppError::Unauthorized(
                "Only applicants can register without being logged in".into(),
            ));
        }
    } else {
        // Si el usuario está autenticado, verificamos su rol
        let claims = claims.as_ref().unwrap(); // Usamos `as_ref` para obtener una referencia

        if claims.role != "admin" {
            return Err(AppError::Unauthorized(
                "Only admins can create users with roles other than applicant".into(),
            ));
        }
    }

    // Obtener la conexión del pool de manera segura
    let conn = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to get database connection: {}", e))
    })?;

    // Validación de los datos usando el trait Validate de la librería validator
    if let Err(e) = new_user.validate() {
        return Err(AppError::ValidationError(format!("Validation failed: {:?}", e)).into());
    }

    // Llamar al servicio para crear el usuario
    match auth_services::create_user(
        &conn,
        new_user.into_inner(),
        claims.as_ref().map(|c| c.role.clone()),
    )
    .await
    {
        Ok(user) => Ok(web::Json(user)),
        Err(e) => Err(AppError::UserCreationError(format!("Error creating user: {}", e)).into()),
    }
}

pub async fn register_user_handler(
    new_user: web::Json<NewUser>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    // Validar los datos usando el trait Validate
    if let Err(e) = new_user.validate() {
        return Err(AppError::ValidationError(format!(
            "Validation failed: {:?}",
            e
        )));
    }

    // Obtener conexión de la base de datos
    let conn = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to get database connection: {}", e))
    })?;

    // Llamar al servicio para crear el usuario, sin rol de admin ya que es un registro público
    match auth_services::create_user(&conn, new_user.into_inner(), None).await {
        Ok(user) => Ok(web::Json(user)),
        Err(e) => Err(AppError::UserCreationError(format!("Error creating user: {}", e)).into()),
    }
}

pub async fn login_user_handler(
    login_data: web::Json<LoginData>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    // Validar los datos de login usando validator
    login_data
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Obtener conexión a la base de datos
    let conn = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to get DB connection: {}", e))
    })?;

    // Llamar al servicio de login (este servicio gestionará la validación de la contraseña y la generación del token)
    let token =
        auth_services::login_user(&conn, login_data.into_inner(), &app_state.jwt_secret).await?;

    // Devolver el token de autenticación
    Ok(web::Json(AuthResponse { token }))
}

pub async fn profile_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    // Extraer claims del middleware de autenticación
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("User claims not found".to_string()))?;

    // Obtener conexión de la base de datos
    let conn = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to get database connection: {}", e))
    })?;

    // Obtener información completa del perfil usando el ID del usuario en los claims
    match auth_services::get_user_profile(&conn, &claims.sub).await {
        Ok(profile) => Ok(web::Json(profile)),
        Err(e) => Err(AppError::NotFoundError(format!("Error fetching profile: {}", e)).into()),
    }
}

pub async fn update_rol_handler(
    update_data: web::Json<UpdateRolData>,
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // Extraer los claims del request
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::Unauthorized("Unauthorized: Missing claims".into()))?;

    // Solo administradores pueden cambiar roles
    if claims.role != "admin" {
        return Err(AppError::Unauthorized(
            "Only admins can update user roles".into(),
        ));
    }

    // Obtener conexión de la base de datos
    let conn = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to get database connection: {}", e))
    })?;

    // Llamar al servicio que actualiza el rol
    match auth_services::update_rol(
        &conn,
        &claims.user_id,
        &update_data.id_number,
        &update_data.new_rol,
    )
    .await
    {
        Ok(updated_user) => Ok(web::Json(updated_user)),
        Err(e) => Err(AppError::UpdateError(format!("Failed to update user role: {}", e)).into()),
    }
}

pub async fn get_all_users_handler(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    // 1. Extraer claims del request
    let claims = req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::AuthenticationError("No se encontraron los claims del usuario".to_string()))?;

    // 2. Validar que el rol sea admin
    if claims.role != "admin" {
        return Err(AppError::Unauthorized("Solo los administradores pueden ver todos los usuarios".to_string()));
    }

    // 3. Obtener conexión a la base de datos
    let client = app_state.pool.get().await
        .map_err(|e| AppError::InternalServerError(format!("Error al obtener conexión a BD: {}", e)))?;

    // 4. Obtener lista de usuarios
    let users = auth_services::get_all_users(&client).await?;

    // 5. Responder con JSON
    Ok(web::Json(users))
}

pub async fn delete_user_handler(
    body: web::Json<DeleteUserPayload>,
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let target_user_id = &body.user_id;

    // Extraer los claims del request
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| AppError::Unauthorized("Unauthorized: Missing claims".into()))?;

    // Solo administradores pueden eliminar usuarios
    if claims.role != "admin" {
        return Err(AppError::Unauthorized(
            "Only admins can delete users".into(),
        ));
    }

    // Obtener conexión de la base de datos
    let conn = app_state.pool.get().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to get database connection: {}", e))
    })?;

    // Llamar al servicio de eliminación
    auth_services::delete_user(&conn, &claims.user_id, target_user_id)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete user: {}", e)))?;

    Ok(web::Json(serde_json::json!({
        "message": "User deleted successfully"
    })))
}