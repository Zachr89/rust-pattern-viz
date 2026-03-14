# Architecture: Rust Pattern Viz

## Overview

Rust Pattern Viz consists of three main components:

1. **Core Library** (`src/analyzer.rs`, `src/models.rs`) - Rust syntax analysis and pattern detection
2. **CLI Tool** (`src/main.rs`) - Command-line interface for batch analysis
3. **LSP Server + VS Code Extension** (`src/lsp_server.rs`, `vscode-extension/`) - Real-time editor integration

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
   Markdown Formatting
       ↓
   Hover Response
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

## Error Handling

### LSP Server

- Parse errors → Return None (no hover)
- Analysis errors → Log to stderr, return None
- Connection errors → Graceful shutdown

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
// Send to custom visualization tool
```

## Testing Strategy

### Unit Tests
- Pattern detection accuracy
- Confidence scoring correctness
- Import analysis logic

### Integration Tests
- LSP server protocol compliance
- Extension activation
- End-to-end hover flow

### Performance Tests
- Analysis speed on large files
- Memory usage with many open documents
- Hover latency benchmarks

## Future Architecture Enhancements

1. **Incremental parsing**: Only re-analyze changed functions
2. **Persistent cache**: Store analysis results per file hash
3. **Code lens support**: Show inline pattern annotations
4. **Diagnostics**: Warn about low-confidence patterns
5. **Quick fixes**: Suggest pattern improvements
6. **Multi-language**: Extend to other languages via tree-sitter
