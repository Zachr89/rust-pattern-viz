// Example: Option<T> pattern matching
fn process_user_input(input: Option<String>) -> String {
    match input {
        Some(text) => {
            if text.is_empty() {
                "Empty input provided".to_string()
            } else {
                format!("Processing: {}", text)
            }
        }
        None => "No input provided".to_string(),
    }
}
