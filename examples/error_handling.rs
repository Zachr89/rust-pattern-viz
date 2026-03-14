// Example: Idiomatic Rust error handling patterns

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    Parse(String),
    Validation(String),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io(err)
    }
}

// Pattern 1: ? operator for early return
fn read_config_file(path: &Path) -> Result<String, ConfigError> {
    let mut file = File::open(path)?; // Automatic error propagation
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Pattern 2: Explicit match for error handling
fn parse_config(contents: &str) -> Result<Config, ConfigError> {
    match contents.parse::<Config>() {
        Ok(config) => {
            // Nested validation
            if config.is_valid() {
                Ok(config)
            } else {
                Err(ConfigError::Validation("Invalid configuration".to_string()))
            }
        }
        Err(e) => Err(ConfigError::Parse(format!("Parse error: {}", e))),
    }
}

// Pattern 3: unwrap_or_else for fallback values
fn get_timeout(config: &Config) -> u64 {
    config
        .timeout
        .unwrap_or_else(|| {
            eprintln!("No timeout configured, using default");
            30
        })
}

// Pattern 4: if let for optional error handling
fn log_error_details(err: &ConfigError) {
    if let ConfigError::Io(io_err) = err {
        eprintln!("IO Error details: {:?}", io_err.kind());
        eprintln!("Consider checking file permissions");
    }
}

// Pattern 5: Result combinator chains
fn load_and_validate(path: &Path) -> Result<Config, ConfigError> {
    read_config_file(path)
        .and_then(|contents| parse_config(&contents))
        .map(|config| {
            println!("Configuration loaded successfully");
            config
        })
        .or_else(|err| {
            eprintln!("Error loading config: {:?}", err);
            Err(err)
        })
}

#[derive(Debug)]
struct Config {
    timeout: Option<u64>,
    retries: u32,
}

impl Config {
    fn is_valid(&self) -> bool {
        self.retries <= 10
    }
}

impl std::str::FromStr for Config {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simplified parsing
        Ok(Config {
            timeout: Some(30),
            retries: 3,
        })
    }
}

fn main() {
    let path = Path::new("config.toml");
    
    match load_and_validate(path) {
        Ok(config) => {
            println!("Configuration: {:?}", config);
            let timeout = get_timeout(&config);
            println!("Using timeout: {}s", timeout);
        }
        Err(err) => {
            log_error_details(&err);
            std::process::exit(1);
        }
    }
}
