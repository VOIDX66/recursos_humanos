use crate::handlers::notification_handlers::{ mark_notification_as_read_handler, get_notifications_for_user_handler };
use actix_web::web;

pub fn protected_notifications_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/notifications/mark_as_read", web::post().to(mark_notification_as_read_handler));
    cfg.route("/notifications", web::get().to(get_notifications_for_user_handler));
}
