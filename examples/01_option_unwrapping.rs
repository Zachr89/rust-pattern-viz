//! Example: Option Pattern Matching
//! 
//! Demonstrates idiomatic Option<T> unwrapping and transformation patterns.

fn process_config(config: Option<String>) -> String {
    // Pattern 1: if let for simple unwrapping
    if let Some(cfg) = config {
        return format!("Config loaded: {}", cfg);
    }
    
    "Using default config".to_string()
}

fn find_user(id: u32) -> Option<User> {
    // Pattern 2: match for exhaustive handling
    match id {
        1 => Some(User { name: "Alice".to_string(), age: 30 }),
        2 => Some(User { name: "Bob".to_string(), age: 25 }),
        _ => None,
    }
}

fn get_user_name(id: u32) -> String {
    // Pattern 3: Chained Option methods with pattern matching
    match find_user(id) {
        Some(user) => user.name,
        None => "Guest".to_string(),
    }
}

struct User {
    name: String,
    age: u32,
}

fn main() {
    // Example usage
    println!("{}", process_config(Some("production.toml".to_string())));
    println!("{}", process_config(None));
    
    println!("User 1: {}", get_user_name(1));
    println!("User 99: {}", get_user_name(99));
}
