use actix_web::{web, Responder, Result};
use actix_web::error::ErrorInternalServerError;
use crate::services::user_services;
use crate::models::user::NewUser;
use crate::state::app_state::AppState;

async fn create_user_handler(
    new_user: web::Json<NewUser>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    // Obtener la conexión del pool
    let conn = app_state.pool.get().await.unwrap();  // Aquí `conn` es un `Object<Manager>`

    // Llamar al servicio para crear el usuario y manejar el error
    let user = user_services::create_user(&conn, new_user.into_inner()).await
        .map_err(|e| ErrorInternalServerError(format!("Error creating user: {}", e)))?;

    Ok(web::Json(user))
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::post().to(create_user_handler));
}
