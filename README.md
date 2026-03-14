# rust-pattern-viz

> Make AI code generation transparent with interactive decision tree visualizations

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that visualizes the decision-making process behind AI-generated code. It parses Rust source files containing embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations that show what imports were considered, confidence scores per line, and the complete reasoning DAG. Think of it as an X-ray machine for AI copilot behavior—perfect for code reviews, PR comments, and building trust in AI-assisted development.

## Features

- **Decision Tree Visualization** - See the complete reasoning path: what the AI considered, rejected, and why
- **Confidence Heatmaps** - Line-by-line confidence scores for AI-generated code
- **Import Analysis** - Track which imports were evaluated and the rationale behind selections
- **Interactive Reports** - Export to SVG/HTML for embedding in PRs and documentation
- **Rust-Native Performance** - Fast parsing with `tree-sitter` and zero-overhead abstractions
- **Local-First** - All processing happens on your machine—no cloud uploads required
- **CLI & Library** - Use as a standalone tool or integrate into your Rust projects

## Quick Start

### Installation

```bash
cargo install rust-pattern-viz
```

Or build from source:

```bash
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

**Analyze a single file:**

```bash
rust-pattern-viz analyze examples/sample.rs --output report.html
```

**Generate an SVG decision tree:**

```bash
rust-pattern-viz visualize src/main.rs --format svg --output decision-tree.svg
```

**Use as a library:**

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = Analyzer::new();
    let patterns = analyzer.parse_file("src/main.rs")?;
    
    let visualizer = Visualizer::new();
    visualizer.generate_html(&patterns, "report.html")?;
    
    Ok(())
}
```

## Usage Examples

### Code Review Workflow

```bash
# Analyze AI-generated code before committing
rust-pattern-viz analyze src/new_feature.rs --output review.html

# Open the interactive report
open review.html
```

### CI/CD Integration

Add to your `.github/workflows/ci.yml`:

```yaml
- name: Visualize AI patterns
  run: |
    cargo install rust-pattern-viz
    rust-pattern-viz analyze src/ --output pattern-report.html
    
- name: Upload visualization
  uses: actions/upload-artifact@v3
  with:
    name: ai-pattern-report
    path: pattern-report.html
```

### Batch Analysis

```bash
# Analyze all Rust files in a directory
find src -name "*.rs" -exec rust-pattern-viz analyze {} --output reports/{}.html \;
```

## Tech Stack

- **Language:** Rust (2021 edition)
- **Parser:** [tree-sitter](https://tree-sitter.github.io/) for syntax analysis
- **Visualization:** SVG generation with embedded interactive JavaScript
- **CLI Framework:** `clap` for command-line argument parsing
- **Testing:** Built-in Rust test framework with integration tests

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Contributing:** Issues and pull requests welcome! See our [CI workflow](.github/workflows/ci.yml) for testing requirements.