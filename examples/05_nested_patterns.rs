//! Example: Nested Pattern Matching
//! 
//! Demonstrates complex nested patterns for real-world data structures.

#[derive(Debug)]
enum ApiResponse {
    Success { data: Data, metadata: Metadata },
    Error { code: u16, message: String },
    Redirect { location: String },
}

#[derive(Debug)]
struct Data {
    items: Vec<Item>,
    total: u32,
}

#[derive(Debug)]
struct Item {
    id: u32,
    status: Status,
}

#[derive(Debug)]
enum Status {
    Active,
    Pending,
    Inactive { reason: String },
}

#[derive(Debug)]
struct Metadata {
    version: String,
    cached: bool,
}

fn process_response(response: ApiResponse) -> String {
    // Pattern 1: Deep nested matching
    match response {
        ApiResponse::Success {
            data: Data { items, total },
            metadata: Metadata { version, cached },
        } => {
            let active_count = items
                .iter()
                .filter(|item| matches!(item.status, Status::Active))
                .count();
            
            format!(
                "Success: {} total items, {} active (v{}, cached: {})",
                total, active_count, version, cached
            )
        }
        ApiResponse::Error { code, message } if code >= 500 => {
            format!("Server error {}: {}", code, message)
        }
        ApiResponse::Error { code, message } => {
            format!("Client error {}: {}", code, message)
        }
        ApiResponse::Redirect { location } => {
            format!("Redirect to: {}", location)
        }
    }
}

fn find_inactive_items(response: &ApiResponse) -> Vec<String> {
    // Pattern 2: Nested pattern with vector matching
    match response {
        ApiResponse::Success { data: Data { items, .. }, .. } => {
            items
                .iter()
                .filter_map(|item| match &item.status {
                    Status::Inactive { reason } => {
                        Some(format!("Item {}: {}", item.id, reason))
                    }
                    _ => None,
                })
                .collect()
        }
        _ => vec![],
    }
}

fn is_cached_success(response: &ApiResponse) -> bool {
    // Pattern 3: Nested pattern with boolean logic
    matches!(
        response,
        ApiResponse::Success {
            metadata: Metadata { cached: true, .. },
            ..
        }
    )
}

#[derive(Debug)]
enum ConfigValue {
    String(String),
    Number(i32),
    List(Vec<ConfigValue>),
    Object(Vec<(String, ConfigValue)>),
}

fn flatten_config(value: ConfigValue) -> Vec<String> {
    // Pattern 4: Recursive nested patterns
    match value {
        ConfigValue::String(s) => vec![s],
        ConfigValue::Number(n) => vec![n.to_string()],
        ConfigValue::List(items) => {
            items.into_iter().flat_map(flatten_config).collect()
        }
        ConfigValue::Object(pairs) => {
            pairs
                .into_iter()
                .flat_map(|(key, val)| {
                    let mut result = vec![key];
                    result.extend(flatten_config(val));
                    result
                })
                .collect()
        }
    }
}

fn main() {
    let response = ApiResponse::Success {
        data: Data {
            items: vec![
                Item { id: 1, status: Status::Active },
                Item { id: 2, status: Status::Inactive { reason: "Expired".to_string() } },
            ],
            total: 2,
        },
        metadata: Metadata {
            version: "1.0".to_string(),
            cached: true,
        },
    };
    
    println!("{}", process_response(response));
    
    let config = ConfigValue::Object(vec![
        ("name".to_string(), ConfigValue::String("app".to_string())),
        ("port".to_string(), ConfigValue::Number(8080)),
    ]);
    println!("Config values: {:?}", flatten_config(config));
}
