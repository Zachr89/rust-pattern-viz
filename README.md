# rust-pattern-viz

> Visualize AI code generation decision trees directly from your Rust source code

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that transforms AI-assisted code into interactive visualizations. It parses embedded AI suggestion metadata from your Rust files, extracts pattern matching logic, and generates beautiful SVG/HTML diagrams showing import alternatives, per-line confidence scores, and the complete reasoning DAG. Perfect for code reviews, understanding AI copilot behavior, and documenting the decision-making process behind generated code.

## Features

- 🔍 **Smart Parsing** — Extracts AI suggestion metadata embedded in Rust source comments
- 🎨 **Interactive Visualizations** — Generates clean SVG/HTML diagrams with hover states and tooltips
- 📊 **Decision Tree Analysis** — Shows what imports were considered and why certain patterns were chosen
- 💯 **Confidence Scoring** — Displays AI confidence metrics per line and per decision node
- 🌐 **Multiple Interfaces** — CLI tool, Rust library, LSP server, VS Code extension, and web demo
- 🚀 **WebAssembly Support** — Run visualizations entirely in the browser with WASM compilation
- 🔗 **Shareable Reports** — Export diagrams for PR comments, documentation, or team reviews
- ⚡ **Real-time Preview** — Live visualization server with hot-reload during development

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

**Generate a visualization from a Rust file:**

```bash
rust-pattern-viz analyze examples/option_pattern.rs --output viz.svg
```

**Start the interactive web server:**

```bash
rust-pattern-viz serve --port 3000
# Open http://localhost:3000
```

**Use as a library:**

```rust
use rust_pattern_viz::{Analyzer, SvgRenderer};

let source = std::fs::read_to_string("main.rs")?;
let analyzer = Analyzer::new();
let tree = analyzer.parse(&source)?;
let renderer = SvgRenderer::new();
let svg = renderer.render(&tree)?;

std::fs::write("output.svg", svg)?;
```

**VS Code Integration:**

Install the `rust-pattern-viz` extension from the marketplace, then hover over any AI-generated code block to see inline visualizations.

## Examples

The `/examples` directory includes:

- **option_pattern.rs** — Basic Option<T> pattern matching analysis
- **result_pattern.rs** — Error handling decision trees
- **nested_match.rs** — Complex nested pattern visualization
- **error_handling.rs** — Multiple error propagation strategies
- **svg_export.rs** — Programmatic SVG generation examples

View pre-rendered diagrams in `/examples/diagrams/`.

## Architecture

```
rust-pattern-viz/
├── src/
│   ├── analyzer.rs       # AST parsing and metadata extraction
│   ├── models.rs         # Decision tree data structures
│   ├── svg_renderer.rs   # SVG generation engine
│   ├── visualizer.rs     # High-level visualization orchestration
│   ├── lsp_server.rs     # Language Server Protocol implementation
│   ├── web_server.rs     # HTTP server for live preview
│   ├── wasm.rs           # WebAssembly bindings
│   └── main.rs           # CLI entry point
├── vscode-extension/     # VS Code extension (TypeScript)
├── web-demo/             # Interactive web demo (React + WASM)
└── examples/             # Sample Rust files with AI metadata
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design documentation.

## Tech Stack

- **Core**: Rust 2021 edition with `syn` for AST parsing
- **Rendering**: Custom SVG generation with embedded CSS/JS
- **Web**: Actix-web server + WebAssembly (wasm-bindgen)
- **LSP**: Tower-LSP for editor integration
- **Frontend**: React + TypeScript + Vite for web demo
- **VS Code Extension**: TypeScript with official VS Code API

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Development setup:**

```bash
cargo build
cargo test
cargo run -- analyze examples/sample.rs
```

**Run the demo:**

```bash
cd web-demo
npm install
npm run dev
```

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Demo:** Try the live web demo at [rust-pattern-viz-demo.vercel.app](https://rust-pattern-viz-demo.vercel.app) (if deployed)

**Docs:** Full API documentation available at [docs.rs/rust-pattern-viz](https://docs.rs/rust-pattern-viz)