use actix_cors::Cors;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use db::init_db;
use log::info;
use std::env;

mod db;
mod handlers;
mod models;
mod services;

use dotenv::dotenv;
//const DATABASE_URL: &str = "sqlite://addresses.db?mode=rwc";

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

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Configure based on your needs

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .route("/addresses", web::get().to(handlers::get_addresses))
                    .route(
                        "/addresses/generate",
                        web::post().to(handlers::generate_and_store_addresses),
                    )
                    .route(
                        "/addresses/stored",
                        web::get().to(handlers::get_stored_addresses),
                    ),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
