use actix_web::{web, App, HttpServer, Responder};
use tokio_postgres::{Client, NoTls};
use dotenv::dotenv;
use std::env;
use crate::responses::json_response; // Importamos el módulo de respuestas

// Importar el módulo de respuestas
mod responses;

async fn get_conn() -> impl Responder {
    // Cargar las variables de entorno desde el archivo .env
    dotenv().ok();
    
    // Obtener la URL de la base de datos desde las variables de entorno
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set in .env file");

    // Establecer la conexión de manera asincrónica a la base de datos
    let (client, connection) =
        tokio_postgres::connect(&db_url, NoTls)
            .await
            .expect("Error connecting to database");

    // Mostrar mensaje en la consola indicando que se conectó exitosamente
    println!("Conexión exitosa a la base de datos!");
        
    let _ : Client = client;
    // Aquí realizarías las operaciones con `client`, como consultas y operaciones en la base de datos

    // Si necesitas manejar la conexión de manera paralela, puedes hacerlo de la siguiente forma
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Responder con un mensaje en formato JSON utilizando el código de estado 200 (OK)
    json_response("Conexión exitosa a la base de datos desde .env", 200)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar las variables de entorno desde el archivo .env
    dotenv().ok();

    // Obtener el puerto desde las variables de entorno, si no está definido usar 8080 por defecto
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    println!("Starting server on http://{host}:{port}");

    // Iniciar el servidor Actix Web
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_conn)) // Ruta para probar la conexión
    })
    .bind(format!("0.0.0.0:{}", port))?  // Usar el puerto configurado en .env
    .run()
    .await
}
