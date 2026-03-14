# Rust Pattern Visualizer - VSCode Extension (Beta)

> **🎯 Pro Version Coming Soon**: This beta preview demonstrates hover-based pattern visualization. The full Pro version will include real-time updates, team collaboration, and enhanced diagram customization.

## Features

### ✨ Hover Pattern Diagrams (Beta)

Hover over Rust functions, structs, or impl blocks to instantly see:
- **Error handling patterns** (Result, Option, unwrap chains)
- **Iterator chains** (map, filter, collect)
- **Control flow** (match, if-let)
- **Decision trees** showing code reasoning

![Demo](https://via.placeholder.com/800x400?text=Hover+Demo+Screenshot)

### 📊 Full File Analysis

Generate complete pattern diagrams for entire files with the command palette:
- Open Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`)
- Run: **"Rust Pattern Viz: Generate Diagram for Current File"**
- View SVG output in split view or browser

## Installation

### Prerequisites

1. **Install the Rust CLI tool** (required - extension calls this binary):
   ```bash
   git clone https://github.com/yourusername/rust-pattern-viz
   cd rust-pattern-viz
   cargo build --release
   cargo install --path .
   ```

2. **Verify installation**:
   ```bash
   rpv --version
   ```

### Extension Setup

#### From VSCode Marketplace (Coming Soon)
1. Search "Rust Pattern Visualizer" in Extensions
2. Click Install
3. Reload VSCode

#### Manual Installation (Beta)
1. Download `.vsix` from [releases](https://github.com/yourusername/rust-pattern-viz/releases)
2. VSCode → Extensions → `...` → Install from VSIX
3. Select downloaded file

## Usage

### Hover Analysis
1. Open any `.rs` file
2. Hover over `fn`, `impl`, `struct`, or `enum` keywords
3. View pattern diagram in hover panel
4. Click links in hover for settings or full diagram

### Generate Full Diagram
- **Command Palette**: `Rust Pattern Viz: Generate Diagram for Current File`
- **Or**: Right-click in editor → future context menu option
- Output: SVG file saved next to source file

### Configuration

Open settings (`Ctrl+,`) and search for "Rust Pattern Viz":

- **`rustPatternViz.cliPath`**: Path to `rpv` binary (auto-detected if installed globally)
- **`rustPatternViz.enableHover`**: Enable/disable hover diagrams
- **`rustPatternViz.maxDiagramSize`**: Maximum diagram height (default: 5000px)

Example settings.json:
```json
{
  "rustPatternViz.cliPath": "/usr/local/bin/rpv",
  "rustPatternViz.enableHover": true,
  "rustPatternViz.maxDiagramSize": 8000
}
```

## Monetization Roadmap

### ✅ Free (Current Beta)
- CLI tool (MIT licensed)
- Basic hover diagrams
- SVG export
- Community support

### 🚀 Pro (Coming Q2 2024)
- **Real-time updates** as you type
- **Team collaboration** - share diagrams with annotations
- **Enhanced visualizations** - interactive SVG with zoom/pan
- **Custom themes** - match your editor colors
- **Priority support** - direct Slack channel
- **License**: Subscription ($10/month or $100/year)

[**Join Pro Waitlist**](https://your-landing-page.com/waitlist) - Get 50% off for early supporters!

## Troubleshooting

### "rpv CLI tool not found"

**Solution**: Ensure `rpv` is installed and in PATH:
```bash
# Check if installed
which rpv  # macOS/Linux
where rpv  # Windows

# If not found, install
cargo install --path .

# Or configure path in settings
# VSCode Settings → rustPatternViz.cliPath → /path/to/rpv
```

### "Diagram not showing"

1. Check Output panel: View → Output → "Rust Pattern Viz"
2. Verify file is valid Rust (compiles without syntax errors)
3. Try generating full diagram via command palette
4. Restart VSCode: Developer → Reload Window

### "SVG too large"

Increase limit in settings:
```json
{
  "rustPatternViz.maxDiagramSize": 10000
}
```

## Development

### Building from Source

```bash
cd vscode-extension
npm install
npm run compile
```

### Running Extension Dev Host

1. Open `vscode-extension/` in VSCode
2. Press F5 to launch Extension Development Host
3. Open a Rust file in the new window
4. Test hover functionality

### Packaging

```bash
npm install -g @vscode/vsce
cd vscode-extension
vsce package
# Creates rust-pattern-viz-0.1.0.vsix
```

## Contributing

We're actively developing the Pro version! Areas we need help:

- **UI/UX**: Diagram layout improvements
- **Performance**: Faster analysis for large files
- **Testing**: Edge cases in pattern detection
- **Documentation**: More examples and tutorials

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

## Feedback

- **Issues**: [GitHub Issues](https://github.com/yourusername/rust-pattern-viz/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/rust-pattern-viz/discussions)
- **Twitter**: [@yourhandle](https://twitter.com/yourhandle)

## License

- **Extension**: MIT License (free forever)
- **CLI Tool**: MIT License
- **Pro Features**: Proprietary (subscription required)

---

**Made with 🦀 by the Rust Pattern Viz team**

[Website](https://your-site.com) • [Docs](https://docs.your-site.com) • [Changelog](CHANGELOG.md)
