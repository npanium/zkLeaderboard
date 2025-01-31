use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

// pub const DATABASE_URL: &str = "sqlite:data/addresses.db";
// use crate::DATABASE_URL;

pub async fn init_db(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("{}?mode=rwc", database_url)) // Use `mode=rwc` to create the file if it doesn't exist
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS addresses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            address TEXT NOT NULL,
            score INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
