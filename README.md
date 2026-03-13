# rust-pattern-viz

**Make AI code generation decisions visible.**

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that transforms opaque AI code suggestions into transparent, visual decision trees. It parses Rust source files with embedded AI metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations that show which imports were considered, confidence scores per line, and the complete reasoning DAG. Built for developers who want to understand and validate AI copilot behavior during code reviews.

## Features

- **Decision Tree Extraction** – Parses AI suggestion metadata from Rust source files
- **Pattern Analysis** – Identifies import decisions, rejected alternatives, and confidence scores
- **Interactive Visualizations** – Generates SVG and HTML reports showing reasoning DAGs
- **PR-Ready Output** – Export visualizations for GitHub PR comments and code reviews
- **Library + CLI** – Use as a standalone tool or integrate into your Rust projects
- **Local Processing** – All analysis happens on your machine—no cloud uploads
- **Fast & Lightweight** – Single binary with minimal dependencies

## Quick Start

### Installation

```bash
# Install from crates.io
cargo install rust-pattern-viz

# Or build from source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a single file
rust-pattern-viz analyze src/main.rs

# Generate HTML report
rust-pattern-viz analyze src/main.rs --output report.html

# Generate SVG decision tree
rust-pattern-viz analyze src/main.rs --format svg --output tree.svg

# Analyze entire project
rust-pattern-viz analyze src/ --recursive
```

## Usage Examples

### CLI Analysis

```bash
# Analyze AI-generated code with confidence thresholds
rust-pattern-viz analyze ai_generated.rs --min-confidence 0.7

# Export for PR comments
rust-pattern-viz analyze src/lib.rs --format html --output pr-comment.html
```

### Library Integration

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

fn main() {
    // Parse source file
    let analyzer = Analyzer::new();
    let decisions = analyzer.parse_file("src/main.rs")?;
    
    // Generate visualization
    let visualizer = Visualizer::new();
    visualizer.render_html(&decisions, "output.html")?;
    
    // Access decision metadata
    for decision in decisions.iter() {
        println!("Line {}: confidence {}", 
                 decision.line, 
                 decision.confidence);
    }
}
```

### Example Output

The tool generates interactive visualizations showing:
- **Import candidates** considered by the AI
- **Confidence scores** for each suggestion (0.0 - 1.0)
- **Pattern matches** that influenced decisions
- **Alternative paths** that were rejected
- **Reasoning chains** from input to output

See `examples/sample.rs` for annotated source files.

## Tech Stack

- **Rust** – Core analysis engine and CLI
- **tree-sitter** – Robust Rust source parsing
- **SVG/HTML Templates** – Portable visualization output
- **Serde** – Metadata serialization
- **Clap** – Command-line interface

## CI/CD

GitHub Actions workflow included for:
- Automated testing on push
- Multi-platform builds (Linux, macOS, Windows)
- Cargo fmt and clippy checks

## Contributing

Contributions welcome! Please open an issue before starting major work.

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built for developers who demand transparency in AI-assisted coding.**