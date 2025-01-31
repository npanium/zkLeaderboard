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
