use core::{LeaderboardEntry, LeaderboardInput, QueryResult};
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

fn main() {
    env::log("Starting guest program");
    let input: LeaderboardInput = env::read();

    // Verify all entries first
    let mut valid_entries = Vec::new();
    for entry in &input.entries {
        if verify_score(entry.score, &entry.proof) {
            valid_entries.push(entry.clone());
        }
    }

    env::log(&format!("Found {} valid entries", valid_entries.len()));
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

    env::log(&format!(
        "Query result: found={}, position={}",
        result.found, result.position
    ));
    env::commit(&result);
}

// Verify score proof (MVP: simple hash verification)
fn verify_score(score: u64, proof: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(score.to_be_bytes());
    hasher.finalize().as_slice() == proof
}
