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

## New in v1.1: Control Flow Pattern Detection

### if let and while let Support

The analyzer now detects and visualizes conditional pattern matching expressions:

```rust
// Detection Pipeline for if let/while let:

Source Code
    ↓
syn::parse_file()
    ↓
PatternVisitor traversal
    ↓
visit_expr() → matches Expr::If / Expr::While
    ↓
Check if condition is Expr::Let
    ↓
Extract pattern information:
    - Pattern type (Some(x), Ok(y), custom enum variant)
    - Line range
    - Success/failure branches
    ↓
Create Pattern entry (for detection)
Create DecisionNode with ControlFlow type
    ↓
AnalysisReport with control flow metadata
    ↓
SvgRenderer::render_control_flow_diagram()
    ↓
Flow diagram with:
    - Condition diamond
    - Success branch (right, green)
    - Failure branch (left, red)  
    - Loop back arrow (for while let)
```

### Flow Diagram Rendering

Control flow patterns are rendered as interactive flow diagrams:

```
                    ┌─────────┐
                    │ Pattern │
                    │ Match?  │
                    └────┬────┘
                         │
            ┌────────────┴────────────┐
            │                         │
           Yes                       No
            │                         │
     ┌──────▼──────┐          ┌──────▼──────┐
     │   Execute   │          │  Skip/Exit  │
     │    Block    │          │   (else)    │
     └──────┬──────┘          └─────────────┘
            │
            │ (while let only)
            └─────────────┐
                          │
                    Continue Loop
```

**Visual Elements:**
- **Diamond**: Pattern matching condition
- **Green path**: Success (pattern matched)
- **Red path**: Failure (pattern didn't match)
- **Dashed arrow**: Loop continuation (while let only)

## SVG Renderer Architecture

### Design Goals

1. **Embeddable** - SVG output should work in GitHub READMEs, static sites, and modern browsers
2. **Standalone** - No external dependencies (CSS, fonts) required for rendering
3. **Semantic** - Visual hierarchy reflects code structure (patterns → decisions → imports)
4. **Accessible** - Clear typography, color-coded confidence levels, responsive sizing
5. **Flow Visualization** - Control flow patterns show branching logic clearly

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
│ │ - Standard decision boxes   │ │
│ │ - Control flow diagrams ★   │ │
│ │   • Condition diamonds      │ │
│ │   • Branch arrows           │ │
│ │   • Loop continuations      │ │
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

**Control Flow Colors:**
- **Success path**: Green (#2e7d32)
- **Failure path**: Red (#c62828)
- **Condition**: Orange fill (#fff3e0)

### Layout Algorithm

```rust
1. Calculate content height:
   - Header: fixed 80px
   - Each pattern: 40px + (reasoning lines × 15px)
   - Each standard decision: fixed 140px
   - Each control flow diagram: fixed 160px ★
   - Each import: fixed 60px
   - Spacing: 15px between items, 30px between sections

2. If calculated height > initial height:
   - Dynamically update SVG height attribute

3. Text wrapping:
   - Max 80 characters per line
   - Word boundary breaks
   - Overflow truncation for long content

4. Flow diagram layout ★:
   - Condition diamond: centered at flow_y
   - Success branch: extends right (+150px)
   - Failure branch: extends left (-150px)
   - Loop arrow: curves from success back to top (while let only)
```

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
For each Expr in function bodies:
  ↓
  Match Expr:
    - Expr::If → Check for if let ★
    - Expr::While → Check for while let ★
    - Expr::Match → Analyze match arms
  ↓
Extract:
  - Pattern type (error handling, iterators, control flow ★)
  - Line range (start_line, end_line)
  - Confidence score (heuristic-based)
  - Reasoning (why this pattern was detected)
  - Control flow metadata ★
```

### Confidence Scoring

Confidence is calculated based on:

- **Syntax completeness** (0.7 base for valid syntax)
- **Pattern maturity** (0.8+ for established patterns like Result<T,E>)
- **Import quality** (stdlib imports get higher scores)
- **Code complexity** (simpler patterns score higher)
- **Control flow clarity** (0.85-0.88 for if let/while let) ★

### Decision Tree Building

```rust
For each Pattern:
  Create DecisionNode:
    - Type: ImportChoice | PatternSelection | ErrorHandling | TypeInference | ControlFlow ★
    - Description: Human-readable explanation
    - Alternatives: Other options that were considered
    - Chosen option: What was actually selected
    - Confidence: Aggregate score
    - Reasoning: Why this choice was made ★
```

**New ControlFlow Decision Node:**
- Generated for if let and while let patterns
- Includes alternatives (match, manual loops, combinators)
- Explains idiomatic usage
- Links to flow diagram visualization

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
6. **Expression visitor optimization**: Only descend into function bodies ★

### New Performance Metrics (v1.1)

- **if let detection**: +5ms per expression (typical)
- **while let detection**: +5ms per expression (typical)
- **Flow diagram rendering**: +10ms per control flow node
- **Overall overhead**: <5% for files with <10 control flow patterns

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
    decision_nodes: Vec<DecisionNode>,// Decision tree (includes control flow ★)
    overall_confidence: f64,
    metadata: ReportMetadata
}

Pattern {
    pattern_type: String,  // "Error Handling", "if let", "while let" ★, etc.
    start_line: usize,
    end_line: usize,
    confidence: f64,
    reasoning: Option<String>,
    code_snippet: String
}

DecisionNode {
    id: String,
    decision_type: DecisionType,  // Added ControlFlow variant ★
    description: String,
    alternatives: Vec<String>,
    chosen: String,
    confidence: f64,
    reasoning: Option<String>,    // Enhanced for control flow ★
}

DecisionType {
    ImportChoice,
    PatternSelection,
    ErrorHandling,
    TypeInference,
    ControlFlow,  // New in v1.1 ★
}
```

## Testing Strategy

### Unit Tests

- Pattern detection (`test_if_let_detection`, `test_while_let_detection`) ★
- SVG rendering (`test_svg_rendering_with_control_flow`) ★
- Confidence calculation
- Text wrapping and escaping

### Integration Tests

- Full analysis pipeline with control flow patterns ★
- CLI output formats (JSON, Markdown, SVG with flow diagrams) ★
- LSP hover responses for if let/while let ★

### Visual Regression Tests

- SVG diagram snapshots for control flow patterns ★
- Ensure flow diagrams render correctly across browsers

## Future Enhancements

1. **Interactive SVG**: Clickable branches that highlight corresponding code
2. **Nested pattern support**: Detect patterns within if let guards
3. **Pattern guards**: Visualize if let with additional boolean conditions
4. **Async patterns**: Support for async/await control flow
5. **Performance**: Incremental parsing for large files
6. **ML-based confidence**: Train model on real-world Rust code

---

**Legend:** ★ = New in v1.1 (if let / while let support)
