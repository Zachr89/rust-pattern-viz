# rust-pattern-viz

**Visualize AI code generation decision trees in Rust projects**

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI-generated code transparent and reviewable. It parses Rust source files containing embedded AI suggestion metadata, extracts pattern matching logic and confidence scores, then generates interactive SVG/HTML visualizations showing the reasoning behind each code decision. Whether you're reviewing a PR with AI-generated code or trying to understand your copilot's suggestions, this tool provides a visual DAG of what was considered, rejected, and why.

## Features

- **Parse AI metadata** from Rust source files (comments, attributes, or embedded JSON)
- **Extract pattern matching logic** including imports, confidence scores per line, and decision trees
- **Generate interactive SVGs** with hover tooltips showing reasoning paths
- **Export to HTML** with embedded visualizations for easy sharing
- **CLI and library** – use standalone or integrate into your build pipeline
- **LSP server** for real-time visualization in your editor
- **VS Code extension** with inline hover previews
- **WebAssembly support** for browser-based demos
- **Web demo** with live examples and interactive editing

## Quick Start

### Installation

```bash
# Install from crates.io
cargo install rust-pattern-viz

# Or clone and build from source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a Rust file and generate SVG
rust-pattern-viz analyze src/main.rs --output decision-tree.svg

# Generate HTML with embedded visualization
rust-pattern-viz analyze src/main.rs --format html --output report.html

# Start the web server for interactive exploration
rust-pattern-viz serve --port 8080

# Run the LSP server for editor integration
rust-pattern-viz lsp
```

### As a Library

```rust
use rust_pattern_viz::{Analyzer, Visualizer, SvgRenderer};

// Parse a Rust file with AI metadata
let analyzer = Analyzer::new();
let analysis = analyzer.analyze_file("examples/option_pattern.rs")?;

// Generate visualization
let visualizer = Visualizer::new();
let dag = visualizer.build_dag(&analysis);

// Render to SVG
let renderer = SvgRenderer::new();
let svg = renderer.render(&dag)?;
std::fs::write("output.svg", svg)?;
```

## Examples

The `examples/` directory contains annotated Rust files demonstrating common AI-assisted patterns:

- **Option unwrapping** – Shows alternative handling strategies with confidence scores
- **Result error handling** – Visualizes error propagation decisions
- **Struct destructuring** – Maps field access patterns and their reasoning
- **Enum matching** – Decision trees for match arm selection
- **Nested patterns** – Complex multi-level pattern analysis

Generate all example visualizations:

```bash
cargo run --example svg_export
```

View them in `examples/*.svg` or browse the interactive web demo at `web-demo/`.

## VS Code Extension

Install the extension from the `vscode-extension/` directory:

```bash
cd vscode-extension
npm install
npm run compile
code --install-extension .
```

Hover over pattern matches in Rust files to see inline AI reasoning visualizations.

## Web Demo

Try it live without installation:

```bash
cd web-demo
npm install
npm run dev
```

Open http://localhost:5173 to explore examples interactively in your browser.

## Tech Stack

- **Core**: Rust with `syn` for parsing, `petgraph` for DAG construction
- **Rendering**: Custom SVG generator with CSS animations
- **CLI**: `clap` for argument parsing
- **LSP**: Tower-LSP for editor integration
- **Web**: Actix-web server + WASM bindings
- **Frontend**: React + TypeScript + Vite
- **CI/CD**: GitHub Actions for testing and demo deployment

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, architecture overview, and contribution guidelines. Check [ARCHITECTURE.md](ARCHITECTURE.md) for system design details.

## License

MIT License – see [LICENSE](LICENSE) for details.

---

**Built for transparency in AI-assisted development.** Star this repo if you believe code reviewers deserve to see the "why" behind AI suggestions.