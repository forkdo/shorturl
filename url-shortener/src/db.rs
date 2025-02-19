// db.rs - Handles database operations for the URL shortener

use sqlx::{SqlitePool, query};
use crate::models::UrlEntry;

/// Initialize the database connection pool
/// 
/// Reads the DATABASE_URL from environment variables and
/// creates a connection pool to the SQLite database
pub async fn init_db() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&database_url).await.expect("Failed to connect to DB")
}

/// Save a new URL mapping to the database
/// 
/// # Arguments
/// * `pool` - SQLite connection pool
/// * `short_code` - The generated short code for the URL
/// * `original_url` - The original URL to be shortened
/// 
/// # Returns
/// * `Result<(), sqlx::Error>` - Success or database error
pub async fn save_url(pool: &SqlitePool, short_code: &str, original_url: &str) -> Result<(), sqlx::Error> {
    query("INSERT INTO urls (short_code, original_url) VALUES (?, ?)")
        .bind(short_code)
        .bind(original_url)
        .execute(pool)
        .await?;
    Ok(())
}

/// Retrieve the original URL for a given short code
/// 
/// # Arguments
/// * `pool` - SQLite connection pool
/// * `short_code` - The short code to look up
/// 
/// # Returns
/// * `Result<Option<UrlEntry>, sqlx::Error>` - The URL entry if found, or None
pub async fn get_url(pool: &SqlitePool, short_code: &str) -> Result<Option<UrlEntry>, sqlx::Error> {
    let result = query!("SELECT short_code, original_url FROM urls WHERE short_code = ?", short_code)
        .fetch_optional(pool)
        .await?;
    Ok(result.map(|row| UrlEntry {
        short_code: row.short_code,
        original_url: row.original_url,
    }))
}