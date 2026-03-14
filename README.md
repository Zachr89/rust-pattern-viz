# rust-pattern-viz

> Visualize AI code generation decision trees from Rust source files

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI-assisted code generation transparent and reviewable. It parses Rust source files containing embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing import considerations, confidence scores, and reasoning DAGs. Perfect for understanding how AI copilots make decisions during code reviews and PR discussions.

## Features

- **Decision Tree Visualization** – See exactly what imports and patterns the AI considered
- **Confidence Scoring** – Line-by-line confidence metrics for AI-generated code
- **Interactive SVG/HTML Output** – Shareable, browser-ready visualizations
- **Pattern Extraction** – Automatic parsing of Rust pattern matching logic
- **Reasoning DAG** – Directed acyclic graph showing AI decision flow
- **VS Code Integration** – Native extension with hover support and inline previews
- **Web Demo** – Try it in your browser with WASM-powered playground
- **LSP Server** – Editor-agnostic language server protocol support
- **Shareable Reports** – Built-in web server for hosting visualization reports

## Quick Start

### Installation

```bash
# From crates.io
cargo install rust-pattern-viz

# From source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a single file
rust-pattern-viz analyze src/main.rs

# Generate SVG visualization
rust-pattern-viz visualize src/main.rs --output output.svg

# Start web server for interactive viewing
rust-pattern-viz serve --port 8080

# Export shareable report
rust-pattern-viz share src/ --output report.html
```

### Library Usage

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

let analyzer = Analyzer::new();
let patterns = analyzer.extract_patterns("src/main.rs")?;

let visualizer = Visualizer::new();
let svg = visualizer.render_svg(&patterns)?;
std::fs::write("output.svg", svg)?;
```

## VS Code Extension

Install the official extension for inline visualizations:

1. Open VS Code Extensions (`Ctrl+Shift+X`)
2. Search for "rust-pattern-viz"
3. Click Install
4. Hover over any pattern match to see AI decision metadata

See [docs/VSCODE_EXTENSION.md](docs/VSCODE_EXTENSION.md) for details.

## Examples

The `examples/` directory contains annotated Rust files demonstrating various pattern matching scenarios:

- **Option unwrapping** – `01_option_unwrapping.rs` with confidence scores
- **Result error handling** – `02_result_error_handling.rs` showing fallback chains
- **Struct destructuring** – `03_struct_destructuring.rs` with import alternatives
- **Enum matching** – `04_enum_matching.rs` displaying exhaustiveness checks
- **Nested patterns** – `05_nested_patterns.rs` with complex reasoning DAGs

Run any example:

```bash
cargo run --example option_pattern
```

## Tech Stack

- **Core**: Rust 2021 edition with `syn` for parsing and `quote` for code generation
- **Visualization**: SVG generation with embedded CSS, HTML templates for interactive views
- **Web**: Actix-web server, WASM bindings via `wasm-bindgen`
- **LSP**: `tower-lsp` for editor integration
- **Frontend**: TypeScript + React (web demo), VS Code Extension API
- **CI/CD**: GitHub Actions for testing, releases, and demo deployment

## Architecture

```
src/
├── analyzer.rs       # Pattern extraction and metadata parsing
├── models.rs         # Data structures for decisions and patterns
├── visualizer.rs     # Graph layout and rendering logic
├── svg_renderer.rs   # SVG generation with styling
├── lsp_server.rs     # Language Server Protocol implementation
├── web_server.rs     # Actix-web server for interactive views
├── share.rs          # Report generation and sharing utilities
└── wasm.rs           # WebAssembly bindings for browser
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design documentation.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Key areas for contribution:
- Additional language support (Python, TypeScript, Go)
- Enhanced visualization layouts
- Performance optimizations for large codebases
- Integration with more AI coding assistants

## Demo

Try the live web demo: [https://rust-pattern-viz-demo.vercel.app](https://rust-pattern-viz-demo.vercel.app)

Or create your own demo GIF:

```bash
cd demo
./create_demo_gif.sh
```

See [DEMO_RECORDING.md](DEMO_RECORDING.md) for recording best practices.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with ❤️ for transparent AI-assisted development**