// models.rs - Data structures for the URL shortener application

use serde::{Serialize, Deserialize};

/// Request structure for URL shortening endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenRequest {
    /// The original URL to be shortened
    pub url: String,
}

/// Response structure for URL shortening endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenResponse {
    /// The complete shortened URL (including domain)
    pub short_url: String,
}

/// Database entity representing a shortened URL mapping
#[derive(Debug, Serialize, Deserialize)]
pub struct UrlEntry {
    /// The unique short code for this URL
    pub short_code: String,
    /// The original URL that was shortened
    pub original_url: String,
}