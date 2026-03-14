# rust-pattern-viz

> Visualize AI code generation decision trees directly from your Rust source code

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that transforms AI-assisted code into interactive visual diagrams. It parses Rust files with embedded AI suggestion metadata, extracts pattern matching logic, and generates beautiful SVG/HTML visualizations showing import decisions, confidence scores, and reasoning DAGs. Perfect for code reviews, PR comments, and understanding how AI copilots make suggestions.

## Features

- **Native Rust parsing** – Zero dependencies on external AST tools
- **AI metadata extraction** – Reads embedded confidence scores and decision trails
- **Interactive SVGs** – Hover tooltips, clickable nodes, zoom/pan support
- **Multiple output formats** – SVG, HTML, JSON for downstream tooling
- **LSP server integration** – Real-time visualization in your editor
- **VS Code extension** – Inline hover previews of decision trees
- **WASM web demo** – Try it in your browser without installation
- **CLI + library** – Use standalone or integrate into your Rust projects
- **Share server** – Generate public URLs for visualizations in PR comments

## Quick Start

### Installation

```bash
# Install from crates.io
cargo install rust-pattern-viz

# Or build from source
git clone https://github.com/yourusername/rust-pattern-viz
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Visualize a single file
rust-pattern-viz analyze examples/option_pattern.rs --output diagram.svg

# Process an entire project
rust-pattern-viz analyze src/ --recursive --format html

# Start the LSP server for editor integration
rust-pattern-viz lsp --stdio

# Launch interactive web demo
rust-pattern-viz web-demo --port 8080
```

## Usage Examples

### CLI Analysis

```bash
# Generate SVG with confidence threshold
rust-pattern-viz analyze src/main.rs --min-confidence 0.7 -o output.svg

# Export JSON for custom processing
rust-pattern-viz analyze src/ --format json > decisions.json

# Start share server for PR integration
rust-pattern-viz share-server --port 3000
```

### Library Integration

```rust
use rust_pattern_viz::{Analyzer, Visualizer, SvgRenderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse Rust file with AI metadata
    let analyzer = Analyzer::new();
    let patterns = analyzer.extract_patterns("src/main.rs")?;
    
    // Generate visualization
    let visualizer = Visualizer::new();
    let dag = visualizer.build_decision_graph(&patterns)?;
    
    // Render to SVG
    let renderer = SvgRenderer::new();
    let svg = renderer.render(&dag)?;
    std::fs::write("output.svg", svg)?;
    
    Ok(())
}
```

### VS Code Extension

1. Install from marketplace: `Rust Pattern Visualizer`
2. Hover over any pattern match expression
3. See inline decision tree with confidence scores
4. Click "Open Full Visualization" for interactive SVG

### WASM Web Demo

Visit the [live demo](https://your-demo-url.com) or run locally:

```bash
cd wasm/www
npm install
npm start
# Open http://localhost:8080
```

## Tech Stack

- **Core**: Rust 1.70+ with `syn` for AST parsing
- **Rendering**: SVG generation with `resvg` backend
- **Web**: Axum web server + WASM bindings (wasm-bindgen)
- **Editor Integration**: LSP server via `tower-lsp`
- **VS Code Extension**: TypeScript + Language Client
- **Web Demo**: React + Vite + WebAssembly
- **CI/CD**: GitHub Actions (test + deploy)

## Project Structure

```
├── src/               # Core library
│   ├── analyzer.rs    # Pattern extraction
│   ├── visualizer.rs  # DAG construction
│   ├── svg_renderer.rs # SVG generation
│   ├── lsp_server.rs  # Editor integration
│   └── web_server.rs  # HTTP API
├── examples/          # Sample visualizations
├── vscode-extension/  # VS Code plugin
├── wasm/              # WASM bindings
├── web-demo/          # React demo site
└── tests/             # Integration tests
```

## Documentation

- [Architecture Guide](ARCHITECTURE.md) – System design and data flow
- [Contributing Guide](CONTRIBUTING.md) – How to contribute
- [Demo Recording](DEMO_RECORDING.md) – Creating screencasts
- [Examples README](examples/README.md) – Sample outputs explained

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with ❤️ for developers who want to understand their AI copilots**