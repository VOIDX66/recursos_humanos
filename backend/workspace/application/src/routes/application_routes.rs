use crate::handlers::application_handlers::{
                                                apply_to_vacancy_handler,
                                                get_applications_for_evaluator_handler,
                                                decide_application_handler
                                            };
use actix_web::web;

pub fn protected_application_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/applications/apply", web::post().to(apply_to_vacancy_handler));
    cfg.route("/applications/get_applications", web::get().to(get_applications_for_evaluator_handler));
    cfg.route("/applications/decide", web::post().to(decide_application_handler));
}
