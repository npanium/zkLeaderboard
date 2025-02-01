use crate::models::AddressScore;
use rand::{rng, Rng};
use sqlx::SqlitePool;

pub fn generate_address() -> String {
    let mut rng = rng();
    let random_bytes: Vec<u8> = (0..20).map(|_| rng.random()).collect();
    format!("0x{}", hex::encode(random_bytes))
}

pub fn generate_score() -> u32 {
    let mut rng = rng();
    rng.random_range(100..=1000)
}

pub fn generate_addresses(count: u32) -> Vec<AddressScore> {
    (0..count)
        .map(|_| AddressScore {
            id: None,
            address: generate_address(),
            score: generate_score(),
            created_at: None,
        })
        .collect()
}

pub async fn generate_and_store_addresses(
    pool: &SqlitePool,
    count: u32,
) -> Result<Vec<AddressScore>, anyhow::Error> {
    let addresses = generate_addresses(count);
    let mut stored_addresses = Vec::with_capacity(addresses.len());

    for addr in addresses {
        let stored = sqlx::query!(
            r#"
            INSERT INTO addresses (address, score)
            VALUES (?, ?)
            RETURNING id, address, score, created_at
            "#,
            addr.address,
            addr.score
        )
        .fetch_one(pool)
        .await?;

        stored_addresses.push(AddressScore {
            id: Some(stored.id),
            address: stored.address,
            score: stored.score as u32,
            created_at: Some(chrono::DateTime::from_naive_utc_and_offset(
                stored
                    .created_at
                    .expect("Time created not found for stored"),
                chrono::Utc,
            )),
        });
    }

    Ok(stored_addresses)
}

pub async fn get_stored_addresses(
    pool: &SqlitePool,
    page: u32,
    per_page: u32,
) -> Result<Vec<AddressScore>, anyhow::Error> {
    let offset = (page - 1) * per_page;

    let addresses = sqlx::query!(
        r#"
        SELECT id, address, score, created_at
        FROM addresses
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(addresses
        .into_iter()
        .map(|row| AddressScore {
            id: Some(row.id),
            address: row.address,
            score: row.score as u32,
            created_at: Some(chrono::DateTime::from_naive_utc_and_offset(
                row.created_at.expect("Time created not found for row"),
                chrono::Utc,
            )),
        })
        .collect())
}

pub async fn get_all_addresses(pool: &SqlitePool) -> Result<Vec<AddressScore>, anyhow::Error> {
    let addresses = sqlx::query!(
        r#"
        SELECT id, address, score, created_at
        FROM addresses
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(addresses
        .into_iter()
        .map(|row| AddressScore {
            id: Some(row.id),
            address: row.address,
            score: row.score as u32,
            created_at: Some(chrono::DateTime::from_naive_utc_and_offset(
                row.created_at.expect("Time created not found for row"),
                chrono::Utc,
            )),
        })
        .collect())
}
