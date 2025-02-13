use actix_cors::Cors;
use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use db::init_db;
use dotenv::dotenv;
use log::info;

use services::{
    addr_logger_contract_service::AddrLoggerContractService,
    betting_token_service::BettingTokenService, hash_contract_service::HashContractService,
};
use std::env;

mod db;
mod handlers;
mod models;
mod services;

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

    let hash_contract_service = HashContractService::new(
        &env::var("RPC_URL").expect("RPC_URL not set"),
        &env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set"),
        &env::var("HASH_CONTRACT_ADDRESS").expect("HASH_CONTRACT_ADDRESS not set"),
    )
    .await
    .expect("Failed to initialize hash contract service");

    let betting_token_service = BettingTokenService::new(
        &env::var("RPC_URL").expect("RPC_URL not set"),
        &env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set"),
        &env::var("TOKEN_CONTRACT_ADDRESS").expect("TOKEN_CONTRACT_ADDRESS not set"),
    )
    .await
    .expect("Failed to initialize betting token service");

    let addr_logger_contract_service = AddrLoggerContractService::new(
        &env::var("RPC_URL").expect("RPC_URL not set"),
        &env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set"),
        &env::var("ADDR_LOGGER_CONTRACT_ADDRESS").expect("ADDR_LOGGER_CONTRACT_ADDRESS not set"),
    )
    .await
    .expect("Failed to initialize address logger contract service");

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Configure based on your needs

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(hash_contract_service.clone()))
            .app_data(Data::new(addr_logger_contract_service.clone()))
            .app_data(Data::new(betting_token_service.clone()))
            .service(
                web::scope("/api/v0/addresses")
                    .route("", web::get().to(handlers::get_all_addresses))
                    .route("/hash", web::get().to(handlers::hash_stored_addresses))
                    .route("/hash/all", web::get().to(handlers::hash_all_addresses))
                    .route(
                        "/hash/store",
                        web::post().to(handlers::hash_and_store_all_addresses),
                    )
                    // Contract initialization
                    .route("/init", web::post().to(handlers::init_contract))
                    // Betting window management
                    .route(
                        "/window/start",
                        web::post().to(handlers::start_betting_window),
                    )
                    .route(
                        "/window/close",
                        web::post().to(handlers::close_betting_window),
                    )
                    .route("/window/status", web::get().to(handlers::get_window_status))
                    // Static routes must come before dynamic routes with parameters
                    .route("/bets/count", web::get().to(handlers::get_bet_count))
                    .route("/bets", web::post().to(handlers::place_bet))
                    .route("/bets/{index}", web::get().to(handlers::get_bet))
                    .route(
                        "/bets/amounts/{index}",
                        web::get().to(handlers::get_betting_amounts),
                    )
                    // Payout processing being done by Solidity contract
                    // .route("/payouts", web::post().to(handlers::process_payouts))
                    // Address generation and storage
                    .route("", web::get().to(handlers::get_addresses))
                    .route(
                        "/generate",
                        web::post().to(handlers::generate_and_store_addresses),
                    )
                    .route("/stored", web::get().to(handlers::get_stored_addresses)),
            )
            .service(
                web::scope("/api/v0/token")
                    .route("/mint", web::post().to(handlers::mint_tokens))
                    .route("/mint-to", web::post().to(handlers::mint_to_address))
                    .route("/burn", web::post().to(handlers::burn_tokens))
                    .route(
                        "/balance/{address}",
                        web::get().to(handlers::get_token_balance),
                    ),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
