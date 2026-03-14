# rust-pattern-viz

*Make AI code generation decisions visible through interactive visualizations*

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that demystifies AI code generation by visualizing decision trees, pattern matching logic, and reasoning paths. It parses Rust source files with embedded AI suggestion metadata and generates interactive SVG/HTML visualizations showing what imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for code reviews, PR comments, and building trust in AI-assisted development.

## Features

- **Decision Tree Extraction** – Parse AI suggestion metadata from Rust source files
- **Interactive Visualizations** – Generate SVG/HTML graphs showing AI reasoning paths
- **Confidence Heatmaps** – See per-line confidence scores for AI-generated code
- **Import Analysis** – Track which imports were considered, chosen, or rejected
- **Reasoning DAG** – Full directed acyclic graph of decision points
- **CLI & Library** – Use as standalone tool or integrate into your Rust projects
- **PR-Ready Output** – Export visualizations for code review comments
- **Local Processing** – All analysis happens locally—no cloud uploads

## Quick Start

### Installation

```bash
cargo install rust-pattern-viz
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
rust-pattern-viz = "0.1"
```

### Usage

#### CLI

Analyze a Rust file with AI metadata:

```bash
rust-pattern-viz analyze src/main.rs --output report.html
```

Generate SVG visualization:

```bash
rust-pattern-viz visualize src/main.rs --format svg --output decision-tree.svg
```

#### Library

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse AI metadata from source
    let analyzer = Analyzer::new();
    let decisions = analyzer.analyze_file("src/main.rs")?;
    
    // Generate visualization
    let visualizer = Visualizer::new();
    visualizer.render_html(&decisions, "report.html")?;
    
    Ok(())
}
```

### Example Output

See `examples/sample.rs` for annotated source code with AI metadata, and run:

```bash
cargo run --example sample
```

This generates an interactive HTML report showing:
- Decision nodes for each AI suggestion
- Confidence scores and alternative options considered
- Import resolution paths
- Pattern matching logic

## Tech Stack

- **Rust** – Core language for parsing and analysis
- **tree-sitter** – Syntax tree parsing
- **SVG/HTML** – Visualization output formats
- **Serde** – Serialization for metadata extraction

## Development

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz

# Build
cargo build

# Run tests
cargo test

# Run integration tests
cargo test --test integration_test
```

## CI/CD

GitHub Actions workflow automatically:
- Runs tests on push
- Checks formatting and lints
- Builds release artifacts

## License

MIT License - see [LICENSE](LICENSE) for details.

---

*Built with 🦀 for developers who want to understand their AI copilots*