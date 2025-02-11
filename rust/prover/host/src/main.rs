use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use sqlx::SqlitePool;

// use anyhow::{Context, Result};
use ciborium::into_writer;
use hex::encode;
use methods::{ZKLEADERBOARD_GUEST_ELF, ZKLEADERBOARD_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use tokio::sync::{mpsc, Mutex};

// Configuration
const DB_PATH: &str = "/Users/nishantpandav/Documents/work/_startups/Blockchain Gods/game/hackathons/zkVerify hackathon 2/zkLeaderboard-app/rust/backend/data/addresses.db";
const SERVER_ADDR: (&str, u16) = ("127.0.0.1", 8080);

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressData {
    pub address: String,
    pub score: i64,
}
#[derive(Serialize, Deserialize)]
struct Output {
    address: String,
    is_top_half: bool,
}

#[derive(Debug, Deserialize)]
struct PositionRequest {
    addresses: Vec<String>,
}

#[derive(Debug, Serialize)]
struct JobStatus {
    status: String,
    proof: Option<String>,
    journal: Option<String>,
    image_id: Option<String>,
    results: Option<Vec<AddressResult>>,
}

#[derive(Debug, Serialize)]
struct AddressResult {
    address: String,
    is_top_half: bool,
}
struct AppState {
    jobs: Mutex<HashMap<String, JobStatus>>,
    tx: mpsc::Sender<(String, Vec<String>)>,
    db_pool: SqlitePool,
}

mod handlers {
    use super::*;

    pub async fn check_position(
        state: web::Data<AppState>,
        req: web::Json<PositionRequest>,
    ) -> Result<HttpResponse> {
        let job_id = uuid::Uuid::new_v4().to_string();

        let mut jobs = state.jobs.lock().await;
        jobs.insert(
            job_id.clone(),
            JobStatus {
                status: "pending".into(),
                proof: None,
                journal: None,
                image_id: None,
                results: None,
            },
        );

        state
            .tx
            .send((job_id.clone(), req.addresses.clone()))
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        Ok(HttpResponse::Ok().json(job_id))
    }

    pub async fn get_job_status(
        state: web::Data<AppState>,
        job_id: web::Path<String>,
    ) -> HttpResponse {
        let jobs = state.jobs.lock().await;
        match jobs.get(&job_id.into_inner()) {
            Some(status) => HttpResponse::Ok().json(status),
            None => HttpResponse::NotFound().finish(),
        }
    }
}

async fn generate_proof(
    addresses: Vec<String>,
    db_pool: &SqlitePool,
) -> std::io::Result<JobStatus> {
    let scores = fetch_scores(db_pool, &addresses)
        .await
        .expect("Failed to fetch scores");

    let env = ExecutorEnv::builder()
        .write(&scores)
        .unwrap()
        .build()
        .expect("Failed to build executor environment");

    let prover = default_prover();
    let receipt = prover.prove(env, ZKLEADERBOARD_GUEST_ELF).unwrap().receipt;

    let results: Vec<Output> = receipt.journal.decode().unwrap();
    let address_results: Vec<AddressResult> = results
        .into_iter()
        .map(|output| AddressResult {
            address: output.address,
            is_top_half: output.is_top_half,
        })
        .collect();
    // receipt
    //     .verify(ZKLEADERBOARD_GUEST_ID)
    //     .context("Proof verification failed")?;

    let mut bin_receipt = Vec::new();
    into_writer(&receipt, &mut bin_receipt).unwrap();
    let proof = encode(&bin_receipt);

    fs::write("proof.txt", hex::encode(&bin_receipt)).unwrap();
    let receipt_journal_bytes_array = &receipt.journal.bytes.as_slice();
    let pub_inputs = hex::encode(&receipt_journal_bytes_array);

    let image_id_hex = hex::encode(
        ZKLEADERBOARD_GUEST_ID
            .into_iter()
            .flat_map(|v| v.to_le_bytes().into_iter())
            .collect::<Vec<_>>(),
    );

    // let image_id_bytes: Vec<u8> = ZKLEADERBOARD_GUEST_ID
    //     .iter()
    //     .flat_map(|v| v.to_le_bytes())
    //     .collect();

    Ok(JobStatus {
        status: "completed".into(),
        proof: Some("0x".to_owned() + &proof),
        journal: Some("0x".to_owned() + &pub_inputs),
        image_id: Some("0x".to_owned() + &image_id_hex),
        results: Some(address_results),
    })
}

async fn fetch_scores(pool: &SqlitePool, addresses: &[String]) -> anyhow::Result<Vec<AddressData>> {
    let median = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT score FROM addresses
        ORDER BY score
        LIMIT 1
        OFFSET (SELECT COUNT(*) FROM addresses) / 2
        "#,
    )
    .fetch_one(pool)
    .await?;

    print!("Median:{}", median);
    // Fetch scores for requested addresses
    let mut address_data = Vec::with_capacity(addresses.len());
    for addr in addresses {
        if let Ok(score) =
            sqlx::query_scalar::<_, i64>("SELECT score FROM addresses WHERE address = ?")
                .bind(addr)
                .fetch_optional(pool)
                .await
        {
            if let Some(score) = score {
                address_data.push(AddressData {
                    address: addr.clone(),
                    score,
                });
            }
        }
    }

    Ok(address_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();

    let db_pool = SqlitePool::connect(&format!("sqlite:{}", DB_PATH))
        .await
        .expect("Failed to connect to database");

    let (tx, mut rx) = mpsc::channel::<(String, Vec<String>)>(32);
    let state = web::Data::new(AppState {
        jobs: Mutex::new(HashMap::new()),
        tx,
        db_pool: db_pool.clone(),
    });

    let state_clone = state.clone();
    tokio::spawn(async move {
        while let Some((job_id, addresses)) = rx.recv().await {
            match generate_proof(addresses, &state_clone.db_pool).await {
                Ok(proof) => {
                    state_clone.jobs.lock().await.insert(job_id, proof);
                }
                Err(e) => {
                    println!("Error generating proof: {}", e);
                }
            }
        }
    });

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .service(
                web::scope("/check_position").route("/", web::post().to(handlers::check_position)),
            )
            .service(web::scope("/job").route("/{job_id}", web::get().to(handlers::get_job_status)))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await?;

    Ok(())
}
