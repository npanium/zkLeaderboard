use ciborium::into_writer;
use core::{LeaderboardEntry, LeaderboardInput, QueryResult};
use hex::encode;
use methods::{ZKLEADERBOARD_GUEST_ELF, ZKLEADERBOARD_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use sha2::{Digest, Sha256};
use std::{env::args, fs::File};

fn main() {
    // Get query address from command line
    let query_address = args()
        .nth(1)
        .expect("Please provide an ETH address to query");

    // Validate address format
    if !query_address.starts_with("0x") || query_address.len() != 42 {
        println!(
            "Error: Invalid ETH address format. Must be 42 characters long and start with '0x'"
        );
        return;
    }

    // Convert address to fixed-size array
    let query_address_bytes: [u8; 20] = hex::decode(&query_address[2..])
        .expect("Invalid address format")
        .try_into()
        .unwrap();

    // Create leaderboard entries programmatically (for testing)
    let entries = vec![
        LeaderboardEntry {
            address: hex::decode("0000000000000000000000000000000000000001")
                .unwrap()
                .try_into()
                .unwrap(),
            score: 1500,
            proof: dummy_proof(1500),
        },
        LeaderboardEntry {
            address: hex::decode("0000000000000000000000000000000000000002")
                .unwrap()
                .try_into()
                .unwrap(),
            score: 2000,
            proof: dummy_proof(2000),
        },
        // Add more entries as needed
    ];

    // Create input for the guest program
    let input = LeaderboardInput {
        entries,
        query_address: query_address_bytes,
    };

    // Initialize the executor environment
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Create the prover
    let prover = default_prover();

    // Run the prover
    println!("Generating proof...");
    let receipt = prover.prove(env, ZKLEADERBOARD_GUEST_ELF).unwrap().receipt;

    // Serialize the proof
    let mut bin_receipt = Vec::new();
    into_writer(&receipt, &mut bin_receipt).unwrap();

    // Save the proof to file
    let out = File::create("proof.bin").unwrap();
    into_writer(&receipt, out).unwrap();

    // Print zkVerify submission data
    println!("\nzkVerify Submission Data:");
    println!("-------------------------");
    println!("1. Serialized proof (hex):");
    // println!("{}", encode(&bin_receipt));

    println!("\n2. Journal bytes (hex):");
    let receipt_journal_bytes_array = &receipt.journal.bytes.as_slice();
    println!("{}", encode(&receipt_journal_bytes_array));

    println!("\n3. Guest program fingerprint (hex):");
    let image_id_hex = encode(
        ZKLEADERBOARD_GUEST_ID
            .into_iter()
            .flat_map(|v| v.to_le_bytes().into_iter())
            .collect::<Vec<_>>(),
    );
    println!("{}", image_id_hex);

    // Print the result
    println!("\nQuery Result:");
    println!("-------------");
    let result: QueryResult = receipt.journal.decode().unwrap();
    println!("Result:{}", result.position);
}

// Generate dummy proof for testing
fn dummy_proof(score: u64) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(score.to_be_bytes());
    hasher.finalize().to_vec()
}
