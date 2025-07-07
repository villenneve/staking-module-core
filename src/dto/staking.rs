use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StakeRequest {
    pub amount: f64,
}
