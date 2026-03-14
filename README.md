# rust-pattern-viz

> Visualize AI code generation decisions in your Rust projects

## What is this?

`rust-pattern-viz` is a developer tool that makes AI-assisted coding transparent. It parses Rust source files containing embedded AI suggestion metadata, extracts decision trees showing what imports were considered, confidence scores per line, and reasoning paths—then generates interactive SVG/HTML visualizations. Perfect for understanding copilot behavior during code reviews, adding context to PRs, or debugging why AI suggested specific patterns.

## Features

- **Decision Tree Visualization** – See the full reasoning DAG behind each AI suggestion
- **Confidence Scoring** – Per-line confidence metrics to identify uncertain generations
- **Import Analysis** – Track what imports were considered vs. selected and why
- **Interactive SVG/HTML Output** – Pan, zoom, and click through reasoning paths
- **CLI + Library** – Use as a standalone tool or integrate into build pipelines
- **LSP Server** – Real-time visualization in VSCode via language server protocol
- **Web Demo** – Try it instantly at [demo link] with WASM-powered browser parsing
- **Shareable Reports** – Generate standalone HTML files for PR comments
- **CI/CD Ready** – GitHub Actions workflow included for automated visualization

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
# Analyze a single file
rust-pattern-viz analyze src/main.rs --output viz.svg

# Generate interactive HTML
rust-pattern-viz visualize src/ --format html --output report.html

# Start LSP server for editor integration
rust-pattern-viz lsp

# Run web demo locally
cd web-demo && npm install && npm run dev
```

## Usage Examples

### Analyzing AI Suggestions

```rust
// Your Rust file with AI metadata comments
// @ai-suggestion: confidence=0.92, alternatives=[tokio::fs, std::fs::read_to_string]
use std::fs;

fn load_config() -> Result<String, std::io::Error> {
    // @ai-decision: selected std::fs over tokio::fs due to sync context
    fs::read_to_string("config.toml")
}
```

Run the analyzer:

```bash
rust-pattern-viz analyze src/config.rs --show-alternatives
```

Output shows a decision tree with:
- Why `std::fs` was chosen (sync context detected)
- What alternatives were considered (`tokio::fs`, `std::fs::read_to_string`)
- Confidence score (0.92) with reasoning

### VSCode Integration

Install the extension from `vscode-extension/` and hover over any AI-generated line to see inline decision trees:

```bash
cd vscode-extension
npm install && vsce package
code --install-extension rust-pattern-viz-0.1.0.vsix
```

### CI/CD Integration

Add to `.github/workflows/ci.yml`:

```yaml
- name: Generate AI Decision Report
  run: |
    cargo install rust-pattern-viz
    rust-pattern-viz visualize src/ --format html --output ai-decisions.html
    
- name: Upload Report
  uses: actions/upload-artifact@v3
  with:
    name: ai-decision-report
    path: ai-decisions.html
```

### Library Usage

```rust
use rust_pattern_viz::{Analyzer, Visualizer, VisualizationFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = Analyzer::new();
    let decisions = analyzer.parse_file("src/main.rs")?;
    
    let visualizer = Visualizer::new();
    let svg = visualizer.render(&decisions, VisualizationFormat::SVG)?;
    
    std::fs::write("output.svg", svg)?;
    Ok(())
}
```

## Tech Stack

- **Core**: Rust (syn, proc-macro2 for AST parsing)
- **Visualization**: SVG generation with `resvg`, HTML templates with `askama`
- **LSP**: `tower-lsp` for editor integration
- **Web**: WASM (`wasm-bindgen`), React + TypeScript (demo UI), Vite
- **Server**: `axum` for share server and web demo hosting
- **CI/CD**: GitHub Actions with artifact uploads

## Documentation

- [Architecture Overview](ARCHITECTURE.md) – System design and component interaction
- [Contributing Guide](CONTRIBUTING.md) – Development setup and PR guidelines
- [Demo Script](docs/demo-script.sh) – Reproduce the demo locally
- [VSCode Extension Docs](vscode-extension/README.md) – Editor integration details

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Try the live demo**: [Insert demo URL]  
**Report issues**: [GitHub Issues](https://github.com/yourusername/rust-pattern-viz/issues)  
**Discussions**: [GitHub Discussions](https://github.com/yourusername/rust-pattern-viz/discussions)