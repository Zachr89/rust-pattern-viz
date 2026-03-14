//! Example Rust code for demonstrating pattern matching visualization
//! Use this file when recording the demo GIF

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process_message(msg: Message) -> String {
    // Hover over this match expression to see the visualization
    match msg {
        Message::Quit => {
            println!("Quitting application");
            "quit".to_string()
        }
        Message::Move { x, y } => {
            println!("Moving to ({}, {})", x, y);
            format!("move_{}_{}", x, y)
        }
        Message::Write(text) => {
            println!("Writing: {}", text);
            text
        }
        Message::ChangeColor(r, g, b) => {
            println!("Changing color to RGB({}, {}, {})", r, g, b);
            format!("color_{}_{}_{}",  r, g, b)
        }
    }
}

// Example with Option - common pattern matching use case
fn get_user_name(user_id: Option<u32>) -> String {
    // Hover here to see pattern matching with Option
    match user_id {
        Some(id) if id > 0 => format!("User #{}", id),
        Some(_) => "Invalid user ID".to_string(),
        None => "Anonymous".to_string(),
    }
}

// Example with Result - error handling pattern
fn parse_config(input: &str) -> Result<u32, String> {
    input.parse::<u32>()
        .map_err(|e| format!("Parse error: {}", e))
}

fn handle_config(input: &str) -> String {
    // Hover here to see Result pattern matching
    match parse_config(input) {
        Ok(value) => format!("Config value: {}", value),
        Err(e) => format!("Error: {}", e),
    }
}

fn main() {
    // Demo examples - use these in the recording
    let msg = Message::Move { x: 10, y: 20 };
    process_message(msg);
    
    let user = Some(42);
    get_user_name(user);
    
    handle_config("123");
}
