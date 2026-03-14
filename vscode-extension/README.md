# Rust Pattern Visualizer - VS Code Extension

Visualize Rust pattern matching, control flow, and decision trees directly in your editor. This extension wraps the powerful rust-pattern-viz analyzer as a VS Code sidebar panel, providing instant visual feedback as you code.

## Features

- **Real-time Pattern Analysis**: Automatically analyzes Rust files as you type (with configurable debouncing)
- **Interactive Visualizations**: SVG-based diagrams showing:
  - `match` expressions and pattern coverage
  - `if let` and `while let` control flow
  - Error handling patterns (`Result<T,E>`, `Option<T>`)
  - Decision trees with confidence scores
  - Import analysis
- **Export Capability**: Save visualizations as SVG files for documentation
- **Zero Configuration**: Works out of the box with any Rust project

## Usage

1. **Install the Extension**: Search for "Rust Pattern Visualizer" in VS Code marketplace
2. **Open a Rust File**: The pattern visualization panel appears in the activity bar (left sidebar)
3. **View Patterns**: The panel automatically updates as you edit code
4. **Export Diagrams**: Click the export icon to save SVG visualizations

## Commands

- `Rust Pattern Viz: Refresh` - Manually refresh the current analysis
- `Rust Pattern Viz: Export SVG` - Export the current visualization to an SVG file

## Configuration

```json
{
  "rustPatternViz.autoRefresh": true,
  "rustPatternViz.debounceDelay": 500,
  "rustPatternViz.showConfidenceScores": true
}
```

- **autoRefresh**: Enable/disable automatic analysis on file changes
- **debounceDelay**: Milliseconds to wait after typing before re-analyzing (default: 500)
- **showConfidenceScores**: Show confidence scores in pattern boxes (default: true)

## Example Visualizations

### Match Expression
```rust
match result {
    Ok(value) => process(value),
    Err(e) => handle_error(e),
}
```
→ Shows both branches with confidence scores and reasoning

### Control Flow
```rust
while let Some(item) = iterator.next() {
    process(item);
}
```
→ Displays loop condition diamond with success/continuation paths

### Error Handling
```rust
fn process() -> Result<Value, Error> {
    let data = fetch_data()?;
    Ok(transform(data))
}
```
→ Highlights error propagation pattern with confidence score

## Requirements

- VS Code 1.75.0 or higher
- Rust files (`.rs` extension)

## Building from Source

```bash
# Install dependencies
cd vscode-extension
npm install

# Build WASM module
npm run build:wasm

# Compile extension
npm run compile

# Package for distribution
npm run package
```

## Known Issues

- Large files (>10k lines) may take a few seconds to analyze
- Complex macro-generated code may not be fully analyzed
- WebAssembly loading requires a modern browser engine (VS Code 1.75+)

## Contributing

Report issues or suggest features on [GitHub](https://github.com/yourusername/rust-pattern-viz/issues).

## License

MIT License - See LICENSE file for details

## Credits

Built on the rust-pattern-viz analyzer by [Your Name]. Inspired by Rustowl and the Rust visualization community.
