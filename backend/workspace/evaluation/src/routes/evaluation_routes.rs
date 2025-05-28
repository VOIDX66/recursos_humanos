use crate::handlers::evaluation_handlers::{ get_evaluations_for_evaluator_handler, grade_evaluation_handler };
use actix_web::web;

pub fn protected_evaluations_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/evaluations", web::get().to(get_evaluations_for_evaluator_handler));
    cfg.route("/evaluations/grade_evaluation", web::post().to(grade_evaluation_handler));
}
