// workspace/evaluation/src/lib.rs
pub mod handlers;
pub mod routes;
pub mod services;

pub fn hello_evaluation() -> &'static str {
    "Evaluation crate funcionando"
}
