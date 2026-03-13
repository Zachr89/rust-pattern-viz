use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use rust_pattern_viz::{
    analyzer::CodeAnalyzer,
    models::{AnalysisReport, VisualizationFormat},
    visualizer::Visualizer,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rpv")]
#[command(about = "Rust Pattern Visualizer - Visualize AI code generation patterns", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a Rust source file for AI patterns
    Analyze {
        /// Path to the Rust source file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output format (json, html, svg)
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Show detailed confidence scores
        #[arg(short, long)]
        verbose: bool,
    },

    /// Visualize an existing analysis report
    Visualize {
        /// Path to JSON analysis report
        #[arg(value_name = "REPORT")]
        input: PathBuf,

        /// Output format (html, svg)
        #[arg(short, long, default_value = "html")]
        format: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Batch analyze a directory of Rust files
    Batch {
        /// Directory containing Rust files
        #[arg(value_name = "DIR")]
        directory: PathBuf,

        /// Output directory for reports
        #[arg(short, long)]
        output: PathBuf,

        /// Format for output (json, html)
        #[arg(short, long, default_value = "html")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            input,
            format,
            output,
            verbose,
        } => {
            println!("{}", "🔍 Analyzing Rust code patterns...".cyan().bold());

            let source = std::fs::read_to_string(&input)
                .with_context(|| format!("Failed to read file: {}", input.display()))?;

            let analyzer = CodeAnalyzer::new();
            let report = analyzer
                .analyze(&source, input.to_str().unwrap())
                .context("Analysis failed")?;

            if verbose {
                print_detailed_report(&report);
            } else {
                print_summary(&report);
            }

            let viz_format = match format.as_str() {
                "json" => VisualizationFormat::Json,
                "html" => VisualizationFormat::Html,
                "svg" => VisualizationFormat::Svg,
                _ => anyhow::bail!("Unsupported format: {}", format),
            };

            let visualizer = Visualizer::new();
            let output_content = visualizer.render(&report, viz_format)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, output_content)
                    .with_context(|| format!("Failed to write output: {}", output_path.display()))?;
                println!(
                    "\n{} {}",
                    "✓ Output written to:".green().bold(),
                    output_path.display()
                );
            } else {
                println!("\n{}", output_content);
            }

            Ok(())
        }

        Commands::Visualize {
            input,
            format,
            output,
        } => {
            println!("{}", "🎨 Generating visualization...".cyan().bold());

            let report_json = std::fs::read_to_string(&input)
                .with_context(|| format!("Failed to read report: {}", input.display()))?;

            let report: AnalysisReport = serde_json::from_str(&report_json)
                .context("Failed to parse analysis report")?;

            let viz_format = match format.as_str() {
                "html" => VisualizationFormat::Html,
                "svg" => VisualizationFormat::Svg,
                _ => anyhow::bail!("Unsupported format for visualization: {}", format),
            };

            let visualizer = Visualizer::new();
            let output_content = visualizer.render(&report, viz_format)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, output_content)
                    .with_context(|| format!("Failed to write output: {}", output_path.display()))?;
                println!(
                    "{} {}",
                    "✓ Visualization saved to:".green().bold(),
                    output_path.display()
                );
            } else {
                println!("{}", output_content);
            }

            Ok(())
        }

        Commands::Batch {
            directory,
            output,
            format,
        } => {
            println!(
                "{}",
                "📁 Batch analyzing directory...".cyan().bold()
            );

            std::fs::create_dir_all(&output)
                .context("Failed to create output directory")?;

            let analyzer = CodeAnalyzer::new();
            let visualizer = Visualizer::new();
            let mut processed = 0;

            for entry in walkdir::WalkDir::new(&directory)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            {
                let path = entry.path();
                println!("  Processing: {}", path.display().to_string().dimmed());

                let source = match std::fs::read_to_string(path) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("  {} {}: {}", "⚠".yellow(), path.display(), e);
                        continue;
                    }
                };

                let report = match analyzer.analyze(&source, path.to_str().unwrap()) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("  {} {}: {}", "⚠".yellow(), path.display(), e);
                        continue;
                    }
                };

                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let output_file = output.join(format!("{}.{}", file_stem, format));

                let viz_format = match format.as_str() {
                    "json" => VisualizationFormat::Json,
                    "html" => VisualizationFormat::Html,
                    _ => VisualizationFormat::Json,
                };

                let output_content = visualizer.render(&report, viz_format)?;
                std::fs::write(&output_file, output_content)?;

                processed += 1;
            }

            println!(
                "\n{} {} files processed",
                "✓".green().bold(),
                processed
            );
            println!(
                "{} {}",
                "Reports saved to:".green(),
                output.display()
            );

            Ok(())
        }
    }
}

fn print_summary(report: &AnalysisReport) {
    println!("\n{}", "Analysis Summary:".bold());
    println!("  File: {}", report.file_path.cyan());
    println!("  Patterns detected: {}", report.patterns.len());
    println!("  Import suggestions: {}", report.import_suggestions.len());
    println!(
        "  Average confidence: {:.1}%",
        report.overall_confidence * 100.0
    );

    if !report.patterns.is_empty() {
        println!("\n{}", "Top Patterns:".bold());
        for (i, pattern) in report.patterns.iter().take(5).enumerate() {
            let confidence_color = if pattern.confidence > 0.8 {
                "green"
            } else if pattern.confidence > 0.5 {
                "yellow"
            } else {
                "red"
            };

            println!(
                "  {}. {} ({:.0}%) - Line {}",
                i + 1,
                pattern.pattern_type,
                pattern.confidence * 100.0,
                pattern.line_number
            );
            println!(
                "     {}",
                format!("▰".repeat((pattern.confidence * 20.0) as usize))
                    .color(confidence_color)
            );
        }
    }
}

fn print_detailed_report(report: &AnalysisReport) {
    print_summary(report);

    if !report.import_suggestions.is_empty() {
        println!("\n{}", "Import Decision Tree:".bold());
        for suggestion in &report.import_suggestions {
            println!("\n  Considered: {}", suggestion.import_path.cyan());
            println!("  Status: {}", suggestion.status);
            if let Some(reason) = &suggestion.reason {
                println!("  Reason: {}", reason.dimmed());
            }
            if !suggestion.alternatives.is_empty() {
                println!("  Alternatives:");
                for alt in &suggestion.alternatives {
                    println!("    - {}", alt.dimmed());
                }
            }
        }
    }

    if !report.decision_nodes.is_empty() {
        println!("\n{}", "Decision Graph:".bold());
        for node in &report.decision_nodes {
            println!(
                "\n  {} (confidence: {:.0}%)",
                node.description,
                node.confidence * 100.0
            );
            if !node.children.is_empty() {
                println!("  Leads to:");
                for child in &node.children {
                    println!("    → {}", child.dimmed());
                }
            }
        }
    }
}
