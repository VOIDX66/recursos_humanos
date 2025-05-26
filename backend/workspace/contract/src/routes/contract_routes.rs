use crate::handlers::contract_handlers::generate_contract_handler;
use actix_web::web;

pub fn protected_contract_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/contracts/generate", web::post().to(generate_contract_handler));
}
