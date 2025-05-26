// workspace/auth/src/lib.rs
pub mod handlers;
pub mod routes;
pub mod services;

pub fn hello_auth() -> &'static str {
    "Auth crate funcionando"
}
