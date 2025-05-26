use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::{error, info};
use auth::routes::auth_routes::protected_user_routes;
use vacancy::routes::vacancies_routes::{protected_vacancy_routes, vacancy_routes};
use contract::routes::contract_routes::protected_contract_routes;
use std::env;

use middleware::auth_middleware::validator;
use auth::routes::auth_routes::user_routes;
use auth::services::welcome::welcome;
use shared::state::app_state::{AppState, get_db_pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger y .env
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();

    // Configuración
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());

    // Pool de BD
    let pool = match get_db_pool().await {
        Ok(p) => {
            info!("✅ Database connection pool established successfully");
            p
        }
        Err(e) => {
            error!("❌ Failed to create database connection pool: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "DB pool init failed",
            ));
        }
    };
    let app_state = web::Data::new(AppState::new(pool, jwt_secret.clone()));

    info!("🚀 Starting server on http://0.0.0.0:{}", port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATH", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            // Rutas abiertas
            .route("/", web::get().to(welcome))
            .configure(user_routes)
            .configure(vacancy_routes)
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(validator))
                    .configure(protected_contract_routes)
                    .configure(protected_user_routes)
                    .configure(protected_vacancy_routes)
            )
    })
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
