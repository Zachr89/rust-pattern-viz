# rust-pattern-viz

**Visualize AI code generation decision trees and pattern matching logic in Rust**

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI-assisted coding transparent. It parses Rust source files containing embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing import considerations, per-line confidence scores, and reasoning DAGs. Whether you're reviewing AI-generated code, adding visualizations to PR comments, or debugging copilot behavior, this tool provides instant insight into how AI models make decisions.

## Features

- **🔍 AI Decision Analysis** – Extracts and visualizes AI suggestion metadata from Rust source files
- **📊 Interactive Visualizations** – Generates SVG/HTML diagrams showing decision trees and reasoning flows
- **🎯 Confidence Scoring** – Display per-line confidence metrics for AI-generated suggestions
- **🌐 Web Demo** – Browser-based interface for exploring examples without installation
- **🔌 VS Code Extension** – Inline visualization panels with hover tooltips and real-time updates
- **🚀 LSP Server** – Language server integration for IDE support across editors
- **📦 WASM Support** – Run the analyzer client-side in web applications
- **🎨 Pattern Matching Focus** – Deep analysis of Rust's pattern matching constructs (match, if-let, destructuring)
- **📤 Export Formats** – SVG, HTML, and shareable links for team collaboration
- **⚡ Zero Config** – Works out-of-the-box with sensible defaults

## Quick Start

### Installation

```bash
# Install via cargo
cargo install rust-pattern-viz

# Or clone and build from source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a Rust file and generate an SVG visualization
rust-pattern-viz analyze src/main.rs --output decision-tree.svg

# Start interactive web server on localhost:3000
rust-pattern-viz serve --port 3000

# Start LSP server for editor integration
rust-pattern-viz lsp
```

### VS Code Extension

1. Open Extensions panel (`Ctrl+Shift+X` or `Cmd+Shift+X`)
2. Search for "Rust Pattern Viz"
3. Click Install
4. Open any `.rs` file and hover over pattern matches to see visualizations

## Usage Examples

### CLI Analysis

```rust
// example.rs with AI metadata comments
fn process_result(value: Result<i32, String>) -> i32 {
    // @ai-suggestion: confidence=0.92, alternatives=["unwrap_or_default", "expect"]
    match value {
        Ok(n) => n,
        Err(_) => 0,
    }
}
```

```bash
rust-pattern-viz analyze example.rs --format html --open
```

### Library Integration

```rust
use rust_pattern_viz::{Analyzer, SvgRenderer};

fn main() {
    let source = std::fs::read_to_string("src/main.rs").unwrap();
    let analyzer = Analyzer::new();
    let patterns = analyzer.extract_patterns(&source).unwrap();
    
    let renderer = SvgRenderer::new();
    let svg = renderer.render(&patterns);
    std::fs::write("output.svg", svg).unwrap();
}
```

### Web Demo

Visit the [live demo](https://your-username.github.io/rust-pattern-viz) to try examples in your browser:

- Option/Result unwrapping strategies
- Enum matching with confidence scores
- Nested pattern alternatives
- Struct destructuring decisions

### Share Server

Start a server to generate shareable links:

```bash
# Start share server
rust-pattern-viz share-server --host 0.0.0.0 --port 8080

# Generate shareable link
curl -X POST http://localhost:8080/share \
  -H "Content-Type: application/json" \
  -d '{"code": "fn main() { ... }", "metadata": {...}}'
```

## Tech Stack

**Core**
- Rust (CLI, library, analysis engine)
- `syn` – Rust AST parsing
- `quote` – Code generation
- `serde` – Serialization/deserialization

**Visualization**
- SVG generation with custom renderer
- HTML templates with embedded interactivity
- CSS for styling decision trees

**Web Components**
- TypeScript (VS Code extension)
- Vite + React (web demo)
- WebAssembly (browser runtime via `wasm-bindgen`)

**Infrastructure**
- GitHub Actions (CI/CD)
- Docker (optional deployment)
- LSP protocol for editor integration

## Project Structure

```
rust-pattern-viz/
├── src/               # Core library and CLI
├── examples/          # Sample visualizations
├── vscode-extension/  # VS Code integration
├── web-demo/          # Browser-based demo
├── wasm/              # WebAssembly bindings
└── docs/              # Documentation
```

## Documentation

- [Architecture Overview](ARCHITECTURE.md)
- [Contributing Guide](CONTRIBUTING.md)
- [VS Code Extension Guide](docs/VSCODE_EXTENSION.md)
- [Creating Demo Content](DEMO_CREATION.md)
- [Examples Gallery](examples/README.md)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code style guidelines
- Testing requirements
- PR submission process

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with ❤️ for the Rust and AI communities**