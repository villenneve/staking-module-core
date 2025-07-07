// Loads .env environment variables and initializes the logger
use dotenv::dotenv;
use std::env;

/// Load environment variables from .env file and initialize logger.
pub fn load_env() {
    dotenv().ok();
    env_logger::init();
    println!("✅ Environment variables loaded");
}