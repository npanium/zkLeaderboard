use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressScore {
    pub id: Option<i64>,
    pub address: String,
    pub score: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct HashResponse {
    // pub serialized_data: String,
    pub hash: String,
    pub timestamp: i64,
    pub record_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct AddressQueryParams {
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct InitRequest {
    pub operator: String,
    pub treasury: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct PlaceBetRequest {
    pub bettor: String,
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

#[derive(Debug, Serialize)]
pub struct WindowStatusResponse {
    pub active: bool,
}

#[derive(Debug, Serialize)]
pub struct BettingAmountsResponse {
    pub up_amount: String,
    pub down_amount: String,
}

#[derive(Debug, Serialize)]
pub struct ContractAddressesResponse {
    pub operator: String,
    pub treasury: String,
    pub token: String,
}

// Token

#[derive(Debug, Deserialize)]
pub struct MintTokenRequest {
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct MintToRequest {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct BurnTokenRequest {
    pub amount: u64,
}

#[derive(Debug, Serialize)]
pub struct TokenBalanceResponse {
    pub balance: u64,
}
