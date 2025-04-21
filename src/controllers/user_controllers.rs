use actix_web::web;
use crate::handlers::user_handlers::create_user_handler;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    // Configuramos las rutas que usarán los handlers
    cfg.route("/auth/create_user", web::post().to(create_user_handler));
}