# rust-pattern-viz

**Visualize AI code generation decision trees directly in your IDE**

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI code suggestions transparent and reviewable. It parses Rust source files containing embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing what imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for understanding AI copilot behavior during code reviews and PR discussions.

## Features

- **AI Decision Tree Parsing** – Extract and analyze embedded AI suggestion metadata from Rust source files
- **Interactive Visualizations** – Generate SVG/HTML diagrams showing confidence scores, alternative imports, and reasoning paths
- **CLI & Library** – Use as a standalone tool or integrate into your Rust projects
- **VSCode Extension** – Hover over code to see AI decision trees inline
- **LSP Server** – Real-time analysis with editor integration
- **Web Demo** – Browser-based visualization powered by WebAssembly
- **Share & Collaborate** – Built-in server for sharing visualizations with teams
- **Pattern Matching Analysis** – Specialized support for Rust's `match`, `Option`, and `Result` patterns

## Quick Start

### Installation

```bash
# Install the CLI
cargo install rust-pattern-viz

# Or build from source
git clone https://github.com/yourusername/rust-pattern-viz
cd rust-pattern-viz
cargo build --release
```

### VSCode Extension

1. Open VSCode
2. Search for "Rust Pattern Viz" in the Extensions marketplace
3. Install and reload
4. Hover over Rust code with AI metadata to see visualizations

## Usage

### CLI Examples

```bash
# Analyze a single file
rust-pattern-viz analyze examples/sample.rs

# Generate SVG visualization
rust-pattern-viz visualize examples/nested_match.rs -o output.svg

# Start web server for interactive exploration
rust-pattern-viz serve --port 8080

# Export diagram for PR comments
rust-pattern-viz export examples/result_pattern.rs --format html
```

### Library Usage

```rust
use rust_pattern_viz::{analyzer, visualizer};

// Parse a Rust file with AI metadata
let analysis = analyzer::analyze_file("src/main.rs")?;

// Generate visualization
let svg = visualizer::render_svg(&analysis)?;
println!("{}", svg);
```

### Example Output

Check out the [examples/diagrams](examples/diagrams) directory for sample visualizations:
- `option_pattern.svg` – Option<T> handling decision tree
- `result_pattern.svg` – Result<T, E> error flow analysis
- `nested_match.svg` – Complex nested pattern matching

## Tech Stack

- **Core**: Rust (syn, quote for AST parsing)
- **Visualization**: SVG generation with custom rendering engine
- **Web**: WebAssembly (wasm-bindgen), React + TypeScript frontend
- **LSP**: tower-lsp for editor integration
- **CLI**: clap for argument parsing
- **VSCode Extension**: TypeScript, VS Code API
- **CI/CD**: GitHub Actions for automated testing and deployment

## Documentation

- [Architecture](ARCHITECTURE.md) – Technical deep dive
- [Contributing](CONTRIBUTING.md) – Development setup and guidelines
- [Demo Recording](DEMO_RECORDING.md) – Video walkthrough
- [Web Demo Docs](web-demo/README.md) – Browser-based visualization guide

## Development

```bash
# Run tests
cargo test

# Run integration tests
cargo test --test integration_test

# Start development server
cargo run -- serve --dev

# Build VSCode extension
cd vscode-extension
npm install
npm run compile
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Note**: This tool is designed for analyzing AI-generated code with embedded metadata. For best results, ensure your AI coding assistant is configured to include suggestion metadata in comments or attributes.