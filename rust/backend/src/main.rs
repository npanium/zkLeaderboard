use actix_cors::Cors;
use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use db::init_db;
use log::info;
use services::contract_service::ContractService;
use std::env;

mod db;
mod handlers;
mod models;
mod services;

use dotenv::dotenv;
//const DATABASE_URL: &str = "sqlite://addresses.db?mode=rwc";

async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::NotFound().json(serde_json::json!({
        "status": "error",
        "message": "Route not found"
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = 3001;
    info!("Starting server at http://localhost:{}", port);

    let pool = init_db(&env::var("DATABASE_URL").expect("Set database url"))
        .await
        .expect("Failed to initialize database");
    // let _pool = SqlitePool::connect("sqlite:data/addresses.db")
    //     .await
    //     .expect("Failed to connect to db");

    let contract_service = ContractService::new(
        &env::var("RPC_URL").expect("RPC_URL not set"),
        &env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set"),
        &env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set"),
    )
    .await
    .expect("Failed to initialize contract service");

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Configure based on your needs

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(contract_service.clone()))
            .service(
                web::scope("/api/v0")
                    .route("/addresses", web::get().to(handlers::get_addresses))
                    .route(
                        "/addresses/generate",
                        web::post().to(handlers::generate_and_store_addresses),
                    )
                    .route(
                        "/addresses/stored",
                        web::get().to(handlers::get_stored_addresses),
                    )
                    .route(
                        "/addresses/hash",
                        web::get().to(handlers::hash_stored_addresses),
                    )
                    .route(
                        "/addresses/hash/all",
                        web::get().to(handlers::hash_all_addresses),
                    )
                    .route(
                        "/addresses/hash/store",
                        web::post().to(handlers::hash_and_store_all_addresses),
                    ),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
