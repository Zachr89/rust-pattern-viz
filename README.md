# rust-pattern-viz

*Visualize AI code generation decision trees directly from your Rust source*

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI-assisted coding transparent. It parses Rust source files with embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing import considerations, per-line confidence scores, and the complete reasoning DAG. Essential for code reviews, PR documentation, and understanding how your AI copilot makes decisions.

## Features

- **Source-to-Graph Parsing** – Extracts AI decision metadata directly from Rust comments
- **Interactive SVG Export** – Generates standalone visualizations for web viewing
- **Pattern Analysis** – Visualizes match statements, Option/Result flows, and error handling
- **Confidence Scoring** – Shows AI certainty levels for each code suggestion
- **LSP Integration** – Provides real-time hover tooltips in VS Code
- **Web Demo** – Browser-based WASM visualization for easy sharing
- **CLI Tool** – Batch process files or integrate into CI/CD pipelines
- **Share Server** – Host and share visualizations with unique URLs

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
# Generate SVG visualization
rust-pattern-viz analyze examples/option_pattern.rs --output diagram.svg

# Start web server for interactive viewing
rust-pattern-viz serve --port 8080

# Run LSP server for editor integration
rust-pattern-viz lsp
```

## Usage Examples

### CLI Visualization

```bash
# Analyze a single file
rust-pattern-viz analyze src/lib.rs -o output/analysis.svg

# Process multiple files
rust-pattern-viz analyze examples/*.rs --output-dir diagrams/

# Export as HTML with interactive features
rust-pattern-viz analyze src/main.rs --format html -o report.html
```

### Library Usage

```rust
use rust_pattern_viz::{Analyzer, SvgRenderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse source file
    let analyzer = Analyzer::new();
    let patterns = analyzer.analyze_file("examples/option_pattern.rs")?;
    
    // Generate visualization
    let renderer = SvgRenderer::new();
    let svg = renderer.render(&patterns)?;
    
    std::fs::write("output.svg", svg)?;
    Ok(())
}
```

### VS Code Integration

Install the VS Code extension to get inline visualizations:

```bash
cd vscode-extension
npm install
npm run compile
code --install-extension rust-pattern-viz-*.vsix
```

Hover over pattern match expressions to see AI decision trees directly in your editor.

### Web Demo

Try the browser-based demo:

```bash
cd web-demo
npm install
npm run dev
```

Open `http://localhost:5173` to visualize patterns without installing anything.

## Tech Stack

- **Core**: Rust 2021 edition with `syn` for parsing and `petgraph` for DAG generation
- **Visualization**: SVG generation with `svg` crate, HTML templates via `askama`
- **Web**: Actix-web for servers, WASM via `wasm-bindgen` for browser support
- **Editor Integration**: LSP protocol implementation with `tower-lsp`
- **Frontend**: TypeScript + React + Vite for web demo
- **CI/CD**: GitHub Actions for testing and automated demo deployment

## Documentation

- [Architecture Overview](ARCHITECTURE.md) – System design and module structure
- [Contributing Guide](CONTRIBUTING.md) – How to contribute
- [Demo Creation](docs/create-demo.md) – Recording visualizations
- [Example Gallery](examples/) – Sample inputs and outputs

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built for transparency in AI-assisted development.** Questions or feedback? Open an issue or start a discussion.