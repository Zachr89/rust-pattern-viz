# Architecture: Rust Pattern Viz

## Overview

Rust Pattern Viz consists of four main components:

1. **Core Library** (`src/analyzer.rs`, `src/models.rs`) - Rust syntax analysis and pattern detection
2. **SVG Renderer** (`src/svg_renderer.rs`) - Converts analysis reports to embeddable SVG diagrams
3. **CLI Tool** (`src/main.rs`) - Command-line interface for batch analysis
4. **LSP Server + VS Code Extension** (`src/lsp_server.rs`, `vscode-extension/`) - Real-time editor integration

## Component Interaction

```
┌─────────────────┐
│   VS Code UI    │
│  (User hovers)  │
└────────┬────────┘
         │
         │ LSP Protocol (JSON-RPC over stdio)
         │
┌────────▼────────────────────────────┐
│  rpv-lsp (Rust Binary)              │
│  ┌──────────────────────────────┐   │
│  │ tower-lsp Server             │   │
│  │ - textDocument/hover         │   │
│  │ - textDocument/didOpen       │   │
│  │ - textDocument/didChange     │   │
│  └──────────┬───────────────────┘   │
│             │                        │
│  ┌──────────▼───────────────────┐   │
│  │ Document Cache (in-memory)   │   │
│  │ HashMap<Url, String>         │   │
│  └──────────┬───────────────────┘   │
│             │                        │
│  ┌──────────▼───────────────────┐   │
│  │ CodeAnalyzer                 │   │
│  │ - syn-based parsing          │   │
│  │ - Pattern detection          │   │
│  │ - Confidence scoring         │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
         │
         │ Returns Markdown
         │
┌────────▼────────┐
│  VS Code UI     │
│  (Shows hover)  │
└─────────────────┘

┌─────────────────┐
│   CLI (rpv)     │
│  analyze cmd    │
└────────┬────────┘
         │
         │ --output-format flag
         │
┌────────▼────────────────────────────┐
│  Output Formatting Pipeline         │
│  ┌──────────────────────────────┐   │
│  │ CodeAnalyzer                 │   │
│  │ → AnalysisReport             │   │
│  └──────────┬───────────────────┘   │
│             │                        │
│  ┌──────────▼───────────────────┐   │
│  │ Format Selection:            │   │
│  │ - JSON (serde_json)          │   │
│  │ - Markdown (string builder)  │   │
│  │ - SVG (SvgRenderer)          │   │
│  └──────────┬───────────────────┘   │
│             │                        │
│  ┌──────────▼───────────────────┐   │
│  │ Output Writer:               │   │
│  │ - stdout (default)           │   │
│  │ - file path (-o flag)        │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
         │
         │ stdout or file
         │
┌────────▼────────┐
│  User / Docs    │
└─────────────────┘
```

## SVG Renderer Architecture

### Design Goals

1. **Embeddable** - SVG output should work in GitHub READMEs, static sites, and modern browsers
2. **Standalone** - No external dependencies (CSS, fonts) required for rendering
3. **Semantic** - Visual hierarchy reflects code structure (patterns → decisions → imports)
4. **Accessible** - Clear typography, color-coded confidence levels, responsive sizing

### Rendering Pipeline

```rust
AnalysisReport
    ↓
SvgRenderer::render()
    ↓
Layout Calculation
    ↓
┌─────────────────────────────────┐
│ SVG Structure                   │
│ ┌─────────────────────────────┐ │
│ │ Header (title, metadata)    │ │
│ └─────────────────────────────┘ │
│ ┌─────────────────────────────┐ │
│ │ Patterns Section            │ │
│ │ - Boxes with confidence     │ │
│ │ - Color-coded backgrounds   │ │
│ │ - Wrapped text              │ │
│ └─────────────────────────────┘ │
│ ┌─────────────────────────────┐ │
│ │ Decision Nodes Section      │ │
│ │ - Decision type badges      │ │
│ │ - Alternative choices       │ │
│ └─────────────────────────────┘ │
│ ┌─────────────────────────────┐ │
│ │ Imports Section             │ │
│ │ - Category grouping         │ │
│ │ - Path display              │ │
│ └─────────────────────────────┘ │
└─────────────────────────────────┘
    ↓
XML String Output
    ↓
File or stdout
```

### Color Scheme

- **High Confidence (≥80%)**: Light green (#c8e6c9) - Established patterns
- **Medium Confidence (50-79%)**: Light yellow (#fff9c4) - Emerging patterns
- **Low Confidence (<50%)**: Light orange (#ffccbc) - Experimental patterns

- **Pattern boxes**: Blue border (#1976d2)
- **Decision boxes**: Orange border (#f57c00)
- **Import boxes**: Purple border (#7b1fa2)

### Layout Algorithm

```rust
1. Calculate content height:
   - Header: fixed 80px
   - Each pattern: 40px + (reasoning lines × 15px)
   - Each decision: fixed 95px
   - Each import: fixed 60px
   - Spacing: 15px between items, 30px between sections

2. If calculated height > initial height:
   - Dynamically update SVG height attribute

3. Text wrapping:
   - Max 80 characters per line
   - Word boundary breaks
   - Overflow truncation for long content
```

## LSP Server Architecture

### Request Flow

1. **Document Open/Change**
   - User opens or edits a `.rs` file
   - VS Code sends `textDocument/didOpen` or `textDocument/didChange`
   - LSP server caches full document text in memory

2. **Hover Request**
   - User hovers over a function/struct/impl
   - VS Code sends `textDocument/hover` with position
   - LSP server:
     - Checks if hovering over relevant construct (function, struct, impl)
     - Retrieves cached document text
     - Calls `CodeAnalyzer::analyze()` with full source
     - Formats results as Markdown
     - Returns hover response

3. **Analysis Pipeline**
   ```rust
   Document Text
       ↓
   syn::parse_file()
       ↓
   PatternVisitor (AST traversal)
       ↓
   Pattern Detection
       ↓
   Import Analysis
       ↓
   Decision Tree Building
       ↓
   AnalysisReport
       ↓
   Markdown Formatting (LSP) OR SVG Rendering (CLI)
       ↓
   Hover Response / File Output
   ```

## CLI Architecture

### Command Flow

```bash
rpv analyze file.rs --output-format svg -o diagram.svg
```

```rust
1. Parse CLI args (clap)
   - file: PathBuf
   - output_format: OutputFormat enum (Json | Markdown | Svg)
   - output: Option<PathBuf>
   - pretty: bool

2. Read source file
   - fs::read_to_string(file)

3. Analyze
   - CodeAnalyzer::new()
   - analyzer.analyze(source, path)
   - Returns AnalysisReport

4. Format output
   - Match output_format:
     - Json → serde_json::to_string[_pretty]
     - Markdown → custom formatter
     - Svg → SvgRenderer::render()

5. Write output
   - If output path provided:
     - fs::write(output, content)
   - Else:
     - stdout.write_all(content)
```

## VS Code Extension Architecture

### Extension Lifecycle

```typescript
activate()
  ↓
Find rpv-lsp binary (auto-detect or config)
  ↓
Create LanguageClient
  ↓
Start LSP server process
  ↓
Register commands (restart, etc.)
  ↓
Extension ready

User hovers → VS Code → LSP Client → rpv-lsp → Analysis → Response
```

### Binary Discovery

The extension searches for `rpv-lsp` in this order:

1. User-configured path (`rustPatternViz.serverPath`)
2. Workspace `target/debug/rpv-lsp`
3. Workspace `target/release/rpv-lsp`
4. System PATH

## Core Analyzer Design

### Pattern Detection

```rust
PatternVisitor::visit_file(&File)
  ↓
For each Item in File:
  ↓
  Match Item:
    - Fn → Analyze function patterns
    - Impl → Analyze method patterns
    - Struct → Analyze data patterns
    - Use → Analyze imports
  ↓
Extract:
  - Pattern type (error handling, iterators, etc.)
  - Line range (start_line, end_line)
  - Confidence score (heuristic-based)
  - Reasoning (why this pattern was detected)
```

### Confidence Scoring

Confidence is calculated based on:

- **Syntax completeness** (0.7 base for valid syntax)
- **Pattern maturity** (0.8+ for established patterns like Result<T,E>)
- **Import quality** (stdlib imports get higher scores)
- **Code complexity** (simpler patterns score higher)

### Decision Tree Building

```rust
For each Pattern:
  Create DecisionNode:
    - Type: ImportChoice | PatternSelection | ErrorHandling | TypeInference
    - Description: Human-readable explanation
    - Alternatives: Other options that were considered
    - Chosen option: What was actually selected
    - Confidence: Aggregate score
```

## Performance Considerations

### Caching Strategy

- **Document caching**: Full text stored in memory per open file
- **No persistent cache**: Analysis is stateless and fast enough (<100ms typical)
- **Lazy analysis**: Only analyze on hover (not on every keystroke)

### Optimization Points

1. **Parse once per hover**: Don't re-parse unchanged documents
2. **Position-aware triggers**: Only show hover on relevant lines (fn, struct, impl)
3. **Async analysis**: LSP server uses Tokio async runtime
4. **Efficient AST traversal**: syn's visitor pattern minimizes allocations
5. **SVG streaming**: Large diagrams written incrementally (no full DOM tree)

## Error Handling

### LSP Server

- Parse errors → Return None (no hover)
- Analysis errors → Log to stderr, return None
- Connection errors → Graceful shutdown

### CLI

- File read errors → Exit with error message
- Parse errors → Exit with syntax error details
- Write errors → Exit with I/O error message

### Extension

- Server not found → Show error notification with setup instructions
- Server crash → Auto-restart capability via command
- Timeout → No hover shown (LSP client handles timeout)

## Data Models

### Core Types

```rust
AnalysisReport {
    file_path: String,
    timestamp: String,
    patterns: Vec<Pattern>,           // Detected code patterns
    import_suggestions: Vec<Import>,  // Import analysis
    decision_nodes: Vec<DecisionNode>,// Decision tree
    overall_confidence: f64,
    metadata: ReportMetadata
}

Pattern {
    pattern_type: String,  // "Error Handling", "Iterator Chain", etc.
    start_line: usize,
    end_line: usize,
    confidence: f64,
    reasoning: Option<String>,
    code_snippet: String
}

DecisionNode {
    id: String,
    decision_type: DecisionType,
    description: String,
    alternatives: Vec<Alternative>,
    chosen: String,
    confidence: f64
}
```

## Extension Points

### Adding New Patterns

1. Add pattern detection logic in `PatternVisitor`
2. Update `DecisionType` enum if needed
3. Add formatting logic in `Backend::format_hover_content()`
4. Update SVG renderer if new visualization needed

### Supporting New Output Formats

1. Add new variant to `OutputFormat` enum
2. Implement formatter in `format_output()`
3. Add CLI documentation
4. Add example to README

### Supporting New Editors

The LSP server is editor-agnostic. To support a new editor:

1. Write editor-specific LSP client (like the VS Code extension)
2. Point it to `rpv-lsp` binary
3. Handle hover requests per editor's API

### Custom Visualizations

The `AnalysisReport` can be serialized to JSON:

```rust
let report = analyzer.analyze(source, path)?;
let json = serde_json::to_string(&report)?;
// Send to custom visualization tool or web frontend
```

## Testing Strategy

### Unit Tests
- Pattern detection accuracy
- Confidence scoring correctness
- Import analysis logic
- SVG XML validity
- Text wrapping algorithm
- Color scheme mapping

### Integration Tests
- LSP server protocol compliance
- Extension activation
- End-to-end hover flow
- CLI output format correctness
- SVG rendering in browsers

### Performance Tests
- Analysis speed on large files
- Memory usage with many open documents
- Hover latency benchmarks
- SVG generation time for complex reports

## Future Architecture Enhancements

1. **Incremental parsing**: Only re-analyze changed functions
2. **Persistent cache**: Store analysis results per file hash
3. **Code lens support**: Show inline pattern annotations
4. **Diagnostics**: Warn about low-confidence patterns
5. **Quick fixes**: Suggest pattern improvements
6. **Multi-language**: Extend to other languages via tree-sitter
7. **Interactive SVG**: Add JavaScript for collapsible sections
8. **Web dashboard**: Real-time project-wide pattern analytics
9. **CI/CD integration**: Pattern quality gates in pipelines
10. **Theme support**: Light/dark SVG themes

## Security Considerations

### Input Validation
- All file paths sanitized before reading
- SVG output XML-escaped to prevent injection
- No eval or dynamic code execution

### Resource Limits
- Maximum file size for analysis: 10MB
- SVG generation timeout: 5 seconds
- Memory limit per analysis: 100MB

### Credentials
- No hardcoded tokens in source code
- LSP server runs with user permissions only
- No network access required for core functionality
