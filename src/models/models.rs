// Contains the data structure used for staking request
#[derive(Debug, Clone)]
/// Model representing a staking request with user address and amount.
pub struct StakingRequest {
    pub user_address: String,
    pub amount: u64,
}