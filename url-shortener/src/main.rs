// main.rs - Main application entry point for the URL shortener service

use axum::Router;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::net::TcpListener;
use dotenvy::dotenv;

// Import application modules
mod routes;
mod db;
mod models;
mod utils;

/// Application entry point
/// 
/// Sets up the database connection, creates the web server,
/// and starts listening for requests.
#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize the database connection pool
    let pool = db::init_db().await;
    
    // Share the pool across threads using Arc
    let shared_pool = Arc::new(pool);
    
    // Create the application router with routes
    let app = routes::create_router(shared_pool.clone());
    
    // Bind the server to the specified address and port
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    // Log that the server is running
    println!("🚀 Server running on http://localhost:3000");
    
    // Start the server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}