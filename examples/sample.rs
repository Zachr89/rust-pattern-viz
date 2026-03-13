use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

/// Sample Rust file with various patterns for demonstration
#[derive(Debug)]
pub struct DataProcessor {
    cache: HashMap<String, Vec<u8>>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Demonstrates error handling pattern
    pub fn load_file(&mut self, path: &str) -> Result<String> {
        let content = fs::read_to_string(path)
            .context("Failed to read file")?;
        
        Ok(content)
    }

    /// Demonstrates iterator chain pattern
    pub fn process_data(&self, input: Vec<i32>) -> Vec<i32> {
        input
            .iter()
            .filter(|&&x| x > 0)
            .map(|&x| x * 2)
            .collect()
    }

    /// Demonstrates pattern matching
    pub fn handle_result(&self, result: Result<String>) -> String {
        match result {
            Ok(value) => format!("Success: {}", value),
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Demonstrates async pattern
    pub async fn fetch_data(&self, url: &str) -> Result<String> {
        // Simulated async operation
        Ok(format!("Data from {}", url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let processor = DataProcessor::new();
        let input = vec![-1, 2, -3, 4];
        let output = processor.process_data(input);
        assert_eq!(output, vec![4, 8]);
    }
}
