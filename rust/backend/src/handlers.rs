use crate::models::AddressQueryParams;
use crate::models::PaginationParams;
use crate::services::contract_service::ContractService;
use crate::services::{address_service, hash_service};
use actix_web::error::ErrorInternalServerError;
use actix_web::{web, HttpResponse, Result};
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
    contract_service: web::Data<ContractService>,
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
