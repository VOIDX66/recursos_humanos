// workspace/notifications/src/lib.rs
pub mod handlers;
pub mod routes;
pub mod services;

pub fn hello_notification() -> &'static str {
    "Notification crate funcionando"
}
