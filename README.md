# rust-pattern-viz

*Visualize AI code generation decision trees in your Rust projects*

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI-assisted coding transparent. It parses Rust source files with embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing what imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for understanding how AI copilots make decisions during code reviews and PRs.

## Features

- **CLI & Library**: Use as a standalone tool or integrate into your Rust workflow
- **Metadata Parsing**: Extracts AI decision metadata embedded in Rust source comments
- **Interactive Visualizations**: Generates SVG/HTML outputs with clickable decision trees
- **Confidence Scoring**: Shows per-line confidence levels for AI suggestions
- **Reasoning DAG**: Visual graph of import considerations and pattern matching logic
- **WASM Support**: Run in the browser via WebAssembly
- **LSP Server**: Real-time analysis in your editor (VS Code extension included)
- **Web Demo**: Try it instantly at your deployed demo URL

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

### Usage

**Analyze a Rust file:**

```bash
rust-pattern-viz analyze examples/sample.rs --output visualization.html
```

**Run as LSP server:**

```bash
rust-pattern-viz lsp --port 9257
```

**As a library:**

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

let analyzer = Analyzer::new();
let results = analyzer.parse_file("src/main.rs")?;
let viz = Visualizer::new();
viz.generate_html(&results, "output.html")?;
```

## Try the Web Demo

Visit the live demo to visualize AI decisions directly in your browser:

1. Clone the repo and navigate to `web-demo/`
2. Run `npm install && npm run dev`
3. Open `http://localhost:5173`
4. Paste Rust code with AI metadata or use the example snippets

The web demo uses WASM bindings for native Rust performance in the browser.

## VS Code Extension

Install the companion extension for inline visualizations:

```bash
cd vscode-extension
npm install
npm run compile
code --install-extension .
```

Features:
- Hover over AI-generated code to see confidence scores
- Command palette: "Visualize AI Decisions"
- Live updates as you edit

## Tech Stack

- **Core**: Rust with `syn` for parsing, `serde` for serialization
- **Visualization**: SVG generation with `resvg`, HTML templating
- **WASM**: `wasm-bindgen` for browser compatibility
- **Web Demo**: Vite + React + TypeScript
- **LSP**: `tower-lsp` for editor integration
- **CI/CD**: GitHub Actions for testing and demo deployment

## Project Structure

```
rust-pattern-viz/
├── src/
│   ├── analyzer.rs      # Metadata extraction logic
│   ├── visualizer.rs    # SVG/HTML generation
│   ├── models.rs        # Data structures
│   ├── lsp_server.rs    # Language server protocol
│   ├── wasm.rs          # WebAssembly bindings
│   └── main.rs          # CLI entrypoint
├── web-demo/            # Browser-based demo (React)
├── vscode-extension/    # VS Code integration
├── examples/            # Sample Rust files with metadata
└── tests/               # Integration tests
```

## Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)**: Technical design and implementation details
- **[CONTRIBUTING.md](CONTRIBUTING.md)**: How to contribute to the project
- **[DEMO_RECORDING.md](DEMO_RECORDING.md)**: Recording usage examples

## Use Cases

- **Code Reviews**: Understand why AI suggested specific imports or patterns
- **PR Comments**: Attach decision tree visualizations to pull requests
- **Learning**: See how AI copilots reason about code context
- **Debugging**: Identify low-confidence suggestions that need human review
- **Team Alignment**: Share AI reasoning with teammates unfamiliar with copilot tools

## Requirements

- Rust 1.70+ (for CLI/library)
- Node.js 18+ (for web demo and VS Code extension)
- Modern browser with WASM support (for web demo)

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with Rust.** Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.