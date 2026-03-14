# rust-pattern-viz

> Make AI code generation transparent with interactive decision tree visualizations

## What is this?

**rust-pattern-viz** is a Rust-native CLI tool and library that turns AI code suggestions into visual decision trees. It parses Rust source files with embedded AI metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations that show what imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for code reviews, PR comments, and understanding what your AI copilot is actually thinking.

## Features

- **Parse AI metadata** from Rust source files with embedded suggestion annotations
- **Extract decision trees** showing pattern matching logic and confidence scores
- **Generate interactive visualizations** as SVG or self-contained HTML
- **Multiple interfaces**: CLI tool, Rust library, LSP server, VS Code extension, and web demo
- **Share and embed** visualizations with built-in sharing server
- **WebAssembly support** for browser-based visualization without server dependencies
- **Real-time hover inspection** via LSP integration in your editor
- **Zero dependencies** for core visualization output (pure SVG/HTML)

## Quick Start

### Installation

```bash
# Install the CLI tool
cargo install rust-pattern-viz

# Or clone and build from source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a Rust file and generate visualization
rust-pattern-viz analyze examples/sample.rs --output decision-tree.html

# Start interactive web server
rust-pattern-viz serve --port 8080

# Run LSP server for editor integration
rust-pattern-viz lsp
```

## Usage Examples

### CLI Analysis

```bash
# Generate SVG output
rust-pattern-viz analyze src/main.rs -o tree.svg

# Generate interactive HTML with embedded JavaScript
rust-pattern-viz analyze src/lib.rs --format html -o interactive.html

# Analyze and auto-open in browser
rust-pattern-viz analyze examples/sample.rs --open
```

### As a Library

```rust
use rust_pattern_viz::{Analyzer, Visualizer, OutputFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse source file with AI metadata
    let analyzer = Analyzer::new();
    let decision_tree = analyzer.parse_file("src/main.rs")?;
    
    // Generate visualization
    let visualizer = Visualizer::new();
    let svg = visualizer.render(&decision_tree, OutputFormat::Svg)?;
    
    std::fs::write("output.svg", svg)?;
    Ok(())
}
```

### VS Code Extension

Install the extension from the `vscode-extension/` directory:

```bash
cd vscode-extension
npm install
npm run compile
code --install-extension .
```

Hover over AI-generated code to see inline decision trees and confidence scores.

### Web Demo

```bash
cd web-demo
npm install
npm run dev
```

Try the live demo with example Rust files or paste your own code with AI annotations.

## Tech Stack

- **Core**: Rust 2021 edition with `syn` for parsing, `serde` for serialization
- **Visualization**: SVG generation with interactive JavaScript for HTML output
- **Web**: Actix-web for HTTP server, WebAssembly via `wasm-bindgen`
- **LSP**: Custom Language Server Protocol implementation for editor integration
- **VS Code Extension**: TypeScript + VS Code Extension API
- **Web Demo**: React + TypeScript + Vite, WebAssembly integration
- **CI/CD**: GitHub Actions for testing and deployment

## Architecture

```
rust-pattern-viz/
├── src/
│   ├── analyzer.rs      # Parse Rust source + AI metadata
│   ├── models.rs        # Decision tree data structures
│   ├── visualizer.rs    # SVG/HTML generation
│   ├── lsp_server.rs    # Language Server Protocol
│   ├── web_server.rs    # HTTP API server
│   ├── share.rs         # Sharing and embedding logic
│   ├── wasm.rs          # WebAssembly bindings
│   └── main.rs          # CLI interface
├── vscode-extension/    # VS Code editor plugin
├── web-demo/            # Browser-based demo app
└── examples/            # Sample annotated Rust files
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design documentation.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with** 🦀 **by developers who want to understand their AI tools**