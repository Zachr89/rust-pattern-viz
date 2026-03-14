# rust-pattern-viz

*Visualize AI code generation decision trees in your Rust projects*

## What is this?

`rust-pattern-viz` is a Rust-native CLI tool and library that makes AI-assisted coding transparent. It parses Rust source files with embedded AI suggestion metadata, extracts pattern matching logic, and generates interactive SVG/HTML visualizations showing what imports were considered, confidence scores per line, and the complete reasoning DAG. Perfect for understanding how AI copilots make decisions, code reviews, and PR documentation.

## Features

- **Parse AI metadata** from Rust source files with embedded suggestions
- **Extract decision trees** showing import candidates, confidence scores, and reasoning chains
- **Generate interactive visualizations** in SVG/HTML format
- **CLI tool** for quick analysis and visualization generation
- **Library API** for programmatic integration into build pipelines
- **VSCode extension** for in-editor visualization
- **Web demo** with WASM-powered browser-based analysis
- **Share server** for collaborative review of AI decision trees
- **LSP integration** for real-time analysis in supported editors

## Quick Start

### Installation

```bash
# Install from crates.io
cargo install rust-pattern-viz

# Or clone and build from source
git clone https://github.com/yourusername/rust-pattern-viz.git
cd rust-pattern-viz
cargo build --release
```

### Basic Usage

```bash
# Analyze a Rust file and generate visualization
rust-pattern-viz analyze examples/sample.rs --output viz.html

# Start the web server for interactive exploration
rust-pattern-viz serve --port 8080

# Run the share server for team collaboration
rust-pattern-viz-share --host 0.0.0.0 --port 3000
```

## Usage Examples

### CLI Analysis

```bash
# Generate SVG visualization
rust-pattern-viz analyze src/main.rs --format svg --output output.svg

# Analyze with verbose decision tree
rust-pattern-viz analyze src/lib.rs --verbose --show-confidence

# Export raw JSON for custom processing
rust-pattern-viz analyze src/analyzer.rs --format json > analysis.json
```

### Library Integration

```rust
use rust_pattern_viz::{Analyzer, Visualizer, VisualizationFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse source file
    let analyzer = Analyzer::new();
    let analysis = analyzer.analyze_file("src/main.rs")?;
    
    // Generate visualization
    let visualizer = Visualizer::new();
    let html = visualizer.render(&analysis, VisualizationFormat::Html)?;
    
    std::fs::write("output.html", html)?;
    Ok(())
}
```

### VSCode Extension

1. Install the extension from the marketplace or `vscode-extension/` directory
2. Open a Rust file with AI metadata
3. Run command: `Rust Pattern Viz: Visualize Decision Tree`
4. View interactive visualization in a side panel

### Web Demo

```bash
cd web-demo
npm install
npm run dev
```

Visit `http://localhost:5173` to try the browser-based analyzer with sample code.

## Tech Stack

**Core:**
- **Rust** - High-performance parsing and analysis engine
- **syn** - Rust syntax parsing
- **serde** - Serialization/deserialization

**Visualization:**
- **SVG generation** - Scalable vector graphics output
- **HTML/CSS/JS** - Interactive web visualizations

**Web Components:**
- **Axum** - Web server framework
- **WASM** (wasm-bindgen) - Browser-based analysis
- **TypeScript** - VSCode extension
- **React + Vite** - Web demo interface

**Development:**
- **GitHub Actions** - CI/CD pipeline
- **cargo** - Build system and package manager

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - System design and component overview
- [CONTRIBUTING.md](CONTRIBUTING.md) - Development guidelines
- [DEMO_RECORDING.md](DEMO_RECORDING.md) - Demo usage and examples
- [VSCode Extension README](vscode-extension/README.md) - Editor integration guide
- [Web Demo README](web-demo/README.md) - Browser app documentation

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Made for developers who want to understand their AI pair programmer** 🤖🔍