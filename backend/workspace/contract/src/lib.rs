// workspace/contract/src/lib.rs
pub mod handlers;
pub mod routes;
pub mod services;

pub fn hello_contract() -> &'static str {
    "Contract crate funcionando"
}
