use actix_web::{HttpResponse, Responder};

/// Health check endpoint
pub async fn health_handler() -> impl Responder {
    HttpResponse::Ok().body("✅ Service is healthy")
}
