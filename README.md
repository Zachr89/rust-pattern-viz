# rust-pattern-viz

> Visualize AI code generation decision trees directly from your Rust source code

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI copilot suggestions transparent and reviewable. It parses Rust source files with embedded AI metadata, extracts pattern matching logic and decision trees, then generates interactive SVG/HTML visualizations showing confidence scores, import alternatives, and reasoning graphs. Perfect for code reviews, PR comments, and understanding what your AI assistant was actually thinking.

## Features

- **Native Rust analyzer** - Fast parsing of Rust source with AI suggestion metadata
- **Interactive visualizations** - SVG and HTML output with confidence scores per line
- **Decision tree DAGs** - See the full reasoning chain for each suggestion
- **Import analysis** - Visualize which imports were considered and why others were rejected
- **Multiple output formats** - CLI tool, library API, VSCode extension, web demo
- **Shareable diagrams** - Generate embeddable visualizations for PR comments and documentation
- **LSP integration** - Real-time visualization in your editor
- **WASM support** - Run in the browser for zero-install demos

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

### Basic Usage

```bash
# Analyze a single file
rust-pattern-viz analyze src/main.rs

# Generate SVG visualization
rust-pattern-viz visualize src/main.rs -o output.svg

# Start interactive web server
rust-pattern-viz serve --port 8080

# Run LSP server for editor integration
rust-pattern-viz lsp
```

## Usage Examples

### CLI Analysis

```bash
# Analyze with detailed output
rust-pattern-viz analyze examples/pattern_match.rs --verbose

# Export to multiple formats
rust-pattern-viz visualize src/lib.rs \
  --format svg,html,json \
  --output-dir ./visualizations
```

### Library API

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

fn main() {
    // Parse source file
    let analyzer = Analyzer::new();
    let patterns = analyzer.analyze_file("src/main.rs")?;
    
    // Generate visualization
    let visualizer = Visualizer::new();
    let svg = visualizer.render_svg(&patterns)?;
    
    std::fs::write("output.svg", svg)?;
}
```

### VSCode Extension

Install from the Extensions marketplace or manually:

```bash
cd vscode-extension
npm install
npm run compile
code --install-extension rust-pattern-viz-*.vsix
```

Hover over any pattern match to see inline decision trees.

### Web Demo

Try it live at [your-demo-url.com] or run locally:

```bash
cd web-demo
npm install
npm run dev
```

## Tech Stack

- **Core**: Rust 2021 edition with `syn` for AST parsing
- **Visualization**: SVG generation with `svg` crate, HTML templates
- **LSP**: `tower-lsp` for editor integration
- **Web**: Axum web server, WASM with `wasm-bindgen`
- **VSCode Extension**: TypeScript + VS Code Extension API
- **Web Demo**: Vite + TypeScript + React
- **CI/CD**: GitHub Actions for testing and deployment

## Documentation

- [Architecture Overview](ARCHITECTURE.md)
- [Contributing Guide](CONTRIBUTING.md)
- [VSCode Extension Docs](docs/VSCODE_EXTENSION.md)
- [Example Gallery](examples/README.md)
- [Demo Recording Guide](demo/RECORDING_GUIDE.md)

## Examples

Check out the [examples/](examples/) directory for annotated samples:

- `01_option_unwrapping.rs` - Basic Option pattern analysis
- `02_result_error_handling.rs` - Error handling decision trees
- `03_struct_destructuring.rs` - Complex destructuring patterns
- `04_enum_matching.rs` - Enum variant confidence scores
- `05_nested_patterns.rs` - Multi-level pattern matching

Each example includes corresponding `.svg` output showing the visualization.

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Built with ❤️ for developers who want to understand their AI copilots**