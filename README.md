# rust-pattern-viz

*Visualize AI code generation decisions in your Rust codebase*

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI copilot behavior transparent. It parses Rust source files with embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing which imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for code reviews, PR comments, and understanding how AI assistants make decisions.

## Features

- **Pattern Analysis** - Parses Rust source files to extract pattern matching structures and AI decision metadata
- **Interactive Visualizations** - Generates SVG/HTML diagrams showing confidence scores, alternative suggestions, and reasoning paths
- **Multiple Interfaces** - CLI tool, Rust library, LSP server, VS Code extension, and web demo
- **Decision Trees** - Visualizes the complete DAG of AI reasoning, including rejected alternatives
- **Code Review Integration** - Embeddable outputs for GitHub PR comments and documentation
- **Real-time Analysis** - LSP server provides hover tooltips and inline diagnostics in your editor
- **WASM Support** - Run visualizations in the browser with zero backend dependencies
- **Sharing** - Built-in server for sharing visualizations with teammates

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
# Analyze a Rust file and generate an SVG visualization
rust-pattern-viz analyze examples/option_pattern.rs --output diagram.svg

# Start the web server for interactive exploration
rust-pattern-viz serve --port 8080

# Launch the LSP server for editor integration
rust-pattern-viz lsp

# Share a visualization
rust-pattern-viz share diagram.svg --expires 24h
```

### VS Code Extension

```bash
cd vscode-extension
npm install
npm run compile
# Press F5 in VS Code to launch extension development host
```

## Usage Examples

### CLI Analysis

```rust
// examples/option_pattern.rs with AI metadata
fn process_value(opt: Option<i32>) -> i32 {
    // @ai-suggestion: confidence=0.95, alternatives=[unwrap_or_default, if-let]
    match opt {
        Some(x) => x * 2,
        None => 0,
    }
}
```

```bash
rust-pattern-viz analyze examples/option_pattern.rs --format svg
```

Generates an interactive diagram showing:
- 95% confidence in using `match` over `unwrap_or_default`
- Visual decision tree with alternative paths
- Line-by-line reasoning annotations

### Library Integration

```rust
use rust_pattern_viz::{Analyzer, SvgRenderer};

let analyzer = Analyzer::new();
let patterns = analyzer.analyze_file("src/main.rs")?;

let renderer = SvgRenderer::new();
let svg = renderer.render(&patterns)?;
std::fs::write("output.svg", svg)?;
```

### Web Demo

```bash
cd web-demo
npm install
npm run dev
# Open http://localhost:5173
```

Paste Rust code with AI metadata to see real-time visualizations in the browser.

## Tech Stack

- **Core**: Rust (syn parser, petgraph for DAG, resvg for rendering)
- **CLI**: clap for argument parsing
- **Web Server**: axum with WebSocket support
- **LSP**: tower-lsp for editor integration
- **WASM**: wasm-bindgen + wasm-pack for browser builds
- **Frontend**: TypeScript + React + Vite
- **VS Code Extension**: TypeScript + vscode-languageclient
- **Visualization**: SVG with embedded JavaScript for interactivity

## Project Structure

```
rust-pattern-viz/
├── src/
│   ├── analyzer.rs       # Rust AST parsing and pattern extraction
│   ├── models.rs         # Data structures for decision trees
│   ├── svg_renderer.rs   # SVG generation with confidence scores
│   ├── visualizer.rs     # High-level visualization API
│   ├── lsp_server.rs     # Language Server Protocol implementation
│   ├── web_server.rs     # HTTP server for interactive demo
│   └── wasm.rs           # WebAssembly bindings
├── examples/             # Sample Rust files with AI metadata
├── vscode-extension/     # VS Code integration
├── web-demo/             # React-based web interface
└── docs/                 # Documentation and demos
```

## Documentation

- [Architecture Overview](ARCHITECTURE.md) - System design and component interactions
- [Contributing Guide](CONTRIBUTING.md) - Development setup and contribution guidelines
- [Demo Recording](DEMO_RECORDING.md) - Video walkthrough of key features
- [Creating Demos](docs/create-demo.md) - How to generate example visualizations

## License

MIT License - see [LICENSE](LICENSE) for details

---

**Note**: This tool is designed for analyzing AI-generated code suggestions. It requires source files to include AI metadata comments (format documented in ARCHITECTURE.md). Integration with popular AI coding assistants is planned for future releases.