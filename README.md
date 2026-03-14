# rust-pattern-viz

> Visualize AI code generation decisions in your Rust projects

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that transforms AI copilot suggestions into interactive visual decision trees. Parse your Rust source files to extract embedded AI metadata, then generate SVG/HTML visualizations showing import considerations, per-line confidence scores, and complete reasoning DAGs. Perfect for code reviews, PR documentation, and understanding what your AI pair programmer is actually thinking.

## Features

- **Parse AI metadata** from Rust source files with embedded suggestion annotations
- **Extract pattern matching logic** with full context preservation
- **Generate interactive visualizations** as SVG or HTML with zoom/pan controls
- **Display confidence scores** per line with color-coded reasoning paths
- **Show import alternatives** that were considered but rejected
- **Render decision DAGs** showing the complete AI reasoning flow
- **CLI + Library** - use as a standalone tool or integrate into your workflow
- **LSP Server** for real-time IDE integration
- **VS Code Extension** with hover tooltips and panel views
- **Web Demo** with live editing and instant visualization
- **Share Server** for generating permanent links to visualizations
- **WASM Support** for browser-based rendering

## Quick Start

### Installation

```bash
# Install via Cargo
cargo install rust-pattern-viz

# Or build from source
git clone https://github.com/yourusername/rust-pattern-viz
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a single file
rust-pattern-viz analyze examples/option_pattern.rs

# Generate SVG output
rust-pattern-viz visualize examples/result_pattern.rs -o output.svg

# Start interactive web viewer
rust-pattern-viz serve examples/

# Run LSP server for IDE integration
rust-pattern-viz lsp

# Share a visualization (generates permanent link)
rust-pattern-viz share examples/nested_match.rs
```

### Usage Examples

**Analyze AI decisions in your code:**

```rust
// Your Rust file with AI metadata comments
fn process_data(input: Option<String>) -> Result<(), Error> {
    // @ai-suggestion confidence=0.92 alternatives=[if-let, match]
    match input {
        Some(data) => println!("{}", data),
        None => return Err(Error::NoData),
    }
    Ok(())
}
```

**Generate a visualization:**

```bash
rust-pattern-viz visualize src/main.rs --format svg --output decision-tree.svg
```

**Integrate as a library:**

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

let analyzer = Analyzer::new();
let results = analyzer.analyze_file("src/lib.rs")?;

let viz = Visualizer::new();
let svg = viz.render_svg(&results)?;
println!("{}", svg);
```

**VS Code Integration:**

Install the extension from `vscode-extension/` to get:
- Hover tooltips showing AI confidence scores
- Side panel with interactive decision trees
- Inline annotations for alternative patterns

## Tech Stack

- **Core**: Rust 2021 edition
- **CLI**: `clap` for argument parsing
- **Parsing**: `syn` for Rust AST analysis
- **Visualization**: Custom SVG renderer with `resvg` support
- **Web Server**: `axum` for HTTP endpoints
- **LSP**: `tower-lsp` for editor integration
- **WASM**: `wasm-bindgen` for browser compatibility
- **Frontend**: TypeScript + React + Vite (web demo)
- **VS Code Extension**: TypeScript + Webpack

## Examples

The `examples/` directory contains annotated samples:

- `01_option_unwrapping.rs` - Basic Option<T> pattern analysis
- `02_result_error_handling.rs` - Result<T, E> decision trees
- `03_struct_destructuring.rs` - Complex destructuring patterns
- `04_enum_matching.rs` - Exhaustive enum match visualization
- `05_nested_patterns.rs` - Multi-level nested pattern reasoning

Each example includes its corresponding `.svg` output for reference.

## Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System design and component overview
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines
- **[VS Code Extension Guide](docs/VSCODE_EXTENSION.md)** - Editor integration setup
- **[Demo Recording](docs/DEMO_RECORDING_GUIDE.md)** - Create your own visualizations

## CI/CD

- Automated testing via GitHub Actions (`.github/workflows/ci.yml`)
- Demo deployment pipeline (`.github/workflows/deploy-demo.yml`)
- Pre-commit hooks for linting and formatting

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with Rust** 🦀 | **[View Demo](web-demo/)** | **[Report Issues](https://github.com/yourusername/rust-pattern-viz/issues)**