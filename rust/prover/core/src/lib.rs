extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaderboardEntry {
    pub address: [u8; 20],
    pub score: u64,
    pub proof: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct LeaderboardInput {
    pub entries: Vec<LeaderboardEntry>,
    pub query_address: [u8; 20],
}

#[derive(Serialize, Deserialize)]
pub struct QueryResult {
    pub found: bool,
    pub position: usize,
    pub total: usize,
    pub is_top_50: bool,
}
