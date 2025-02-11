use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
// use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
struct AddressData {
    address: String,
    score: i64,
}

#[derive(Serialize, Deserialize)]
struct Output {
    address: String,
    is_top_half: bool,
}

fn main() {
    env::log("Starting guest program");
    let scores: Vec<AddressData> = env::read();

    let mut sorted_scores: Vec<i64> = scores.iter().map(|d| d.score).collect();
    sorted_scores.sort_unstable();

    let median = match sorted_scores.len() % 2 {
        1 => sorted_scores[sorted_scores.len() / 2],
        _ => {
            (sorted_scores[sorted_scores.len() / 2 - 1] + sorted_scores[sorted_scores.len() / 2])
                / 2
        }
    };

    // Generate results
    let results: Vec<Output> = scores
        .into_iter()
        .map(|data| Output {
            address: data.address,
            is_top_half: data.score >= median,
        })
        .collect();

    // Commit results to journal
    env::commit(&results);
}

// Verify score proof (MVP: simple hash verification)
// fn verify_score(score: u64, proof: &[u8]) -> bool {
//     let mut hasher = Sha256::new();
//     hasher.update(score.to_be_bytes());
//     hasher.finalize().as_slice() == proof
// }
