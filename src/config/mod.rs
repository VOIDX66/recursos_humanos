use std::env;
use dotenv::dotenv;

pub fn config() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    Ok(())
}