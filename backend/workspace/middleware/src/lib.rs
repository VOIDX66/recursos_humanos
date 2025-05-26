// workspace/middleware/src/lib.rs
pub mod auth_middleware;

pub fn hello_middleware() -> &'static str {
    "Middleware crate funcionando"
}
