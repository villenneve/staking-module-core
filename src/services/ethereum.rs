// Ethereum integration service using ethers-rs
use ethers::prelude::*;
use std::env;
use std::sync::Arc;

/// Connects to Sepolia network with signer (wallet)
pub async fn connect_provider_with_signer() -> Result<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>, Box<dyn std::error::Error>> {
    let rpc_url = env::var("ETH_RPC_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(11155111u64); // Sepolia

    let client = SignerMiddleware::new(provider, wallet);
    Ok(client)
}

/// Execute the stake function on the smart contract.
pub async fn stake_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let client = connect_provider_with_signer().await?;
    let chain_id = client.signer().chain_id();
    println!("🔗 Connected to Ethereum chain ID: {}", chain_id);

    // TODO: Call stake() on the smart contract here

    Ok(())
}

/// Execute the unstake function on the smart contract.
pub async fn unstake_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let client = connect_provider_with_signer().await?;
    let chain_id = client.signer().chain_id();
    println!("🔗 Connected to Ethereum chain ID: {}", chain_id);

    // TODO: Call unstake() on the smart contract here

    Ok(())
}
