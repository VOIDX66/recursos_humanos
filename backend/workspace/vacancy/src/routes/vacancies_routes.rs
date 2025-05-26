use crate::handlers::vacancies_handlers::{ create_vacancy_handler, get_all_vacancies_handler, get_vacancy_by_id_handler };
use actix_web::web;

// Rutas públicas
pub fn vacancy_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/vacancies", web::get().to(get_all_vacancies_handler));
    cfg.route("/vacancies/{id}", web::get().to(get_vacancy_by_id_handler));
}


// Rutas protegidas para vacantes
pub fn protected_vacancy_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/vacancies/create", web::post().to(create_vacancy_handler));
}