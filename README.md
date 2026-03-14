# rust-pattern-viz

**Visualize AI code generation decision trees for Rust projects**

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI copilot behavior transparent. It parses Rust source files with embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing imports considered, per-line confidence scores, and reasoning DAGs. Ideal for code reviews, PR comments, and understanding how AI assistants make decisions.

## Features

- **Parse AI metadata** from Rust source comments or sidecar files
- **Extract decision trees** showing what alternatives the AI considered
- **Generate interactive visualizations** (SVG + HTML) with zoom/pan controls
- **LSP server integration** for real-time IDE insights
- **VS Code extension** for seamless editor integration
- **Confidence scoring** to highlight uncertain suggestions
- **Export formats**: SVG, HTML, JSON, DOT (Graphviz)
- **CLI and library** APIs for flexible integration

## Quick Start

### Installation

```bash
# Install via cargo
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

# Generate visualization
rust-pattern-viz visualize src/main.rs --output decision-tree.html

# Start LSP server for IDE integration
rust-pattern-viz lsp
```

### VS Code Extension

1. Install the extension from `vscode-extension/`
2. Open a Rust project
3. Right-click any file → "Visualize AI Patterns"
4. View decision trees inline or export to HTML

## Usage Examples

### CLI

```bash
# Analyze with custom output format
rust-pattern-viz analyze src/lib.rs --format json > analysis.json

# Generate SVG with confidence threshold
rust-pattern-viz visualize src/analyzer.rs --format svg --min-confidence 0.8

# Batch process directory
rust-pattern-viz analyze src/ --recursive --output reports/
```

### Library

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

// Parse source with AI metadata
let analyzer = Analyzer::new();
let decisions = analyzer.parse_file("src/main.rs")?;

// Generate visualization
let visualizer = Visualizer::new();
let html = visualizer.render_html(&decisions)?;
std::fs::write("output.html", html)?;
```

### Embedding AI Metadata

Add comments to your Rust code:

```rust
// @ai-suggestion: considered std::collections::HashMap, confidence: 0.95
// @ai-alternative: hashbrown::HashMap, confidence: 0.78
use std::collections::HashMap;

// @ai-pattern: builder pattern, confidence: 0.88
// @ai-reasoning: mutable state, method chaining
pub struct Config {
    // ...
}
```

## Tech Stack

- **Core**: Rust 2021 edition
- **Parsing**: `syn`, `quote` for Rust AST manipulation
- **Visualization**: `svg` crate, embedded JavaScript for interactivity
- **LSP**: `tower-lsp` for Language Server Protocol
- **VS Code Extension**: TypeScript + VS Code API
- **CLI**: `clap` for argument parsing
- **Testing**: `cargo test` with integration tests

## Development

```bash
# Run tests
cargo test

# Run with examples
cargo run --example sample

# Build VS Code extension
cd vscode-extension
npm install
npm run compile
```

## CI/CD

GitHub Actions workflow (`.github/workflows/ci.yml`) automatically:
- Runs tests on push/PR
- Builds for Linux, macOS, Windows
- Publishes releases to crates.io
- Packages VS Code extension

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Contributing**: Issues and PRs welcome! See [ARCHITECTURE.md](ARCHITECTURE.md) for codebase overview.