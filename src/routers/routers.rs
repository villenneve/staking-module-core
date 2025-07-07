use actix_web::web;
use crate::handlers::{stake::stake_handler, unstake::unstake_handler};
use crate::health::health_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/staking")
            .route("/stake", web::post().to(stake_handler))
            .route("/unstake", web::post().to(unstake_handler)),
    )
    .route("/health", web::get().to(health_handler));
}
