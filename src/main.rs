use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::info;
use crate::db::database::connect_db;
use crate::routers::routers::config;
use crate::redis_client::redis_client::RedisClient;

mod auth;
mod db;
mod dto;
mod handlers;
mod middleware;
mod models;
mod redis_client;
mod routers;
mod services;
mod health;

/// Entry point for the Staking Core Service
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();
    info!("📌 Environment variables loaded successfully.");

    // Connect to MongoDB
    let db = connect_db().await;
    info!("✅ Connected to MongoDB.");

    // Connect to Redis
    let redis = RedisClient::new().await;
    info!("✅ Connected to Redis.");

    // Get server address from environment or fallback to default
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:4000".to_string());
    info!("🚀 Staking Core running at {}", server_address);

    // Start HTTP server
    HttpServer::new(move || {
        info!("📌 Loading routes...");
        App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .app_data(actix_web::web::Data::new(redis.clone()))
            .configure(config)
    })
    .bind(server_address)?
    .run()
    .await
}
