// Handler to process a staking request from the gateway

use actix_web::{HttpResponse, Responder};
use crate::services::ethereum::stake_tokens;

/// HTTP handler for staking request.
pub async fn stake_handler() -> impl Responder {
    match stake_tokens().await {
        Ok(_) => HttpResponse::Ok().body("🔐 Stake executed successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error during stake operation: {}", e)),
    }
}