# rust-pattern-viz

> Visualize AI code generation decision trees directly from Rust source code

## What is this?

**rust-pattern-viz** is a Rust-native CLI tool and library that makes AI copilot suggestions transparent and reviewable. It parses Rust source files with embedded AI metadata, extracts pattern matching logic and confidence scores, then generates interactive SVG/HTML visualizations showing the reasoning DAG behind every AI-suggested line. Perfect for understanding what your AI pair programmer was "thinking" during code reviews and PR discussions.

## Features

- **AI Decision Tree Extraction** — Parses embedded metadata from AI code generators to reconstruct the full decision graph
- **Confidence Score Visualization** — Shows per-line confidence levels and alternative suggestions considered
- **Interactive SVG/HTML Output** — Clickable diagrams with hover states revealing import alternatives and reasoning paths
- **Pattern Matching Analysis** — Extracts and visualizes Rust `match` expressions, `Option`/`Result` handling, and enum destructuring
- **Multi-Format Export** — Generate standalone SVGs, embeddable HTML widgets, or JSON for custom tooling
- **VS Code Extension** — In-editor hovers showing AI reasoning without leaving your workflow
- **Web Demo** — Browser-based WASM playground for experimenting without installation
- **LSP Server** — Real-time visualization updates as you code
- **Share Server** — Publish visualizations with unique URLs for async code review

## Quick Start

### Installation

```bash
# Via cargo
cargo install rust-pattern-viz

# Or build from source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a single file and generate SVG
rust-pattern-viz analyze examples/option_pattern.rs -o output.svg

# Generate interactive HTML
rust-pattern-viz analyze src/main.rs --format html -o viz.html

# Start web server for live preview
rust-pattern-viz serve --watch src/

# Launch LSP server for editor integration
rust-pattern-viz lsp --port 9257
```

### Library Usage

```rust
use rust_pattern_viz::{Analyzer, SvgRenderer};

let analyzer = Analyzer::new();
let patterns = analyzer.parse_file("examples/nested_match.rs")?;
let renderer = SvgRenderer::default();
let svg = renderer.render(&patterns)?;

std::fs::write("output.svg", svg)?;
```

## Examples

The `examples/` directory contains annotated Rust files demonstrating:

- **Option unwrapping patterns** (`01_option_unwrapping.rs`) — Shows AI choosing between `match`, `if let`, and `unwrap_or`
- **Result error handling** (`02_result_error_handling.rs`) — Visualizes propagation vs. handling trade-offs
- **Struct destructuring** (`03_struct_destructuring.rs`) — Confidence scores for field extraction strategies
- **Enum matching** (`04_enum_matching.rs`) — Import suggestions and exhaustiveness reasoning
- **Nested patterns** (`05_nested_patterns.rs`) — Multi-level decision DAGs

Run `cargo run --example svg_export` to generate all visualizations at once.

## VS Code Extension

Install from the marketplace or build locally:

```bash
cd vscode-extension
npm install
npm run compile
code --install-extension rust-pattern-viz-*.vsix
```

Hover over any `match` expression to see the AI reasoning tooltip inline.

## Web Demo

Try it live at **[https://rust-pattern-viz.demo](#)** or run locally:

```bash
cd web-demo
npm install
npm run dev
```

Paste Rust code, see instant visualizations powered by WASM.

## Tech Stack

- **Core**: Rust 2021 edition with `syn` for AST parsing
- **Visualization**: SVG generation with embedded JavaScript for interactivity
- **Web Server**: `axum` for HTTP endpoints and WebSocket live updates
- **LSP**: `tower-lsp` for editor integration protocol
- **WASM**: `wasm-bindgen` for browser deployment
- **Frontend**: React + TypeScript + Vite (web demo)
- **VS Code Extension**: TypeScript with Language Server Protocol client

## CI/CD

- **GitHub Actions** automated testing on push (`.github/workflows/ci.yml`)
- **Demo deployment** to GitHub Pages on release (`.github/workflows/deploy-demo.yml`)
- **Clippy + rustfmt** enforced for all PRs

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, architecture decisions ([ARCHITECTURE.md](ARCHITECTURE.md)), and recording demo videos ([DEMO_RECORDING_GUIDE.md](docs/DEMO_RECORDING_GUIDE.md)).

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Tip**: Combine with `cargo-llm-audit` to overlay LLM-generated code markers automatically before visualization.