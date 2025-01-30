use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone)]
struct LeaderboardEntry {
    address: [u8; 20], // Fixed-size Ethereum address
    score: u64,
    proof: Vec<u8>, // Proof of score validity
}

#[derive(Serialize, Deserialize)]
struct LeaderboardInput {
    entries: Vec<LeaderboardEntry>,
    query_address: [u8; 20], // Fixed-size query address
}

#[derive(Serialize, Deserialize)]
struct QueryResult {
    found: bool,
    position: usize,
    total: usize,
    is_top_50: bool,
}

fn main() {
    // Read the input containing both the data and query
    let input: LeaderboardInput = env::read();

    // Verify all entries first
    let mut valid_entries = Vec::new();
    for entry in &input.entries {
        if verify_score(entry.score, &entry.proof) {
            valid_entries.push(entry.clone());
        }
    }

    // Sort valid entries by score in descending order
    valid_entries.sort_by(|a, b| b.score.cmp(&a.score));

    // Find position of query address
    let total_entries = valid_entries.len();
    let mut position = 0;
    let mut found = false;

    for (i, entry) in valid_entries.iter().enumerate() {
        if entry.address == input.query_address {
            position = i;
            found = true;
            break;
        }
    }

    // Calculate if in top 50%
    let is_top_50 = if found {
        position < total_entries / 2
    } else {
        false
    };

    // Create structured output
    let result = QueryResult {
        found,
        position: position + 1,
        total: total_entries,
        is_top_50,
    };

    // Commit the result
    env::commit(&result);
}

// Verify score proof (MVP: simple hash verification)
fn verify_score(score: u64, proof: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(score.to_be_bytes());
    hasher.finalize().as_slice() == proof
}
