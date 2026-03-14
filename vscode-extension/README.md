# Rust Pattern Visualizer for VS Code

Visualize AI-generated code patterns and decision trees directly in your editor with hover tooltips.

## Features

- **Hover Analysis**: Hover over Rust functions, structs, or impl blocks to see:
  - Pattern complexity scores
  - AI confidence ratings
  - Decision tree summaries
  - Import analysis
  
- **Real-time Insights**: Get immediate feedback on code patterns as you write
- **Non-intrusive**: Works seamlessly with rust-analyzer
- **Local Processing**: All analysis happens locally—no cloud uploads

## Installation

### From VS Code Marketplace

1. Search for "Rust Pattern Visualizer" in the Extensions view
2. Click Install

### From Source

1. Build the LSP server:
   ```bash
   cd /path/to/rust-pattern-viz
   cargo build --release --bin rpv-lsp
   ```

2. Install the extension:
   ```bash
   cd vscode-extension
   npm install
   npm run compile
   code --install-extension $(npm run package | tail -1)
   ```

## Usage

1. Open any Rust file in VS Code
2. Hover over a function definition, struct, or impl block
3. See the pattern analysis tooltip appear automatically

## Configuration

Open VS Code settings and search for "Rust Pattern Viz":

- `rustPatternViz.enable`: Enable/disable the extension (default: true)
- `rustPatternViz.serverPath`: Custom path to rpv-lsp binary (auto-detected by default)
- `rustPatternViz.trace.server`: Debug LSP communication (default: off)

## Requirements

- VS Code 1.75.0 or higher
- Rust toolchain (for building the LSP server)

## Development

```bash
# Build LSP server
cargo build --bin rpv-lsp

# Build extension
cd vscode-extension
npm install
npm run compile

# Debug in VS Code
# Press F5 to open Extension Development Host
```

## Troubleshooting

### "Could not find rpv-lsp binary"

Build the LSP server:
```bash
cargo build --release --bin rpv-lsp
```

Or set a custom path in settings:
```json
{
  "rustPatternViz.serverPath": "/path/to/rpv-lsp"
}
```

### Hover tooltips not appearing

1. Make sure the extension is enabled in settings
2. Check that you're hovering over supported constructs (functions, structs, impls)
3. Restart the language server: `Cmd+Shift+P` → "Restart Pattern Viz Server"

## License

MIT - See [LICENSE](../LICENSE) for details
