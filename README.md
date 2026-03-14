# rust-pattern-viz

> Visualize AI code generation decision trees from Rust source files

## What is this?

**rust-pattern-viz** is a Rust-native CLI tool and library that makes AI copilot behavior transparent and reviewable. It parses Rust source files containing embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing import considerations, per-line confidence scores, and complete reasoning DAGs. Perfect for understanding what your AI assistant was "thinking" during code reviews and PR discussions.

## Features

- **AI Decision Tree Visualization** – Convert embedded AI metadata into interactive SVG/HTML diagrams
- **Pattern Matching Analysis** – Extract and visualize Rust pattern matching logic with confidence scoring
- **Multiple Output Formats** – Generate SVG, HTML, and shareable web visualizations
- **CLI & Library** – Use as a standalone tool or integrate into your Rust projects
- **LSP Server** – Real-time visualizations in your editor via Language Server Protocol
- **VS Code Extension** – First-class support with hover previews and inline diagnostics
- **Web Demo** – Try it instantly in your browser via WebAssembly
- **Share Server** – One-click sharing of visualizations with permanent URLs

## Quick Start

### Installation

```bash
# Install via cargo
cargo install rust-pattern-viz

# Or clone and build from source
git clone https://github.com/yourusername/rust-pattern-viz
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a Rust file and generate SVG
rust-pattern-viz analyze examples/option_pattern.rs --output diagram.svg

# Start the web server for interactive exploration
rust-pattern-viz serve --port 8080

# Launch the share server for collaborative reviews
rust-pattern-viz share --host 0.0.0.0:3000
```

### VS Code Integration

1. Install the extension from `vscode-extension/`
2. Open any Rust file with AI metadata comments
3. Hover over pattern matches to see inline visualizations
4. Use `Cmd+Shift+P` → "Visualize Pattern" to generate full diagrams

## Usage Examples

### CLI Analysis

```bash
# Generate all examples at once
cargo run --example svg_export

# Analyze specific pattern types
rust-pattern-viz analyze examples/04_enum_matching.rs \
  --format html \
  --show-confidence \
  --output report.html
```

### Library Integration

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

let source = std::fs::read_to_string("src/main.rs")?;
let analyzer = Analyzer::new();
let patterns = analyzer.extract_patterns(&source)?;

let visualizer = Visualizer::new();
let svg = visualizer.render_svg(&patterns)?;
std::fs::write("output.svg", svg)?;
```

### Web Demo

Visit the [live demo](https://your-demo-url.com) or run locally:

```bash
cd web-demo
npm install
npm run dev
```

## Tech Stack

- **Core**: Rust 2021 edition with `syn` for AST parsing
- **Visualization**: SVG generation with D3.js-inspired layout algorithms
- **Web**: Actix-web server, WebAssembly (wasm-bindgen), React + TypeScript frontend
- **LSP**: Tower-LSP for editor integration
- **CLI**: Clap for argument parsing, indicatif for progress bars

## Documentation

- [Architecture Overview](ARCHITECTURE.md) – System design and component interaction
- [Contributing Guide](CONTRIBUTING.md) – How to contribute
- [Examples Gallery](examples/README.md) – 10+ annotated examples with visualizations
- [Demo Recording Guide](docs/DEMO_RECORDING_GUIDE.md) – Creating promotional materials

## Examples Included

| Example | Pattern Type | Visualization |
|---------|-------------|---------------|
| `01_option_unwrapping.rs` | Option<T> handling | [SVG](examples/01_option_unwrapping.svg) |
| `02_result_error_handling.rs` | Result propagation | [SVG](examples/02_result_error_handling.svg) |
| `03_struct_destructuring.rs` | Struct patterns | [SVG](examples/03_struct_destructuring.svg) |
| `04_enum_matching.rs` | Enum variants | [SVG](examples/04_enum_matching.svg) |
| `05_nested_patterns.rs` | Complex nesting | [SVG](examples/05_nested_patterns.svg) |

## CI/CD

Automated workflows handle testing and deployment:

- **CI Pipeline** (`.github/workflows/ci.yml`) – Runs tests, lints, and builds on every commit
- **Demo Deployment** (`.github/workflows/deploy-demo.yml`) – Auto-deploys web demo to GitHub Pages

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with** ❤️ **for developers who want to understand their AI coding assistants**