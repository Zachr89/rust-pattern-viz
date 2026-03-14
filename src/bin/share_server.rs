//! Standalone share server binary.
//!
//! Runs a web server that provides the share API for analysis results.

use clap::Parser;
use rust_pattern_viz::ShareServer;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rpv-share-server")]
#[command(about = "Rust Pattern Viz - Share Server", long_about = None)]
struct Args {
    /// Port to run the server on
    #[arg(short, long, default_value = "3030")]
    port: u16,

    /// Directory to store shared analyses
    #[arg(short, long, default_value = ".rpv-shares")]
    storage_dir: PathBuf,

    /// Base URL for share links (e.g., https://example.com)
    #[arg(short, long, default_value = "http://localhost:3030")]
    base_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Starting Rust Pattern Viz Share Server");
    println!("Storage directory: {}", args.storage_dir.display());
    println!("Base URL: {}", args.base_url);

    let server = ShareServer::new(args.storage_dir, args.base_url)?;
    server.run(args.port).await;

    Ok(())
}
