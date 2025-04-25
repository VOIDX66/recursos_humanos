use actix_web::web;
use crate::handlers::user_handlers::{create_user_handler, login_user_handler, profile_handler, register_user_handler};

// Definir las rutas públicas
pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login_user", web::get().to(login_user_handler));
    cfg.route("/auth/register_user", web::post().to(register_user_handler));
}

// Definir las rutas protegidas
pub fn protected_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/me", web::get().to(profile_handler));
    cfg.route("/auth/create_user", web::post().to(create_user_handler));
}
