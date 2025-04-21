use deadpool_postgres::{Pool, Manager};
use tokio_postgres::{NoTls, Config};
use std::env;
use dotenv::dotenv;
use std::str::FromStr;

pub struct AppState {
    pub pool: Pool,
}

impl AppState {
    pub fn new(pool: Pool) -> Self {
        AppState { pool }
    }
}

pub async fn get_db_pool() -> Result<Pool, Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    // Parsear la URL completa a configuración de tokio_postgres
    let config = Config::from_str(&database_url)?;

    let manager = Manager::new(config, NoTls);

    let pool = Pool::builder(manager)
        .max_size(16)
        .build()?; // puede fallar, por eso propagamos el error

    Ok(pool)
}
