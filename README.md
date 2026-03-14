# rust-pattern-viz

A Rust-native visualization tool that makes AI code generation transparent and reviewable.

## What is this?

`rust-pattern-viz` parses Rust source files containing embedded AI suggestion metadata and generates interactive SVG/HTML visualizations of the decision-making process. It reveals what imports were considered, confidence scores per line, and the complete reasoning DAG behind AI-generated code. Built for developers who need to understand, audit, and communicate AI copilot behavior in code reviews and PRs.

## Features

- **Decision Tree Visualization** – Parse embedded AI metadata and render complete reasoning graphs
- **Pattern Matching Analysis** – Extract and visualize Rust pattern matching logic with confidence scores
- **Multiple Output Formats** – Generate SVG, HTML, and interactive web visualizations
- **CLI & Library** – Use as a standalone tool or integrate into your Rust projects
- **LSP Server** – Real-time visualizations in your editor via Language Server Protocol
- **VS Code Extension** – First-class IDE integration with hover previews
- **Web Demo** – Browser-based WASM-powered visualization without installation
- **Share Server** – Generate shareable links for PR comments and documentation

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
# Generate SVG visualization from annotated Rust file
rust-pattern-viz analyze examples/option_pattern.rs --output diagram.svg

# Start interactive web server
rust-pattern-viz serve --port 8080

# Run LSP server for editor integration
rust-pattern-viz lsp
```

## Usage Examples

### Command Line

```bash
# Analyze all examples and generate diagrams
cargo run -- analyze examples/*.rs --output-dir diagrams/

# Export shareable visualization
cargo run -- share examples/nested_match.rs --upload
```

### Library Integration

```rust
use rust_pattern_viz::{Analyzer, Visualizer, RenderConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = Analyzer::new();
    let tree = analyzer.parse_file("src/main.rs")?;
    
    let visualizer = Visualizer::new(RenderConfig::default());
    let svg = visualizer.render_to_svg(&tree)?;
    
    std::fs::write("output.svg", svg)?;
    Ok(())
}
```

### VS Code Extension

1. Install the extension from `vscode-extension/`
2. Open a Rust file with AI metadata comments
3. Hover over pattern matches to see inline visualizations
4. Run command `Rust Pattern Viz: Generate Diagram` to export

## Project Structure

```
rust-pattern-viz/
├── src/
│   ├── analyzer.rs       # Parse Rust source & extract metadata
│   ├── visualizer.rs     # Generate decision tree representations
│   ├── svg_renderer.rs   # SVG export functionality
│   ├── lsp_server.rs     # Language Server Protocol implementation
│   ├── web_server.rs     # Interactive web interface
│   ├── share.rs          # Shareable link generation
│   └── wasm.rs           # WebAssembly bindings
├── examples/             # Sample Rust files with visualizations
├── web-demo/            # React + WASM web application
├── vscode-extension/    # VS Code integration
└── tests/               # Integration tests
```

## Tech Stack

- **Core**: Rust (syn, quote for parsing; resvg for SVG rendering)
- **Web**: Actix-web (server), React + TypeScript (demo UI), WASM (browser integration)
- **LSP**: tower-lsp for editor protocol implementation
- **CI/CD**: GitHub Actions (testing, deployment)

## Documentation

- [Architecture Overview](ARCHITECTURE.md) – System design and component interaction
- [Contributing Guide](CONTRIBUTING.md) – How to contribute code and examples
- [Demo Instructions](DEMO_RECORDING.md) – Recording screencasts and usage examples

## Real-World Use Cases

- **Code Reviews** – Visualize AI suggestions inline with PR comments
- **Team Onboarding** – Explain complex pattern matching decisions
- **AI Auditing** – Review confidence scores and alternative paths not taken
- **Documentation** – Generate diagrams for technical specifications

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built by developers, for developers who believe AI code generation should be transparent, not opaque.**