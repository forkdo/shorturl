// main.rs - Main application entry point for the URL shortener service

use dotenvy::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;

// Import application modules
mod db;
mod models;
mod routes;
mod utils;

/// Application entry point
///
/// Sets up the database connection, creates the web server,
/// and starts listening for requests.
#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let site_url = std::env::var("SITE_URL").unwrap_or_else(|_| format!("http://{}", addr));

    // Initialize the database connection pool
    let pool = db::init_db().await;

    // Share the pool across threads using Arc
    let shared_pool = Arc::new(pool);

    // Create the application router with routes
    let app = routes::create_router(shared_pool.clone(), site_url.clone());

    println!("🚀 Server running on http://{}", addr);
    println!("🔗 Shortened URL: {}", site_url);

    // Bind the server to the specified address and port
    let listener = TcpListener::bind(&addr).await.unwrap();

    // Log that the server is running

    // Start the server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
