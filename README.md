# rust-pattern-viz

> Visualize AI code generation decision trees from Rust source files

## What is this?

**rust-pattern-viz** is a Rust-native CLI tool and library that makes AI copilot behavior transparent. It parses Rust source files containing embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations. See what imports were considered, review confidence scores per line, and explore the complete reasoning DAG—perfect for code reviews, PR comments, and debugging AI-generated code.

## Features

- 🔍 **Pattern Analysis** - Extracts match expressions, enum variants, and control flow from Rust source
- 📊 **Decision Tree Visualization** - Generates interactive SVG/HTML diagrams showing AI reasoning paths
- 🎯 **Confidence Scoring** - Displays per-line confidence metrics from AI suggestion metadata
- 🌐 **Multi-Format Export** - Output as SVG, HTML with embedded interactivity, or JSON
- 🚀 **LSP Integration** - Built-in language server for IDE hover tooltips and inline visualizations
- 🌍 **Web Demo** - Interactive browser-based playground (WASM-powered)
- 🔌 **VS Code Extension** - First-class editor integration with one-click visualization
- 📤 **Share Server** - Generate shareable URLs for code review discussions

## Quick Start

### Installation

```bash
# Install via cargo
cargo install rust-pattern-viz

# Or clone and build from source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a single file
rust-pattern-viz analyze src/main.rs

# Generate SVG visualization
rust-pattern-viz visualize examples/option_pattern.rs -o output.svg

# Start the web server for interactive exploration
rust-pattern-viz serve --port 3000

# Launch LSP server for IDE integration
rust-pattern-viz lsp
```

## Usage Examples

### CLI Analysis

```bash
# Analyze pattern matching with confidence scores
rust-pattern-viz analyze --show-confidence examples/nested_match.rs

# Export to HTML with interactive features
rust-pattern-viz visualize examples/result_pattern.rs \
  --format html \
  --interactive \
  -o report.html
```

### Library Usage

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

// Parse and analyze
let analyzer = Analyzer::new();
let patterns = analyzer.analyze_file("src/main.rs")?;

// Generate visualization
let visualizer = Visualizer::new();
let svg = visualizer.render_svg(&patterns)?;
std::fs::write("output.svg", svg)?;
```

### VS Code Extension

1. Install from marketplace: `rust-pattern-viz`
2. Open any `.rs` file
3. Right-click → "Visualize AI Patterns"
4. View inline diagram or export to file

### Web Demo

Visit the [interactive playground](https://rust-pattern-viz.dev/demo) to try it in your browser—no installation required.

## Architecture

- **CLI** (`src/main.rs`) - Command-line interface for batch processing
- **Analyzer** (`src/analyzer.rs`) - AST parsing and pattern extraction
- **Visualizer** (`src/visualizer.rs`) - DAG construction and layout engine
- **SVG Renderer** (`src/svg_renderer.rs`) - Graph-to-SVG conversion with theming
- **LSP Server** (`src/lsp_server.rs`) - Language server protocol implementation
- **Web Server** (`src/web_server.rs`) - HTTP API for browser-based visualization
- **Share Server** (`src/share.rs`) - URL shortening and persistence layer
- **WASM Module** (`src/wasm.rs`) - Browser runtime compilation

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design decisions.

## Tech Stack

- **Core**: Rust 2021 edition
- **Parsing**: `syn` (Rust AST), `quote` (code generation)
- **Rendering**: `svg` crate, custom layout algorithms
- **Web**: `axum` (HTTP server), `tower` (middleware)
- **LSP**: `tower-lsp`, `lsp-types`
- **WASM**: `wasm-bindgen`, `wasm-pack`
- **VS Code**: TypeScript, VS Code Extension API
- **Demo**: Vite, React, TypeScript

## Examples

Explore the [`examples/`](examples/) directory for sample Rust files with visualizations:

- `01_option_unwrapping.rs` - Basic `Option<T>` handling patterns
- `02_result_error_handling.rs` - `Result<T, E>` with custom error types
- `03_struct_destructuring.rs` - Nested struct pattern matching
- `04_enum_matching.rs` - Complex enum variants with guards
- `05_nested_patterns.rs` - Multi-level match expressions with AI reasoning

Each example includes a generated `.svg` visualization.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with ❤️ for transparent AI-assisted development**