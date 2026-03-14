# VS Code Extension Development Guide

## Overview

The Rust Pattern Visualizer VS Code extension provides a sidebar panel that displays real-time pattern analysis and visualizations for Rust code. It wraps the core Rust analyzer as a WebAssembly module, enabling fast in-browser analysis without external dependencies.

## Architecture

```
VS Code Extension (TypeScript)
    ↓
WebView (HTML + JavaScript)
    ↓
WASM Module (Rust → wasm-bindgen)
    ↓
Core Analyzer (Rust)
    ↓
SVG Renderer (Rust)
    ↓
Display in WebView
```

## Component Breakdown

### Extension Host (`extension.ts`)
- **Activation**: Triggered when Rust files are opened
- **Command Registration**: Refresh and export commands
- **Event Listeners**: File changes, editor switches
- **Lifecycle Management**: WebView creation and disposal

### WebView Provider (`viewProvider.ts`)
- **WebView Management**: Creates and configures the visualization panel
- **Message Handling**: Bidirectional communication with WebView
- **Content Updates**: Sends code to WASM for analysis
- **SVG Storage**: Caches current visualization for export

### WebView Content (HTML + JS in `viewProvider.ts`)
- **WASM Loading**: Initializes the Rust analyzer module
- **Analysis Trigger**: Receives code from extension, calls WASM functions
- **DOM Rendering**: Displays SVG output from WASM
- **Error Handling**: Shows user-friendly error messages

### WASM Module (`lib.rs`)
- **Entry Points**:
  - `analyze_code_to_svg()` - Returns SVG visualization
  - `analyze_code_to_json()` - Returns JSON report
- **Panic Hook**: Better error messages in browser console
- **Memory Management**: Efficient string passing via wasm-bindgen

## Development Workflow

### 1. Setup

```bash
cd vscode-extension
npm install
```

### 2. Build WASM Module

```bash
npm run build:wasm
# Internally runs: wasm-pack build --target web --out-dir vscode-extension/wasm
```

This creates:
- `wasm/rust_pattern_viz_bg.wasm` - Compiled Rust code
- `wasm/rust_pattern_viz.js` - JavaScript bindings
- `wasm/rust_pattern_viz.d.ts` - TypeScript definitions

### 3. Compile Extension

```bash
npm run compile
# Or watch mode: npm run watch
```

### 4. Test in VS Code

Press `F5` in VS Code to launch the Extension Development Host. This opens a new VS Code window with the extension loaded.

**Test checklist**:
- [ ] Extension activates when opening `.rs` files
- [ ] Sidebar panel appears in activity bar
- [ ] Opening a Rust file triggers analysis
- [ ] SVG renders correctly in panel
- [ ] Refresh command works
- [ ] Export command saves SVG file
- [ ] Configuration changes take effect
- [ ] No console errors in Dev Tools (Help > Toggle Developer Tools)

### 5. Package for Distribution

```bash
npm run vscode:prepublish
# Creates: rust-pattern-viz-1.0.0.vsix
```

## Configuration

Users can customize behavior via VS Code settings:

```json
{
  "rustPatternViz.autoRefresh": true,
  "rustPatternViz.debounceDelay": 500,
  "rustPatternViz.showConfidenceScores": true
}
```

Access in code:
```typescript
const config = vscode.workspace.getConfiguration('rustPatternViz');
const delay = config.get<number>('debounceDelay', 500);
```

## WebView Security

The extension uses a restrictive Content Security Policy:

```html
<meta http-equiv="Content-Security-Policy" 
      content="default-src 'none'; 
               style-src ${webview.cspSource} 'unsafe-inline'; 
               script-src ${webview.cspSource} 'unsafe-inline'; 
               img-src ${webview.cspSource} data:;">
```

**Why `unsafe-inline`?**
- Required for embedding SVG (data URIs)
- Required for inline script that loads WASM module
- WebView is sandboxed, so risk is minimal

**Resource Loading**:
All external resources (WASM, JS) must use `webview.asWebviewUri()` to get proper URIs:

```typescript
const wasmUri = webview.asWebviewUri(
    vscode.Uri.joinPath(this._extensionUri, 'wasm', 'rust_pattern_viz_bg.wasm')
);
```

## Message Passing

### Extension → WebView

```typescript
this._view.webview.postMessage({
    command: 'analyze',
    code: document.getText(),
    fileName: document.fileName
});
```

### WebView → Extension

```typescript
vscode.postMessage({ 
    command: 'error', 
    error: 'Analysis failed' 
});
```

Received in extension:
```typescript
webviewView.webview.onDidReceiveMessage(message => {
    switch (message.command) {
        case 'error':
            vscode.window.showErrorMessage(message.error);
            break;
    }
});
```

## Performance Optimization

### Debouncing
Prevent analysis on every keystroke:

```typescript
let debounceTimer: NodeJS.Timeout | undefined;
vscode.workspace.onDidChangeTextDocument(event => {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
        viewProvider?.updateContent(event.document);
    }, 500);
});
```

### WebView Retention
Keep WebView alive when hidden:

```typescript
vscode.window.registerWebviewViewProvider(
    'rustPatternViz.view',
    viewProvider,
    { webviewOptions: { retainContextWhenHidden: true } }
);
```

### WASM Size
The production build optimizes WASM size:

```toml
[profile.release]
opt-level = "z"
lto = true
strip = true

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz"]
```

Current size: ~150KB (compressed)

## Troubleshooting

### WASM Module Not Loading

**Symptoms**: White panel, "WASM module not initialized" error

**Solutions**:
1. Check WASM files exist: `ls vscode-extension/wasm/`
2. Rebuild WASM: `npm run build:wasm`
3. Check browser console (Help > Toggle Developer Tools)
4. Verify CSP isn't blocking resources

### Analysis Takes Too Long

**Symptoms**: "Analyzing..." never completes

**Solutions**:
1. Check file size (>10k lines may be slow)
2. Look for complex macros (syn may struggle)
3. Check WASM console for Rust panics
4. Increase `debounceDelay` in settings

### SVG Not Rendering

**Symptoms**: Empty panel or broken image

**Solutions**:
1. Check SVG syntax in browser inspector
2. Verify `analyze_code_to_svg()` returns valid SVG
3. Check for JavaScript errors in console
4. Test with simple Rust code first

## Publishing to Marketplace

### Prerequisites
1. Create a publisher account at https://marketplace.visualstudio.com/manage
2. Get a Personal Access Token (PAT) from Azure DevOps
3. Install vsce: `npm install -g @vscode/vsce`

### Steps

```bash
# Login
vsce login your-publisher-name

# Publish
vsce publish

# Or publish specific version
vsce publish 1.0.1
```

### Before Publishing
- [ ] Update version in `package.json`
- [ ] Test on Windows, macOS, Linux
- [ ] Update README with screenshots
- [ ] Add CHANGELOG.md
- [ ] Verify icon.svg displays correctly
- [ ] Test installation from VSIX

### Continuous Deployment

Set up GitHub Actions to auto-publish on release:

```yaml
name: Publish Extension
on:
  release:
    types: [created]
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm ci
      - run: npm run vscode:prepublish
      - run: npx vsce publish -p ${{ secrets.VSCE_PAT }}
```

## Future Enhancements

- [ ] Add hover preview (show pattern viz on hover)
- [ ] Integrate with CodeLens (inline annotations)
- [ ] Add "Jump to Definition" for patterns
- [ ] Support custom color themes
- [ ] Add pattern complexity metrics
- [ ] Export to PNG/PDF (via SVG conversion)
- [ ] Multi-file project analysis
- [ ] Pattern search across workspace

## Resources

- [VS Code Extension API](https://code.visualstudio.com/api)
- [WebView API Guide](https://code.visualstudio.com/api/extension-guides/webview)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [Extension Publishing](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)
