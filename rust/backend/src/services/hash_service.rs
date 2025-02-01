use crate::models::{AddressScore, HashResponse};
use anyhow::Result;
use chrono::Utc;
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};

pub fn hash_address_data(addresses: Vec<AddressScore>) -> Result<HashResponse> {
    // Sort addresses deterministically by address string
    let mut sorted_addresses = addresses;
    sorted_addresses.sort_by(|a, b| a.address.cmp(&b.address));

    // Create a canonical JSON representation
    let canonical_data: Vec<Value> = sorted_addresses
        .iter()
        .map(|addr| {
            json!({
                "address": addr.address.to_lowercase(), // Normalize Ethereum addresses
                "score": addr.score
            })
        })
        .collect();

    // Serialize to string with sorted keys
    let serialized = serde_json::to_string(&canonical_data)?;

    // Create Keccak-256 hash
    let mut hasher = Keccak256::new();
    hasher.update(serialized.as_bytes());
    let hash = hasher.finalize();

    Ok(HashResponse {
        // serialized_data: serialized,
        hash: hex::encode(hash),
        timestamp: Utc::now().timestamp(),
        record_count: sorted_addresses.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_determinism() {
        let addresses = vec![
            AddressScore {
                id: None,
                address: "0xB".to_string(),
                score: 500,
                created_at: None,
            },
            AddressScore {
                id: None,
                address: "0xA".to_string(),
                score: 100,
                created_at: None,
            },
        ];

        let result1 = hash_address_data(addresses.clone()).unwrap();
        let result2 = hash_address_data(addresses).unwrap();

        assert_eq!(
            result1.hash, result2.hash,
            "Hashes should be identical for same input"
        );
    }
}
