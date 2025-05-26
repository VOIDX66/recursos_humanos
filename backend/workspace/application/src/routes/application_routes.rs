use crate::handlers::application_handlers::apply_to_vacancy_handler;
use actix_web::web;

pub fn protected_application_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/applications/apply", web::post().to(apply_to_vacancy_handler));
}