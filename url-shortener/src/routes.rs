// routes.rs - API endpoint definitions and handlers

use axum::{extract::State, routing::get, Json, Router};
use sqlx::SqlitePool;
use std::sync::Arc;
use crate::{
    db::{save_url, get_url},
    models::{ShortenRequest, ShortenResponse, UrlEntry},
    utils::generate_short_code,
};

/// Handler for URL shortening endpoint
/// 
/// Accepts a JSON payload with the URL to shorten, generates a short code,
/// saves the mapping to the database, and returns the shortened URL.
/// 
/// # Arguments
/// * `State(pool)` - Database connection pool
/// * `Json(payload)` - Request payload containing the URL to shorten
async fn shorten_url(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<ShortenRequest>,
) -> Json<ShortenResponse> {
    // Generate a unique short code
    let short_code = generate_short_code();
    
    // Create the complete shortened URL
    let short_url = format!("http://localhost:3000/{}", short_code);
    
    // Save the URL mapping to the database
    if let Err(e) = save_url(&pool, &short_code, &payload.url).await {
        eprintln!("DB Error: {:?}", e);
    }
    
    // Return the shortened URL
    Json(ShortenResponse { short_url })
}

/// Handler for URL resolution endpoint
/// 
/// Looks up the original URL for a given short code and returns it.
/// 
/// # Arguments
/// * `State(pool)` - Database connection pool
/// * `Path(short_code)` - The short code from the URL path
async fn resolve_url(
    State(pool): State<Arc<SqlitePool>>,
    axum::extract::Path(short_code): axum::extract::Path<String>,
) -> Json<Option<UrlEntry>> {
    // Look up the URL in the database
    let result = get_url(&pool, &short_code).await.ok().flatten();
    
    // Return the URL entry if found
    Json(result)
}

/// Create and configure the application router
/// 
/// Sets up all API routes and attaches the database connection pool.
/// 
/// # Arguments
/// * `pool` - Shared database connection pool
pub fn create_router(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        // Route for creating shortened URLs
        .route("/shorten", axum::routing::post(shorten_url))
        // Route for resolving short codes to original URLs
        .route("/:short_code", get(resolve_url))
        // Attach the database connection pool to all routes
        .with_state(pool)
}