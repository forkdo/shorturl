// routes.rs - API endpoint definitions and handlers

use crate::{
    db::{get_url, save_url},
    models::{ShortenRequest, ShortenResponse, UrlEntry},
    utils::generate_short_code,
};
use axum::{
    Json, Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use sqlx::SqlitePool;
use std::sync::Arc;

/// Handler for URL shortening endpoint
///
/// Accepts a JSON payload with the URL to shorten, generates a short code,
/// saves the mapping to the database, and returns the shortened URL.
///
/// # Arguments
/// * `State(pool)` - Database connection pool
/// * `Json(payload)` - Request payload containing the URL to shorten
///
/// # Returns
/// A `ShortenResponse` containing the shortened URL and its associated short code
///
/// # Errors
/// If there is an error while writing to the database,
/// an error message is logged to stderr using `eprintln!`.
async fn shorten_url(
    State((pool, site_url)): State<(Arc<SqlitePool>, String)>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, axum::http::StatusCode> {
    // Generate a unique short code
    let short_code = generate_short_code();

    // Create the complete shortened URL
    let short_url = format!("{}/{}", site_url, short_code);

    // Save the URL mapping to the database
    match save_url(&pool, &short_code, &payload.url).await {
        Ok(_) => Ok(Json(ShortenResponse {
            short_code,
            short_url,
        })),
        Err(e) => {
            eprintln!("DB Error: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Handler for URL resolution endpoint
///
/// Looks up the original URL for a given short code and returns it.
///
/// # Arguments
/// * `State((pool, _))` - Database connection pool
/// * `Path(short_code)` - The short code from the URL path
///
/// # Returns
/// A `Json` containing the resolved URL entry, if found
async fn resolve_url(
    State((pool, _)): State<(Arc<SqlitePool>, String)>,
    axum::extract::Path(short_code): axum::extract::Path<String>,
) -> Json<Option<UrlEntry>> {
    // Look up the URL in the database
    let result = get_url(&pool, &short_code).await.ok().flatten();

    // Return the URL entry if found
    Json(result)
}

/// Redirect a shortened URL to its original URL
///
/// This function attempts to look up the original URL in the database using
/// the provided short code and performs a redirect to that URL. If the short
/// code is not found or an error occurs during the lookup, it redirects to the
/// root URL with a 307 status code.
///
/// # Arguments
/// * `State(pool)` - The database connection pool wrapped in an `Arc`
/// * `Path(short_code)` - The short code extracted from the URL path
///
/// # Returns
/// A `Redirect` response that points to the original URL if found, or the root URL
/// otherwise. A 307 status code is used for redirection.
async fn redirect_url(
    State((pool, _)): State<(Arc<SqlitePool>, String)>,
    axum::extract::Path(short_code): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Attempt to retrieve the original URL from the database using the short code
    match get_url(&pool, &short_code).await {
        // If found, redirect temporarily to the original URL
        Ok(Some(url_entry)) => Redirect::temporary(&url_entry.original_url),
        // If not found, redirect temporarily to the root URL
        Ok(None) => Redirect::temporary("/404"),
        // If an error occurs, also redirect temporarily to the root URL
        Err(_) => Redirect::temporary("/404"),
    }
}

/// Create and configure the application router
///
/// Sets up all API routes and attaches the database connection pool.
///
/// # Arguments
/// * `pool` - Shared database connection pool
/// * `site_url` - The URL of the site that will be used to build the shortened URLs
///
/// # Returns
/// A configured application router with all routes and the database connection pool
pub fn create_router(pool: Arc<SqlitePool>, site_url: String) -> Router {
    Router::new()
        // Root route that returns a friendly message
        .route("/", get(|| async { "Short URL Service" }))
        .route("/404", get(|| async { "Short URL Not Found" }))
        // Route for creating shortened URLs
        .route("/shorten", post(shorten_url))
        // Route for resolving short codes to original URLs
        .route("/get/{short_code}", get(resolve_url))
        // Route for redirecting to the original URL
        .route("/{short_code}", get(redirect_url))
        // Attach the database connection pool to all routes
        .with_state((pool, site_url))
}
