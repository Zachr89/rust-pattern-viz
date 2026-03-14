# Changelog - Rust Pattern Visualizer VSCode Extension

All notable changes to this extension will be documented in this file.

## [0.1.0] - 2024-01-XX (Beta Release)

### Added
- Initial beta release of VSCode extension
- Hover provider for pattern visualization
  - Displays SVG diagrams on hover over functions, structs, impl blocks
  - Base64-encoded SVG embedding in Markdown hover panels
- Command: "Generate Diagram for Current File"
  - Generates full SVG diagram and opens in split view
  - Saves output as `filename_pattern_diagram.svg`
- Configuration options:
  - `rustPatternViz.cliPath` - Custom path to rpv binary
  - `rustPatternViz.enableHover` - Toggle hover functionality
  - `rustPatternViz.maxDiagramSize` - Maximum diagram size limit
- Auto-detection of CLI binary:
  - Checks user config
  - Checks workspace target/debug and target/release
  - Checks system PATH
- Output channel for debugging
- Welcome message with Pro version preview
- Beta marketplace metadata with "Pro coming soon" messaging

### Technical Details
- Built with TypeScript 5.0
- Uses VSCode API 1.75+
- Spawns `rpv` CLI as subprocess for analysis
- Async/await pattern for all file operations
- Error handling with user-friendly messages

### Known Issues
- Large files (>10k lines) may cause hover delay
- SVG embedding limited by VSCode's Markdown renderer
- No caching between hover requests (will be in Pro)

### Coming in Pro Version
- Real-time analysis as you type
- Interactive SVG with zoom/pan
- Team collaboration features
- Custom themes and styling
- Performance optimizations
- Priority support

---

## Roadmap

### [0.2.0] - Target: Q1 2024 (Free)
- [ ] Cache analysis results per file
- [ ] Add progress indicators for large files
- [ ] Improve error messages with actionable steps
- [ ] Add keyboard shortcut for diagram generation
- [ ] Support for workspace-level analysis

### [1.0.0] - Target: Q2 2024 (Pro Launch)
- [ ] Real-time incremental analysis
- [ ] Webview-based diagram viewer with pan/zoom
- [ ] Team sharing and annotations
- [ ] Custom color themes
- [ ] Export to multiple formats (PNG, PDF)
- [ ] Subscription management UI
- [ ] Analytics dashboard

### Future Considerations
- Language Server Protocol (LSP) integration for better performance
- Inline code lens annotations
- Diff visualization for PRs
- CI/CD integration
- Jupyter notebook support
