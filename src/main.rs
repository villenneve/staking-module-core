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
    dotenv().ok();
    env_logger::init();
    info!("📌 Environment variables loaded successfully.");

    let db = connect_db().await;
    info!("✅ Connected to MongoDB.");

    let redis = RedisClient::new().await;
    info!("✅ Connected to Redis.");

    // Cloud Run exige PORT como variável de ambiente
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("0.0.0.0:{}", port);
    info!("🚀 Staking Core running at {}", server_address);

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

