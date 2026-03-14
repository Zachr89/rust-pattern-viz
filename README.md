# rust-pattern-viz

> Visualize AI code generation decision trees in interactive SVG/HTML

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI-assisted coding transparent. It parses Rust source files with embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive visualizations showing which imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for understanding how your AI copilot makes decisions during code reviews and PR discussions.

## Features

- **Parse AI metadata** — Extracts decision trees from specially-formatted comments in Rust source files
- **Interactive visualizations** — Generates SVG/HTML outputs with clickable nodes and confidence indicators
- **Confidence scoring** — See per-line confidence levels for AI suggestions
- **Import analysis** — Visualize which dependencies were considered and why
- **Reasoning DAG** — Full directed acyclic graph of AI decision-making process
- **CLI & library** — Use standalone or integrate into your build pipeline
- **LSP server** — Real-time visualization in compatible editors
- **VSCode extension** — First-class integration with Visual Studio Code

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

```bash
# Analyze a single file
rust-pattern-viz analyze src/main.rs

# Generate HTML visualization
rust-pattern-viz visualize src/main.rs --output report.html

# Start LSP server for editor integration
rust-pattern-viz lsp --port 9257
```

### As a Library

```rust
use rust_pattern_viz::{Analyzer, Visualizer};

let analyzer = Analyzer::new();
let patterns = analyzer.parse_file("src/main.rs")?;
let visualizer = Visualizer::new();
visualizer.render_svg(&patterns, "output.svg")?;
```

### VSCode Extension

Install the extension from `vscode-extension/` directory:

```bash
cd vscode-extension
npm install
vsce package
code --install-extension rust-pattern-viz-*.vsix
```

Then open any Rust file with AI metadata comments and trigger the visualization command (`Ctrl+Shift+P` → "Visualize AI Patterns").

## Example

Given a Rust file with AI metadata:

```rust
// AI-SUGGESTION: import std::collections::HashMap [confidence: 0.92]
// AI-ALTERNATIVE: import std::collections::BTreeMap [confidence: 0.73]
use std::collections::HashMap;

// AI-REASONING: HashMap chosen for O(1) lookup vs BTreeMap O(log n)
fn process_data(data: Vec<String>) -> HashMap<String, i32> {
    // ...
}
```

Running `rust-pattern-viz visualize` produces an interactive HTML report showing:
- The decision tree (HashMap vs BTreeMap)
- Confidence scores (0.92 vs 0.73)
- Reasoning explanations with clickable nodes

## Tech Stack

- **Language**: Rust (2021 edition)
- **Parsing**: `syn` for Rust AST analysis
- **Visualization**: SVG generation with embedded JavaScript for interactivity
- **LSP**: `tower-lsp` for Language Server Protocol implementation
- **CLI**: `clap` for command-line argument parsing
- **Testing**: Native Rust test framework with integration tests

## Documentation

- [Architecture](ARCHITECTURE.md) — System design and component overview
- [Contributing](CONTRIBUTING.md) — Development setup and guidelines
- [Demo Recording](DEMO_RECORDING.md) — Video walkthrough of key features
- [Examples](examples/) — Sample Rust files with AI metadata

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, coding standards, and PR guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

Built with ❤️ for developers who want to understand their AI coding assistants.