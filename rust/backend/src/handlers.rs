use actix_web::error::ErrorInternalServerError;
use actix_web::{web, HttpResponse, Result};

use log::debug;

use crate::models::AddressQueryParams;
use crate::services::{address_service, hash_service};

use crate::models::PaginationParams;
use sqlx::SqlitePool;

pub async fn get_addresses(query: web::Query<AddressQueryParams>) -> Result<HttpResponse> {
    let count = query.count.unwrap_or(1000);
    debug!("Generating {} addresses", count);

    let addresses = address_service::generate_addresses(count);
    Ok(HttpResponse::Ok().json(addresses))
}

pub async fn generate_and_store_addresses(
    pool: web::Data<SqlitePool>,
    query: web::Query<AddressQueryParams>,
) -> Result<HttpResponse> {
    let count = query.count.unwrap_or(1000);
    debug!("Generating and storing {} addresses", count);

    let addresses = address_service::generate_and_store_addresses(&pool, count)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(addresses))
}

pub async fn get_stored_addresses(
    pool: web::Data<SqlitePool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(100);

    let addresses = address_service::get_stored_addresses(&pool, page, per_page)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(addresses))
}

pub async fn hash_stored_addresses(
    pool: web::Data<SqlitePool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(100);

    let addresses = address_service::get_stored_addresses(&pool, page, per_page)
        .await
        .map_err(ErrorInternalServerError)?;

    let hash_result =
        hash_service::hash_address_data(addresses).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(hash_result))
}

pub async fn hash_all_addresses(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse, actix_web::Error> {
    let addresses = address_service::get_all_addresses(&pool)
        .await
        .map_err(ErrorInternalServerError)?;

    let hash_result =
        hash_service::hash_address_data(addresses).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(hash_result))
}
