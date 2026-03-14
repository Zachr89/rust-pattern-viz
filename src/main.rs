use clap::{Parser, ValueEnum};
use rust_pattern_viz::{CodeAnalyzer, SvgRenderer};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    /// JSON format (default)
    Json,
    /// Markdown format
    Markdown,
    /// SVG diagram format
    Svg,
}

#[derive(Parser)]
#[command(name = "rpv")]
#[command(about = "Rust Pattern Visualizer - Analyze and visualize Rust code patterns")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Analyze a Rust source file
    Analyze {
        /// Path to the Rust source file to analyze
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output format
        #[arg(short, long, value_enum, default_value = "json")]
        output_format: OutputFormat,

        /// Write output to file instead of stdout
        #[arg(short = 'o', long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Pretty-print JSON output
        #[arg(short, long)]
        pretty: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            file,
            output_format,
            output,
            pretty,
        } => {
            // Read source file
            let source = fs::read_to_string(&file)?;
            let file_path = file.to_string_lossy().to_string();

            // Analyze
            let analyzer = CodeAnalyzer::new();
            let report = analyzer.analyze(&source, &file_path)?;

            // Format output
            let output_content = match output_format {
                OutputFormat::Json => {
                    if pretty {
                        serde_json::to_string_pretty(&report)?
                    } else {
                        serde_json::to_string(&report)?
                    }
                }
                OutputFormat::Markdown => {
                    format_markdown(&report)
                }
                OutputFormat::Svg => {
                    let renderer = SvgRenderer::new();
                    renderer.render(&report)
                }
            };

            // Write output
            if let Some(output_path) = output {
                fs::write(output_path, output_content)?;
            } else {
                io::stdout().write_all(output_content.as_bytes())?;
                io::stdout().write_all(b"\n")?;
            }

            Ok(())
        }
    }
}

fn format_markdown(report: &rust_pattern_viz::AnalysisReport) -> String {
    let mut md = String::new();

    md.push_str(&format!("# Analysis Report: {}\n\n", report.file_path));
    md.push_str(&format!("**Generated:** {}\n\n", report.timestamp));
    md.push_str(&format!("**Overall Confidence:** {:.1}%\n\n", report.overall_confidence * 100.0));

    if !report.patterns.is_empty() {
        md.push_str("## Detected Patterns\n\n");
        for pattern in &report.patterns {
            md.push_str(&format!(
                "### {} (lines {}-{})\n\n",
                pattern.pattern_type, pattern.start_line, pattern.end_line
            ));
            md.push_str(&format!("**Confidence:** {:.1}%\n\n", pattern.confidence * 100.0));
            if let Some(reasoning) = &pattern.reasoning {
                md.push_str(&format!("**Reasoning:** {}\n\n", reasoning));
            }
        }
    }

    if !report.decision_nodes.is_empty() {
        md.push_str("## Decision Points\n\n");
        for node in &report.decision_nodes {
            md.push_str(&format!("### {:?}\n\n", node.decision_type));
            md.push_str(&format!("**Description:** {}\n\n", node.description));
            md.push_str(&format!("**Confidence:** {:.1}%\n\n", node.confidence * 100.0));
            md.push_str(&format!("**Chosen:** {}\n\n", node.chosen));
        }
    }

    if !report.import_suggestions.is_empty() {
        md.push_str("## Import Analysis\n\n");
        for import in &report.import_suggestions {
            md.push_str(&format!("- `{}` ({})\n", import.path, import.category));
        }
        md.push_str("\n");
    }

    md
}
