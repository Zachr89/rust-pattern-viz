# rust-pattern-viz

> Visualize AI code generation decisions directly in your codebase

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI coding assistants transparent. It parses embedded metadata in Rust source files to extract and visualize the decision trees behind AI-generated code—showing what imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for code reviews, PR comments, and understanding what your AI copilot is actually doing.

## Features

- **Native Rust parsing** – Fast, zero-dependency analysis of Rust source files
- **Decision tree extraction** – Identifies pattern matching logic and alternative suggestions
- **Interactive visualizations** – Generates SVG/HTML graphs with confidence scores and reasoning paths
- **Multiple interfaces** – CLI tool, Rust library, LSP server, VSCode extension, and web demo
- **Shareable reports** – Export visualizations for PR comments and team reviews
- **WASM support** – Run in the browser with full feature parity
- **Real-time hover tooltips** – VSCode integration shows AI reasoning inline

## Quick Start

### Installation

```bash
# Install via cargo
cargo install rust-pattern-viz

# Or build from source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Usage

#### CLI

```bash
# Analyze a single file and generate visualization
rust-pattern-viz analyze src/main.rs --output viz.html

# Analyze with confidence threshold filtering
rust-pattern-viz analyze src/main.rs --min-confidence 0.7

# Generate shareable report
rust-pattern-viz share src/main.rs --upload
```

#### As a Library

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

fn main() {
    let analyzer = Analyzer::new();
    let decisions = analyzer.parse_file("src/main.rs")?;
    
    let visualizer = Visualizer::default();
    visualizer.render_to_html(&decisions, "output.html")?;
}
```

#### VSCode Extension

1. Install the extension from `vscode-extension/`
2. Open any Rust file with AI metadata comments
3. Hover over code to see confidence scores and alternatives
4. Run command `Rust Pattern Viz: Visualize File` to generate interactive graph

#### Web Demo

```bash
cd web-demo
npm install
npm run dev
# Open http://localhost:5173
```

## Usage Examples

### Analyzing AI-Generated Code

Given a Rust file with embedded AI metadata:

```rust
// @ai-suggestion confidence=0.85 alternatives=["tokio::time", "async-std"]
use std::time::Duration;

// @ai-pattern match=import_selection reasoning="stdlib preferred for simple cases"
fn delay(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}
```

Run the analyzer:

```bash
rust-pattern-viz analyze example.rs -o report.html
```

The generated visualization shows:
- **Decision nodes** for each import choice with confidence scores
- **Alternative paths** that were considered but rejected
- **Reasoning chains** explaining why specific patterns were selected
- **Line-by-line annotations** with color-coded confidence levels

### Integration in CI/CD

```yaml
# .github/workflows/ci.yml
- name: Generate AI Decision Report
  run: |
    cargo install rust-pattern-viz
    rust-pattern-viz analyze src/ --output ai-report.html
    
- name: Upload Report
  uses: actions/upload-artifact@v3
  with:
    name: ai-decisions
    path: ai-report.html
```

### LSP Server Mode

```bash
# Start LSP server for editor integration
rust-pattern-viz lsp --stdio
```

Configure your editor to use the LSP server for real-time AI decision visualization.

## Tech Stack

- **Core**: Rust 2021 edition
- **Parsing**: `syn` for Rust AST manipulation
- **Visualization**: SVG generation with custom graph layout engine
- **Web**: 
  - Server: `axum` + `tower`
  - Frontend: React + TypeScript + Vite
  - WASM: `wasm-bindgen` + `wasm-pack`
- **LSP**: `tower-lsp` for editor integration
- **Testing**: `cargo test` + integration tests

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) – System design and component overview
- [CONTRIBUTING.md](CONTRIBUTING.md) – Development setup and guidelines
- [Demo Recording](DEMO_RECORDING.md) – Video walkthrough and screenshots
- [VSCode Extension](vscode-extension/README.md) – Editor plugin documentation
- [Web Demo](web-demo/README.md) – Browser-based interface guide

## License

MIT License - see [LICENSE](LICENSE) for details

---

**Note**: This tool works best with AI coding assistants that emit structured metadata comments. Examples and templates for compatible comment formats are in `examples/`.