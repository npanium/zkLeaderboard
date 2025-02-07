use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressScore {
    pub id: Option<i64>,
    pub address: String,
    pub score: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct AddressQueryParams {
    pub count: Option<u32>,
}

#[derive(Deserialize)]
pub struct PlaceBetRequest {
    pub selected_address: String,
    pub position: bool,
    pub amount: String, // ETH amount in string format for precision
}

#[derive(Serialize)]
pub struct BetResponse {
    pub bettor: String,
    pub selected_address: String,
    pub position: bool,
    pub amount: String,
}

#[derive(Serialize)]
pub struct BetCountResponse {
    pub count: String, // U256 as string
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct HashResponse {
    // pub serialized_data: String,
    pub hash: String,
    pub timestamp: i64,
    pub record_count: usize,
}
