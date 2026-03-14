# rust-pattern-viz

> Visualize AI code generation decision trees in Rust

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI copilot behavior transparent. It parses Rust source files with embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing what imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for code reviews, PR comments, and understanding why your AI assistant made specific suggestions.

## Features

- **Parse AI metadata** from Rust source files with embedded suggestion comments
- **Extract decision trees** showing confidence scores, alternative imports, and reasoning chains
- **Generate interactive visualizations** as SVG, HTML, or both
- **CLI tool** for quick analysis from the command line
- **Library API** for embedding in your own tools
- **LSP server** for real-time hover tooltips in VS Code
- **Web demo** with WASM support for browser-based visualization
- **Share server** for generating shareable visualization links
- **VS Code extension** for in-editor AI decision inspection

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

### Usage

**CLI:**

```bash
# Analyze a Rust file and generate SVG
rust-pattern-viz analyze examples/sample.rs --output decision-tree.svg

# Generate interactive HTML
rust-pattern-viz analyze examples/sample.rs --format html --output viz.html

# Start the web server for live visualization
rust-pattern-viz serve --port 8080
```

**Library:**

```rust
use rust_pattern_viz::{Analyzer, SvgRenderer};

fn main() {
    // Parse a Rust file with AI metadata
    let analyzer = Analyzer::new();
    let decision_tree = analyzer.parse_file("src/main.rs").unwrap();
    
    // Generate SVG visualization
    let renderer = SvgRenderer::new();
    let svg = renderer.render(&decision_tree);
    std::fs::write("output.svg", svg).unwrap();
}
```

**VS Code Extension:**

1. Install the extension from `vscode-extension/`
2. Hover over any line with AI metadata comments
3. View confidence scores and alternative suggestions inline

**Web Demo:**

```bash
cd web-demo
npm install
npm run dev
```

Visit `http://localhost:5173` to try the interactive browser-based visualizer.

## Tech Stack

- **Core**: Rust with `syn` for parsing, `serde` for serialization
- **Visualization**: SVG generation with custom DAG layout algorithms
- **CLI**: `clap` for argument parsing
- **Web**: `axum` for HTTP server, WebAssembly (`wasm-bindgen`) for browser support
- **LSP**: `tower-lsp` for VS Code integration
- **Frontend**: TypeScript + React + Vite

## Project Structure

```
rust-pattern-viz/
├── src/
│   ├── analyzer.rs       # Parse Rust files and extract metadata
│   ├── models.rs         # Decision tree data structures
│   ├── svg_renderer.rs   # Generate SVG visualizations
│   ├── visualizer.rs     # High-level visualization API
│   ├── lsp_server.rs     # Language Server Protocol implementation
│   ├── web_server.rs     # HTTP server for live previews
│   ├── share.rs          # Shareable link generation
│   └── wasm.rs           # WebAssembly bindings
├── vscode-extension/     # VS Code extension
├── web-demo/             # Browser-based demo
├── examples/             # Usage examples
└── docs/                 # Documentation and demo materials
```

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - System design and component overview
- [CONTRIBUTING.md](CONTRIBUTING.md) - Development setup and guidelines
- [docs/create-demo.md](docs/create-demo.md) - How to create demo visualizations

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with ❤️ in Rust** | [Report Issues](https://github.com/yourusername/rust-pattern-viz/issues) | [Contribute](CONTRIBUTING.md)