use actix_web::error::ErrorInternalServerError;
use actix_web::{web, HttpResponse, Result};

use log::debug;

use crate::models::AddressQueryParams;
use crate::services::address_service;

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
