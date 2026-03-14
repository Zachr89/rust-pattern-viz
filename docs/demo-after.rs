// Refactored: Flat error handling with explicit context
// Complexity Score: 4 (GOOD)

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User { id: u64, name: String, email: String }

async fn handle_user_request(id: u64) -> Result<User, HandlerError> {
    // ✅ Each step is clear, testable, and logged
    let response = fetch_user_data(id).await
        .map_err(|e| HandlerError::Network(format!("Failed to fetch user {}: {}", id, e)))?;

    let user = response
        .error_for_status()
        .map_err(|status| HandlerError::Http(status, "Non-2xx status".into()))?
        .json::<User>().await
        .map_err(|e| HandlerError::Parse(format!("Invalid JSON: {}", e)))?;

    validate_user(&user)
        .map(|_| user)
        .map_err(|e| HandlerError::Validation(e))
}

// Explicit error type with context
#[derive(Debug)]
enum HandlerError {
    Network(String),
    Http(u16, String),
    Parse(String),
    Validation(String),
}

impl std::fmt::Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandlerError::Network(msg) => write!(f, "Network error: {}", msg),
            HandlerError::Http(status, msg) => write!(f, "HTTP {}: {}", status, msg),
            HandlerError::Parse(msg) => write!(f, "Parse error: {}", msg),
            HandlerError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

// Mock implementations for demo
async fn fetch_user_data(id: u64) -> Result<Response, reqwest::Error> {
    reqwest::get(&format!("https://api.example.com/users/{}", id)).await
}

fn validate_user(user: &User) -> Result<(), String> {
    if user.email.contains('@') { Ok(()) } 
    else { Err("Invalid email".to_string()) }
}

// Enhanced Response type with error_for_status
struct Response { status: u16, body: String }
impl Response {
    fn error_for_status(self) -> Result<Self, u16> {
        if (200..300).contains(&self.status) { Ok(self) } 
        else { Err(self.status) }
    }
    async fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }
}
