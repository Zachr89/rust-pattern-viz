use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rust_pattern_viz::CodeAnalyzer;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rpv")]
#[command(about = "Rust Pattern Visualizer - Analyze and visualize Rust code patterns", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a Rust source file
    Analyze {
        /// Path to the Rust source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        
        /// Output format (json or text)
        #[arg(short, long, default_value = "text")]
        format: String,
        
        /// Output file path (optional, defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file, format, output } => {
            analyze_command(file, format, output)?;
        }
    }

    Ok(())
}

fn analyze_command(file: PathBuf, format: String, output: Option<PathBuf>) -> Result<()> {
    // Read source file
    let source = fs::read_to_string(&file)
        .with_context(|| format!("Failed to read file: {}", file.display()))?;
    
    // Analyze
    let analyzer = CodeAnalyzer::new();
    let report = analyzer.analyze(&source, file.to_str().unwrap())?;
    
    // Format output
    let output_str = match format.as_str() {
        "json" => serde_json::to_string_pretty(&report)?,
        "text" => format_text_report(&report),
        _ => anyhow::bail!("Unknown format: {}", format),
    };
    
    // Write output
    match output {
        Some(path) => {
            fs::write(&path, output_str)
                .with_context(|| format!("Failed to write output to: {}", path.display()))?;
            println!("Analysis written to: {}", path.display());
        }
        None => {
            println!("{}", output_str);
        }
    }
    
    Ok(())
}

fn format_text_report(report: &rust_pattern_viz::AnalysisReport) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("=== Analysis Report for {} ===\n", report.file_path));
    output.push_str(&format!("Timestamp: {}\n", report.timestamp));
    output.push_str(&format!("Overall Confidence: {:.2}%\n\n", report.overall_confidence * 100.0));
    
    output.push_str("=== Detected Patterns ===\n");
    for pattern in &report.patterns {
        output.push_str(&format!(
            "- {} (lines {}-{}): {:.0}% confidence\n",
            pattern.pattern_type,
            pattern.start_line,
            pattern.end_line,
            pattern.confidence * 100.0
        ));
        if let Some(reasoning) = &pattern.reasoning {
            output.push_str(&format!("  Reasoning: {}\n", reasoning));
        }
    }
    
    if !report.decision_nodes.is_empty() {
        output.push_str("\n=== Decision Tree ===\n");
        for decision in &report.decision_nodes {
            output.push_str(&format!(
                "- {:?}: {} (confidence: {:.0}%)\n",
                decision.decision_type,
                decision.description,
                decision.confidence * 100.0
            ));
            output.push_str(&format!("  Chosen: {}\n", decision.chosen));
        }
    }
    
    output
}
