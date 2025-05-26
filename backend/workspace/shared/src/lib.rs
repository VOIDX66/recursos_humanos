// workspace/shared/src/lib.rs
pub mod models;
pub mod state;
pub mod responses;

pub fn hello_shared() -> &'static str {
    "Shared crate funcionando"
}
