// utils.rs - Utility functions for the URL shortener

use uuid::Uuid;

/// Generate a unique short code for a URL
///
/// Uses UUID v4 to create a unique identifier and truncates it
/// to the first 8 characters to create a short code.
///
/// # Returns
/// * `String` - A unique 8-character short code
pub fn generate_short_code() -> String {
    // Generate a UUID v4 and take the first 8 characters
    Uuid::new_v4().to_string()[..8].to_string()
}

// Note: Additional utility functions could be added here, such as:
// - URL validation
// - Rate limiting helpers
// - Analytics tracking
