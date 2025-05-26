// workspace/application/src/lib.rs
pub mod handlers;
pub mod routes;
pub mod services;

pub fn hello_application() -> &'static str {
    "Auth crate funcionando"
}
