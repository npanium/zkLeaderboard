use crate::models::{
    AddressQueryParams, BetCountResponse, BetResponse, PaginationParams, PlaceBetRequest,
};
use crate::services::addr_logger_contract_service::AddrLoggerContractService;
use crate::services::hash_contract_service::HashContractService;
use crate::services::{address_service, hash_service};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, HttpResponse, Result};
use ethers::types::{Address, U256};
use ethers::utils::parse_ether;
use log::{debug, error, info};
use serde_json::json;
use sqlx::SqlitePool;

pub async fn get_addresses(query: web::Query<AddressQueryParams>) -> Result<HttpResponse> {
    let count = query.count.unwrap_or(1000);
    debug!(
        "get_addresses: Starting address generation with count={}",
        count
    );

    let addresses = address_service::generate_addresses(count);
    debug!(
        "get_addresses: Successfully generated {} addresses",
        addresses.len()
    );

    Ok(HttpResponse::Ok().json(addresses))
}

pub async fn generate_and_store_addresses(
    pool: web::Data<SqlitePool>,
    query: web::Query<AddressQueryParams>,
) -> Result<HttpResponse> {
    let count = query.count.unwrap_or(1000);
    debug!(
        "generate_and_store_addresses: Starting generation with count={}",
        count
    );

    let addresses = match address_service::generate_and_store_addresses(&pool, count).await {
        Ok(addrs) => {
            debug!(
                "generate_and_store_addresses: Successfully stored {} addresses",
                addrs.len()
            );
            addrs
        }
        Err(e) => {
            error!(
                "generate_and_store_addresses: Failed to generate/store addresses: {}",
                e
            );
            return Err(ErrorInternalServerError(e));
        }
    };

    Ok(HttpResponse::Ok().json(addresses))
}

pub async fn get_stored_addresses(
    pool: web::Data<SqlitePool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(100);
    debug!(
        "get_stored_addresses: Retrieving page {} with {} items per page",
        page, per_page
    );

    let addresses = match address_service::get_stored_addresses(&pool, page, per_page).await {
        Ok(addrs) => {
            debug!("get_stored_addresses: Retrieved {} addresses", addrs.len());
            addrs
        }
        Err(e) => {
            error!("get_stored_addresses: Failed to retrieve addresses: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };

    Ok(HttpResponse::Ok().json(addresses))
}

pub async fn hash_stored_addresses(
    pool: web::Data<SqlitePool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(100);
    debug!(
        "hash_stored_addresses: Processing page {} with {} items per page",
        page, per_page
    );

    let addresses = match address_service::get_stored_addresses(&pool, page, per_page).await {
        Ok(addrs) => {
            debug!(
                "hash_stored_addresses: Retrieved {} addresses for hashing",
                addrs.len()
            );
            addrs
        }
        Err(e) => {
            error!("hash_stored_addresses: Failed to retrieve addresses: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };

    let hash_result = match hash_service::hash_address_data(addresses) {
        Ok(result) => {
            debug!(
                "hash_stored_addresses: Successfully generated hash: {}",
                result.hash
            );
            result
        }
        Err(e) => {
            error!("hash_stored_addresses: Failed to hash addresses: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };

    Ok(HttpResponse::Ok().json(hash_result))
}

pub async fn hash_all_addresses(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("hash_all_addresses: Starting to retrieve all addresses");

    let addresses = match address_service::get_all_addresses(&pool).await {
        Ok(addrs) => {
            debug!(
                "hash_all_addresses: Retrieved {} addresses for hashing",
                addrs.len()
            );
            addrs
        }
        Err(e) => {
            error!("hash_all_addresses: Failed to retrieve addresses: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };

    let hash_result = match hash_service::hash_address_data(addresses) {
        Ok(result) => {
            debug!(
                "hash_all_addresses: Successfully generated hash: {}",
                result.hash
            );
            result
        }
        Err(e) => {
            error!("hash_all_addresses: Failed to hash addresses: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };

    Ok(HttpResponse::Ok().json(hash_result))
}

pub async fn hash_and_store_all_addresses(
    pool: web::Data<SqlitePool>,
    contract_service: web::Data<HashContractService>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("hash_and_store_all_addresses: Starting process");

    let addresses = match address_service::get_all_addresses(&pool).await {
        Ok(addrs) => {
            debug!(
                "hash_and_store_all_addresses: Retrieved {} addresses",
                addrs.len()
            );
            addrs
        }
        Err(e) => {
            error!(
                "hash_and_store_all_addresses: Failed to retrieve addresses: {}",
                e
            );
            return Err(ErrorInternalServerError(e));
        }
    };

    let hash_result = match hash_service::hash_address_data(addresses) {
        Ok(result) => {
            debug!(
                "hash_and_store_all_addresses: Generated hash: {}",
                result.hash
            );
            result
        }
        Err(e) => {
            error!(
                "hash_and_store_all_addresses: Failed to generate hash: {}",
                e
            );
            return Err(ErrorInternalServerError(e));
        }
    };

    let hash_bytes = match hex::decode(&hash_result.hash) {
        Ok(bytes) => {
            debug!("hash_and_store_all_addresses: Successfully decoded hash to bytes");
            bytes
        }
        Err(e) => {
            error!("hash_and_store_all_addresses: Failed to decode hash: {}", e);
            return Err(ErrorInternalServerError(e));
        }
    };

    let mut hash_array = [0u8; 32];
    hash_array.copy_from_slice(&hash_bytes);

    let result = match contract_service
        .store_hash(hash_array, hash_result.timestamp, hash_result.record_count)
        .await
    {
        Ok(res) => {
            info!("hash_and_store_all_addresses: Successfully stored hash in contract");
            debug!(
                "hash_and_store_all_addresses: Transaction result: {}",
                hex::encode(&res)
            );
            res
        }
        Err(e) => {
            error!(
                "hash_and_store_all_addresses: Failed to store hash in contract: {}",
                e
            );
            return Err(ErrorInternalServerError(e));
        }
    };

    Ok(HttpResponse::Ok().json(json!({
        "hash": hash_result.hash,
        "timestamp": hash_result.timestamp,
        "record_count": hash_result.record_count,
        "transaction_result": hex::encode(result)
    })))
}

pub async fn log_random_addresses(
    pool: web::Data<SqlitePool>,
    contract_service: web::Data<AddrLoggerContractService>,
    query: web::Query<AddressQueryParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let count = query.count.unwrap_or(5);
    debug!("log_random_addresses: Starting with count={}", count);

    let addresses = sqlx::query!(
        "SELECT address FROM addresses ORDER BY RANDOM() LIMIT ?",
        count
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    debug!("Selected addresses:");
    for row in &addresses {
        debug!("Address: {}", row.address);
    }

    // Convert string addresses to H160 (Address) type
    let eth_addresses: Vec<Address> = addresses
        .into_iter()
        .filter_map(|row| row.address.parse().ok())
        .collect();

    let transaction_result = contract_service
        .log_addresses(eth_addresses)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({
        "count": count,
        "transaction_hash": hex::encode(transaction_result)
    })))
}

/// Place a new bet on an address
/// Endpoint: POST /api/v0/addresses/bets
/// Body: {
///     "selected_address": "0x...",
///     "position": true,
///     "amount": "0.1"
/// }
pub async fn place_bet(
    contract_service: web::Data<AddrLoggerContractService>,
    bet_request: web::Json<PlaceBetRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("place_bet: Processing bet request");

    let selected_address = bet_request
        .selected_address
        .parse::<Address>()
        .map_err(|e| {
            error!("place_bet: Invalid address format: {}", e);
            ErrorBadRequest("Invalid address format")
        })?;

    let amount = parse_ether(bet_request.amount.as_str()).map_err(|e| {
        error!("place_bet: Invalid ETH amount: {}", e);
        ErrorBadRequest("Invalid ETH amount")
    })?;

    let transaction_result = contract_service
        .place_bet(selected_address, bet_request.position, amount)
        .await
        .map_err(|e| {
            error!("place_bet: Transaction failed: {}", e);
            ErrorInternalServerError("Failed to place bet")
        })?;

    Ok(HttpResponse::Ok().json(json!({
        "transaction_result": hex::encode(transaction_result)
    })))
}

/// Get details of a specific bet by index
/// Endpoint: GET /api/v0/addresses/bets/{index}
/// Example: GET /api/v0/addresses/bets/0 for first bet
pub async fn get_bet(
    contract_service: web::Data<AddrLoggerContractService>,
    index: web::Path<u64>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("get_bet: Retrieving bet at index {}", index);

    let bet = contract_service
        .get_bet(U256::from(index.into_inner()))
        .await
        .map_err(|e| {
            error!("get_bet: Failed to retrieve bet: {}", e);
            ErrorInternalServerError("Failed to retrieve bet")
        })?;

    let response = BetResponse {
        bettor: format!("{:?}", bet.0),
        selected_address: format!("{:?}", bet.1),
        position: bet.2,
        amount: bet.3.to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Get total number of bets placed
/// Endpoint: GET /api/v0/addresses/bets/count
pub async fn get_bet_count(
    contract_service: web::Data<AddrLoggerContractService>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("get_bet_count: Retrieving total bet count");

    let count = contract_service.get_bet_count().await.map_err(|e| {
        error!("get_bet_count: Failed to retrieve count: {}", e);
        ErrorInternalServerError("Failed to retrieve bet count")
    })?;

    Ok(HttpResponse::Ok().json(BetCountResponse {
        count: count.to_string(),
    }))
}
