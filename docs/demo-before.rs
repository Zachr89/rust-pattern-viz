// Real-world example: HTTP handler with nested error handling
// Complexity Score: 12 (HIGH)

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User { id: u64, name: String, email: String }

async fn handle_user_request(id: u64) -> Option<User> {
    // ❌ Problem: 4 levels of nested matches
    match fetch_user_data(id).await {
        Ok(response) => match response.status() {
            200..=299 => match response.json::<User>().await {
                Ok(user) => match validate_user(&user) {
                    Ok(()) => Some(user),
                    Err(e) => {
                        // Only this error path logs context
                        eprintln!("Validation failed: {}", e);
                        None
                    }
                },
                // Silent error: no context about why parsing failed
                Err(_) => None
            },
            // Silent error: no context about HTTP status
            _ => None
        },
        // Silent error: no context about network failure
        Err(_) => None
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

// Simplified Response type for demo
struct Response { status: u16, body: String }
impl Response {
    fn status(&self) -> u16 { self.status }
    async fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }
}
