// src/main.rs
pub mod responses;
pub mod state;
pub mod controllers;
pub mod services;
pub mod models;
pub mod schema;
pub mod handlers;

use actix_web::{web, App, HttpServer};
use crate::controllers::user_controllers::user_routes;
use crate::state::app_state::{get_db_pool, AppState};
use crate::services::welcome::welcome;
use std::env;
use log::{info, error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar el logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    dotenv::dotenv().ok();
    
    // Obtener el puerto desde las variables de entorno
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    // Crear el pool de conexiones a la base de datos
    let pool = match get_db_pool().await {
        Ok(pool) => {
            info!("✅ Database connection pool established successfully");
            pool
        },
        Err(e) => {
            error!("❌ Failed to create database connection pool: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed"));
        }
    };
    
    // Crear el estado de la aplicación con el pool
    let app_state = web::Data::new(AppState::new(pool));
    
    info!("🚀 Starting server on http://0.0.0.0:{}", port);
    
    // Iniciar el servidor Actix Web
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(welcome)) // Ruta de bienvenida
            .configure(user_routes)  // Rutas de usuario
    })
    .bind(format!("0.0.0.0:{}", port))
    .map_err(|e| {
        error!("❌ Failed to bind to port {}: {}", port, e);
        e
    })?
    .run()
    .await
    .map_err(|e| {
        error!("❌ Server error: {}", e);
        e
    })
}
