use ethers::{
    core::types::SignatureError,
    signers::{LocalWallet, Signer},
    types::Address,
    utils::hash_message,
};

use ethers_core::rand::thread_rng;

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct BetMessage {
    selected_address: String,
    position: bool,
    amount: String,
    nonce: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test wallet

    let wallet = LocalWallet::new(&mut thread_rng());
    println!("Test wallet address: {}", wallet.address());

    // Create bet message
    let message = BetMessage {
        selected_address: "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string(),
        position: true,
        amount: "100".to_string(),
        nonce: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64,
    };

    // Convert message to string
    let message_str = serde_json::to_string(&message)?;
    println!("Message: {}", message_str);

    // Sign message
    let signature = wallet.sign_message(message_str.as_bytes()).await?;
    println!("Signature: {}", signature);

    // This is what you'd send to your API
    println!("\nAPI Request Body (JSON):");
    println!(
        "{}",
        serde_json::json!({
            "message": message_str,
            "signature": signature.to_string(),
            "selected_address": message.selected_address,
            "position": message.position,
            "amount": message.amount
        })
    );

    Ok(())
}
