# Contributing to rust-pattern-viz

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## Quick Links

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [High-Priority Tasks](#high-priority-tasks)

## Code of Conduct

This project adheres to the Rust Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to dev@example.com.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Node.js 18+ (for VS Code extension)
- VS Code (for testing the extension)
- Git

### Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/rust-pattern-viz.git
cd rust-pattern-viz
git remote add upstream https://github.com/ORIGINAL_OWNER/rust-pattern-viz.git
```

### Build Everything

```bash
# Build Rust components
cargo build

# Build VS Code extension
cd vscode-extension
npm install
npm run compile
cd ..

# Run tests
cargo test
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-123
```

### 2. Make Changes

- Write clear, commented code
- Follow Rust naming conventions
- Add tests for new functionality
- Update documentation as needed

### 3. Test Locally

```bash
# Run all tests
cargo test

# Test the LSP server
cargo run --bin rpv-lsp

# Test in VS Code
cd vscode-extension
code . # Opens extension development host
# Press F5 to launch Extension Development Host
```

### 4. Commit

```bash
git add .
git commit -m "feat: add support for lifetime patterns"
# or
git commit -m "fix: handle nested match expressions correctly"
```

**Commit Message Format:**
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Build process, dependencies

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then open a Pull Request on GitHub.

## Testing

### Unit Tests

```bash
# Run all unit tests
cargo test

# Run specific test
cargo test test_pattern_detection

# Run with output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Test CLI
cargo run --bin rpv analyze tests/fixtures/sample.rs

# Test LSP server (requires manual testing in VS Code)
cargo build --bin rpv-lsp
# Follow "Test in VS Code" steps above
```

### Add New Tests

Place test files in:
- `tests/` - Integration tests
- `src/*/tests.rs` - Unit tests alongside code

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_result_pattern() {
        let source = r#"
            fn example() -> Result<i32, String> {
                Ok(42)
            }
        "#;
        
        let analyzer = CodeAnalyzer::new();
        let report = analyzer.analyze(source, "test.rs").unwrap();
        
        assert!(report.patterns.iter().any(|p| 
            p.pattern_type == "Error Handling"
        ));
    }
}
```

## Pull Request Process

### Before Submitting

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated (if applicable)
- [ ] CHANGELOG.md updated (if user-facing change)

### PR Description Template

```markdown
## Description
Brief description of changes

## Motivation
Why is this change necessary?

## Changes Made
- Added X
- Fixed Y
- Refactored Z

## Testing
How was this tested?

## Screenshots (if applicable)
Before/after images

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
```

### Review Process

1. Maintainer reviews PR within 48 hours
2. Address feedback
3. Maintainer approves and merges
4. Changes appear in next release

## High-Priority Tasks

### 🎬 URGENT: Record Demo GIF

**Impact:** Critical for first-time visitors
**See:** [DEMO_RECORDING.md](./DEMO_RECORDING.md)

We need a 15-second GIF showing:
1. Typing a match expression
2. Real-time visualization
3. Catching an exhaustiveness bug

**How to help:**
- Record the demo following the guide
- Submit PR with `assets/demo.gif`
- Update README to show the GIF

**Estimated time:** 30-60 minutes

---

### Other High-Value Contributions

#### Pattern Detectors

Add support for new patterns:
- Lifetime patterns
- Trait bound patterns
- Async/await patterns
- Macro invocations

**File:** `src/analyzer.rs`

#### Performance Optimization

- Implement incremental parsing
- Add caching layer
- Benchmark large files (1000+ lines)

**File:** `src/analyzer.rs`, `src/lsp_server.rs`

#### Documentation

- Add more examples
- Create video tutorials
- Write blog posts
- Translate README

#### VS Code Extension Features

- Add code lens support
- Implement quick fixes
- Add diagnostic warnings
- Create settings UI

**Directory:** `vscode-extension/`

## Code Style

### Rust

Follow standard Rust conventions:
```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy
```

### TypeScript (Extension)

```bash
cd vscode-extension
npm run lint
npm run format
```

## Project Structure

```
rust-pattern-viz/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lsp_server.rs    # LSP server entry point
│   ├── analyzer.rs      # Core analysis logic
│   ├── models.rs        # Data structures
│   └── lib.rs          # Library exports
├── vscode-extension/
│   ├── src/
│   │   └── extension.ts # Extension logic
│   └── package.json     # Extension manifest
├── tests/
│   └── integration_test.rs
├── assets/
│   └── demo.gif        # Demo GIF (to be added)
└── docs/
    └── ARCHITECTURE.md  # Design documentation
```

## Adding Dependencies

### Rust

```bash
cargo add <dependency-name>
```

Only add dependencies if:
- Widely used and maintained
- Necessary for functionality
- Doesn't duplicate existing deps

### TypeScript

```bash
cd vscode-extension
npm install <package-name>
```

## Documentation

### Code Comments

```rust
/// Analyzes Rust source code for pattern matching constructs.
/// 
/// # Arguments
/// * `source` - The source code to analyze
/// * `file_path` - Path to the file (for error reporting)
/// 
/// # Returns
/// * `AnalysisReport` containing detected patterns and decision trees
/// 
/// # Example
/// ```
/// let analyzer = CodeAnalyzer::new();
/// let report = analyzer.analyze(source, "main.rs")?;
/// ```
pub fn analyze(&self, source: &str, file_path: &str) -> Result<AnalysisReport> {
    // ...
}
```

### Architecture Changes

Update [ARCHITECTURE.md](./ARCHITECTURE.md) when making structural changes.

## Release Process

(Maintainers only)

1. Update version in `Cargo.toml` and `vscode-extension/package.json`
2. Update CHANGELOG.md
3. Create git tag: `git tag v0.2.0`
4. Push tag: `git push --tags`
5. GitHub Actions will build and publish

## Getting Help

- **GitHub Discussions**: For questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **Email**: dev@example.com for private inquiries

## Recognition

Contributors are recognized in:
- CHANGELOG.md (per release)
- README.md (significant contributions)
- GitHub contributor graph

Thank you for contributing! 🎉
