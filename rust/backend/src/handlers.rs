use std::env;

use crate::models::{
    AddressQueryParams, BetCountResponse, BetResponse, BettingAmountsResponse, BurnTokenRequest,
    InitRequest, MintToRequest, MintTokenRequest, PaginationParams, PlaceBetRequest,
    TokenBalanceResponse, WindowStatusResponse,
};
use crate::services::addr_logger_contract_service::AddrLoggerContractService;
use crate::services::betting_token_service::BettingTokenService;
use crate::services::hash_contract_service::HashContractService;
use crate::services::{address_service, hash_service};
use actix_web::error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError};
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

/// Initialize the contract with operator, treasury and token addresses
pub async fn init_contract(
    contract_service: web::Data<AddrLoggerContractService>,
    init_request: web::Json<InitRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("init_contract: Starting contract initialization");

    let operator = init_request.operator.parse::<Address>().map_err(|e| {
        error!("init_contract: Invalid operator address: {}", e);
        ErrorBadRequest("Invalid operator address")
    })?;

    let treasury = init_request.treasury.parse::<Address>().map_err(|e| {
        error!("init_contract: Invalid treasury address: {}", e);
        ErrorBadRequest("Invalid treasury address")
    })?;

    let token = env::var("TOKEN_CONTRACT_ADDRESS")
        .map_err(|_| {
            error!("init_contract: TOKEN_CONTRACT_ADDRESS not set in environment");
            ErrorInternalServerError("Token address not configured")
        })?
        .parse::<Address>()
        .map_err(|e| {
            error!("init_contract: Invalid token address in environment: {}", e);
            ErrorInternalServerError("Invalid token address configuration")
        })?;

    let transaction_result = contract_service
        .init(operator, treasury, token)
        .await
        .map_err(|e| {
            error!("init_contract: Transaction failed: {}", e);
            ErrorInternalServerError("Failed to initialize contract")
        })?;

    Ok(HttpResponse::Ok().json(json!({
        "transaction_result": hex::encode(transaction_result)
    })))
}

pub async fn start_betting_window(
    pool: web::Data<SqlitePool>,
    contract_service: web::Data<AddrLoggerContractService>,
    query: web::Query<AddressQueryParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let count = query.count.unwrap_or(5);
    debug!("start_betting_window: Starting with count={}", count);

    // Check if a window is already active
    if contract_service.get_window_active().await.map_err(|e| {
        error!("start_betting_window: Failed to check window status: {}", e);
        ErrorInternalServerError("Failed to check window status")
    })? {
        return Err(ErrorForbidden("A betting window is already active"));
    }

    let addresses = sqlx::query!(
        "SELECT address FROM addresses ORDER BY RANDOM() LIMIT ?",
        count
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    // Log raw addresses from database
    debug!("Selected addresses from database:");
    let raw_addresses: Vec<String> = addresses.iter().map(|row| row.address.clone()).collect();
    for (i, addr) in raw_addresses.iter().enumerate() {
        debug!("Address {}: {}", i + 1, addr);
    }

    let eth_addresses: Vec<Address> = addresses
        .into_iter()
        .filter_map(|row| row.address.parse().ok())
        .collect();

    // Log converted Ethereum addresses
    debug!("Converted Ethereum addresses:");
    for (i, addr) in eth_addresses.iter().enumerate() {
        debug!("ETH Address {}: {:?}", i + 1, addr);
    }

    let transaction_result = contract_service
        .start_betting_window(eth_addresses.clone())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({
        "count": count,
        "addresses": raw_addresses,
        "eth_addresses": eth_addresses.iter().map(|addr| format!("{:?}", addr)).collect::<Vec<String>>(),
        "transaction_result": hex::encode(transaction_result)
    })))
}
pub async fn close_betting_window(
    contract_service: web::Data<AddrLoggerContractService>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("close_betting_window: Closing current betting window");

    // Check if a window is active
    if !contract_service.get_window_active().await.map_err(|e| {
        error!("close_betting_window: Failed to check window status: {}", e);
        ErrorInternalServerError("Failed to check window status")
    })? {
        return Err(ErrorForbidden("No active betting window found"));
    }

    let transaction_result = contract_service.close_betting_window().await.map_err(|e| {
        error!("close_betting_window: Transaction failed: {}", e);
        ErrorInternalServerError("Failed to close betting window")
    })?;

    Ok(HttpResponse::Ok().json(json!({
        "transaction_result": hex::encode(transaction_result)
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

    // Check if betting window is active
    if !contract_service.get_window_active().await.map_err(|e| {
        error!("place_bet: Failed to check window status: {}", e);
        ErrorInternalServerError("Failed to check window status")
    })? {
        return Err(ErrorForbidden("No active betting window"));
    }

    let selected_address = bet_request
        .selected_address
        .parse::<Address>()
        .map_err(|e| {
            error!("place_bet: Invalid address format: {}", e);
            ErrorBadRequest("Invalid address format")
        })?;

    // Verify if address is valid for current window
    if !contract_service
        .is_valid_address(selected_address)
        .await
        .map_err(|e| {
            error!("place_bet: Failed to validate address: {}", e);
            ErrorInternalServerError("Failed to validate address")
        })?
    {
        return Err(ErrorBadRequest(
            "Invalid address for current betting window",
        ));
    }

    let amount = U256::from_dec_str(&bet_request.amount).map_err(|e| {
        error!("place_bet: Invalid amount format: {}", e);
        ErrorBadRequest("Invalid amount format")
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

pub async fn get_window_status(
    contract_service: web::Data<AddrLoggerContractService>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("get_window_status: Checking betting window status");

    let is_active = contract_service.get_window_active().await.map_err(|e| {
        error!("get_window_status: Failed to get window status: {}", e);
        ErrorInternalServerError("Failed to get window status")
    })?;

    Ok(HttpResponse::Ok().json(WindowStatusResponse { active: is_active }))
}

pub async fn get_betting_amounts(
    contract_service: web::Data<AddrLoggerContractService>,
    index: web::Path<u64>,
) -> Result<HttpResponse, actix_web::Error> {
    let addr_index = U256::from(index.into_inner());
    debug!(
        "get_betting_amounts: Retrieving amounts for index {}",
        addr_index
    );

    let up_amount = contract_service
        .get_up_amount(addr_index)
        .await
        .map_err(|e| {
            error!("get_betting_amounts: Failed to get up amount: {}", e);
            ErrorInternalServerError("Failed to get up amount")
        })?;

    let down_amount = contract_service
        .get_down_amount(addr_index)
        .await
        .map_err(|e| {
            error!("get_betting_amounts: Failed to get down amount: {}", e);
            ErrorInternalServerError("Failed to get down amount")
        })?;

    Ok(HttpResponse::Ok().json(BettingAmountsResponse {
        up_amount: up_amount.to_string(),
        down_amount: down_amount.to_string(),
    }))
}

pub async fn process_payouts(
    contract_service: web::Data<AddrLoggerContractService>,
    winners: web::Json<Vec<bool>>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "process_payouts: Processing payouts for {} addresses",
        winners.len()
    );

    // Check if betting window is closed
    if contract_service.get_window_active().await.map_err(|e| {
        error!("process_payouts: Failed to check window status: {}", e);
        ErrorInternalServerError("Failed to check window status")
    })? {
        return Err(ErrorForbidden(
            "Betting window must be closed before processing payouts",
        ));
    }

    contract_service
        .process_payouts(winners.into_inner())
        .await
        .map_err(|e| {
            error!("process_payouts: Failed to process payouts: {}", e);
            ErrorInternalServerError("Failed to process payouts")
        })?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Payouts processed successfully"
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

// Tokens

pub async fn mint_tokens(
    contract_service: web::Data<BettingTokenService>,
    request: web::Json<MintTokenRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "mint_tokens: Processing mint request for {} tokens",
        request.amount
    );

    let transaction_result = contract_service.mint(request.amount).await.map_err(|e| {
        error!("mint_tokens: Transaction failed: {}", e);
        ErrorInternalServerError("Failed to mint tokens")
    })?;

    Ok(HttpResponse::Ok().json(json!({
        "transaction_result": hex::encode(transaction_result)
    })))
}

pub async fn mint_to_address(
    contract_service: web::Data<BettingTokenService>,
    request: web::Json<MintToRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "mint_to_address: Processing mint request for {} tokens",
        request.amount
    );

    let to = request.address.parse::<Address>().map_err(|e| {
        error!("mint_to_address: Invalid address format: {}", e);
        ErrorBadRequest("Invalid address format")
    })?;

    let transaction_result = contract_service
        .mint_to(to, request.amount)
        .await
        .map_err(|e| {
            error!("mint_to_address: Transaction failed: {}", e);
            ErrorInternalServerError("Failed to mint tokens")
        })?;

    Ok(HttpResponse::Ok().json(json!({
        "transaction_result": hex::encode(transaction_result)
    })))
}

pub async fn burn_tokens(
    contract_service: web::Data<BettingTokenService>,
    request: web::Json<BurnTokenRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "burn_tokens: Processing burn request for {} tokens",
        request.amount
    );

    let transaction_result = contract_service.burn(request.amount).await.map_err(|e| {
        error!("burn_tokens: Transaction failed: {}", e);
        ErrorInternalServerError("Failed to burn tokens")
    })?;

    Ok(HttpResponse::Ok().json(json!({
        "transaction_result": hex::encode(transaction_result)
    })))
}

pub async fn get_token_balance(
    contract_service: web::Data<BettingTokenService>,
    address: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "get_token_balance: Retrieving balance for address {}",
        address
    );

    let account = address.parse::<Address>().map_err(|e| {
        error!("get_token_balance: Invalid address format: {}", e);
        ErrorBadRequest("Invalid address format")
    })?;

    debug!("Account {}", account);
    let balance = contract_service.balance_of(account).await.map_err(|e| {
        error!("get_token_balance: Failed to get balance: {}", e);
        ErrorInternalServerError("Failed to get token balance")
    })?;
    debug!("Balance {}", balance);

    Ok(HttpResponse::Ok().json(TokenBalanceResponse { balance: balance }))
}
