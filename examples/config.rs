// Example Rust code demonstrating nested pattern matching
// This file is analyzed by rust-pattern-viz to generate the README visualization

use std::path::PathBuf;

#[derive(Debug)]
enum ConnectionType {
    Postgres { host: String, port: u16 },
    Sqlite { path: PathBuf },
}

#[derive(Debug)]
struct Database {
    connection_type: ConnectionType,
    max_connections: usize,
}

#[derive(Debug)]
struct Config {
    database: Option<Database>,
    log_level: String,
}

#[derive(Debug)]
enum Error {
    ParseError(String),
    IoError(std::io::Error),
}

fn process_config(result: Result<Config, Error>) {
    match result {
        Ok(config) => {
            println!("Configuration loaded successfully");
            
            if let Some(database) = config.database {
                match database.connection_type {
                    ConnectionType::Postgres { host, port } => {
                        println!("Connecting to Postgres at {}:{}", host, port);
                        println!("Max connections: {}", database.max_connections);
                    }
                    ConnectionType::Sqlite { path } => {
                        println!("Using SQLite at {}", path.display());
                        println!("Max connections: {}", database.max_connections);
                    }
                }
            } else {
                println!("No database configuration found, using defaults");
            }
            
            println!("Log level: {}", config.log_level);
        }
        Err(e) => {
            eprintln!("Configuration error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let config = Config {
        database: Some(Database {
            connection_type: ConnectionType::Postgres {
                host: "localhost".to_string(),
                port: 5432,
            },
            max_connections: 10,
        }),
        log_level: "info".to_string(),
    };
    
    process_config(Ok(config));
}
