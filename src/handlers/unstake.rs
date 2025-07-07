// Handler to process an unstaking request from the gateway
use actix_web::{HttpResponse, Responder};
use crate::services::ethereum::unstake_tokens;

/// HTTP handler for unstaking request.
pub async fn unstake_handler() -> impl Responder {
    match unstake_tokens().await {
        Ok(_) => HttpResponse::Ok().body("🔓 Unstake executed successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error during unstake operation: {}", e)),
    }
}