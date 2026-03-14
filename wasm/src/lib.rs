use wasm_bindgen::prelude::*;
use rpv_core::{CodeAnalyzer, AnalysisReport};
use serde_json;

#[wasm_bindgen]
pub fn analyze_code(source: &str) -> Result<String, JsValue> {
    let analyzer = CodeAnalyzer::new();
    
    match analyzer.analyze(source) {
        Ok(report) => {
            serde_json::to_string(&report)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        }
        Err(e) => Err(JsValue::from_str(&format!("Analysis error: {}", e)))
    }
}

#[wasm_bindgen]
pub struct HardPattern {
    name: String,
    description: String,
    code: String,
}

#[wasm_bindgen]
impl HardPattern {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.clone()
    }
}

const HARD_PATTERNS: [(&str, &str, &str); 5] = [
    (
        "Nested Option<Result<T,E>>",
        "The classic 'option of result' unwrapping challenge - demonstrates nested pattern matching",
        r#"fn process_response(response: Option<Result<String, String>>) -> String {
    match response {
        Some(Ok(data)) => format!("Success: {}", data),
        Some(Err(e)) => format!("Error: {}", e),
        None => "No response".to_string(),
    }
}

fn handle_api_call() {
    let result = Some(Ok("data".to_string()));
    println!("{}", process_response(result));
}"#
    ),
    (
        "Multi-level Enum Destructuring",
        "Deep pattern matching with guards on complex enum hierarchies",
        r#"enum Message {
    Request { id: u32, payload: Payload },
    Response { id: u32, status: Status },
}

enum Payload {
    Text(String),
    Binary(Vec<u8>),
}

enum Status {
    Ok(String),
    Error { code: u16, message: String },
}

fn handle_message(msg: Message) -> String {
    match msg {
        Message::Request { id, payload: Payload::Text(text) } if id > 100 => {
            format!("High priority text request {}: {}", id, text)
        }
        Message::Request { id, payload: Payload::Binary(data) } if data.len() > 1024 => {
            format!("Large binary request {}: {} bytes", id, data.len())
        }
        Message::Response { id, status: Status::Ok(result) } => {
            format!("Success response {}: {}", id, result)
        }
        Message::Response { id, status: Status::Error { code, message } } => {
            format!("Error response {}: [{}] {}", id, code, message)
        }
        _ => "Other message type".to_string(),
    }
}"#
    ),
    (
        "Slice Patterns with Rest",
        "Array and slice pattern matching using the rest pattern (..)",
        r#"fn analyze_sequence(numbers: &[i32]) -> String {
    match numbers {
        [] => "Empty sequence".to_string(),
        [single] => format!("Single element: {}", single),
        [first, second] => format!("Pair: {} and {}", first, second),
        [first, .., last] if first == last => {
            format!("Palindrome-like: starts and ends with {}", first)
        }
        [1, rest @ .., 9] => {
            format!("Special sequence: 1..9 with {} middle elements", rest.len())
        }
        [first, middle @ .., last] => {
            format!("Sequence: {} ... {} (length {})", first, last, middle.len() + 2)
        }
    }
}

fn demo_slices() {
    println!("{}", analyze_sequence(&[1, 2, 3, 4, 9]));
    println!("{}", analyze_sequence(&[5, 10, 15, 5]));
}"#
    ),
    (
        "Tuple Struct Destructuring",
        "Pattern matching on newtype patterns and tuple structs",
        r#"struct UserId(u32);
struct Email(String);
struct PhoneNumber(String);

enum Contact {
    User(UserId, Email),
    Guest(Email),
    Anonymous,
    Admin(UserId, Email, PhoneNumber),
}

fn format_contact(contact: Contact) -> String {
    match contact {
        Contact::User(UserId(id), Email(email)) if id < 1000 => {
            format!("Early user #{}: {}", id, email)
        }
        Contact::User(UserId(id), Email(email)) => {
            format!("User #{}: {}", id, email)
        }
        Contact::Admin(UserId(id), Email(email), PhoneNumber(phone)) => {
            format!("Admin #{}: {} / {}", id, email, phone)
        }
        Contact::Guest(Email(email)) => {
            format!("Guest: {}", email)
        }
        Contact::Anonymous => "Anonymous user".to_string(),
    }
}

fn demo_contacts() {
    let contact = Contact::User(UserId(42), Email("user@example.com".to_string()));
    println!("{}", format_contact(contact));
}"#
    ),
    (
        "Guard Combinations with OR Patterns",
        "Complex guards combined with OR patterns for sophisticated matching logic",
        r#"enum Task {
    Pending { priority: u8 },
    Running { progress: f32, priority: u8 },
    Completed { duration_ms: u64 },
    Failed { error: String, retry_count: u32 },
}

fn should_escalate(task: &Task) -> bool {
    match task {
        Task::Pending { priority } | Task::Running { priority, .. } 
            if *priority > 7 => true,
        
        Task::Running { progress, priority } 
            if *progress < 0.1 && *priority > 5 => true,
        
        Task::Failed { retry_count, .. } 
            if *retry_count > 3 => true,
        
        Task::Completed { duration_ms } 
            if *duration_ms > 60_000 => true,
        
        _ => false,
    }
}

fn categorize_task(task: Task) -> String {
    match task {
        Task::Pending { priority: p } | Task::Running { priority: p, .. } 
            if p > 8 => "Critical".to_string(),
        
        Task::Failed { error, retry_count } 
            if error.contains("network") && retry_count < 5 => {
            format!("Retriable network issue (attempt {})", retry_count + 1)
        }
        
        _ => "Normal".to_string(),
    }
}"#
    ),
];

#[wasm_bindgen]
pub fn get_hard_patterns() -> Vec<JsValue> {
    HARD_PATTERNS
        .iter()
        .map(|(name, description, code)| {
            let pattern = HardPattern {
                name: name.to_string(),
                description: description.to_string(),
                code: code.to_string(),
            };
            JsValue::from(pattern)
        })
        .collect()
}
