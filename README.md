# rust-pattern-viz

*Visualize AI code generation decision trees directly in your editor*

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI copilot behavior transparent. It parses Rust source files with embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing what imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for code reviews, PR comments, and understanding how AI assistants make decisions.

## Features

- **Parse AI metadata** from Rust source files with embedded suggestions
- **Extract decision trees** showing imports, alternatives, and confidence scores
- **Generate interactive visualizations** as SVG/HTML for easy sharing
- **CLI tool** for batch processing and CI/CD integration
- **Library API** for programmatic access and custom tooling
- **LSP server** for real-time editor integration
- **VS Code extension** for inline visualization and hover tooltips
- **CI/CD ready** with GitHub Actions support

## Quick Start

### Installation

**CLI tool:**
```bash
cargo install rust-pattern-viz
```

**As a library:**
```toml
[dependencies]
rust-pattern-viz = "0.1"
```

**VS Code extension:**
1. Download `.vsix` from releases
2. Run `code --install-extension rust-pattern-viz-0.1.0.vsix`
3. Reload VS Code

### Usage

**CLI:**
```bash
# Analyze a single file
rust-pattern-viz analyze src/main.rs --output viz.html

# Batch process a directory
rust-pattern-viz analyze src/ --format svg --output-dir ./visualizations

# Generate JSON report for CI
rust-pattern-viz analyze src/ --format json > report.json
```

**Library:**
```rust
use rust_pattern_viz::{Analyzer, Visualizer};

let analyzer = Analyzer::new();
let decisions = analyzer.parse_file("src/main.rs")?;

let visualizer = Visualizer::new();
let html = visualizer.render_html(&decisions)?;
std::fs::write("output.html", html)?;
```

**VS Code:**
1. Open any Rust file with AI metadata comments
2. Hover over annotated code to see decision info
3. Run command `Rust Pattern Viz: Show Visualization` (Ctrl+Shift+P)
4. View interactive DAG in the side panel

## Examples

Check `examples/sample.rs` for a file with embedded AI metadata:

```rust
// @ai-suggestion: import std::collections::HashMap | confidence: 0.92
// @ai-alternatives: ["std::collections::BTreeMap (0.65)", "hashbrown::HashMap (0.48)"]
use std::collections::HashMap;
```

Run:
```bash
cargo run --example sample
```

## Tech Stack

- **Core:** Rust (CLI + library + LSP server)
- **Parsing:** `syn` for Rust AST analysis, custom comment parser
- **Visualization:** SVG generation with `resvg`, HTML templating
- **LSP:** `tower-lsp` for editor integration
- **VS Code Extension:** TypeScript, VS Code API
- **CI/CD:** GitHub Actions for testing and releases

## Development

```bash
# Build everything
cargo build --release

# Run tests
cargo test

# Build VS Code extension
cd vscode-extension && npm install && npm run compile

# Start LSP server for development
cargo run --bin lsp-server
```

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design docs including:
- Metadata format specification
- Decision tree data structures
- Visualization rendering pipeline
- LSP protocol implementation

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Contributing:** PRs welcome! Please open an issue first to discuss major changes.

**Support:** File issues on GitHub or reach out via Discussions.