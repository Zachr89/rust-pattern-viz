#!/usr/bin/env bash
# Script to generate the demo.gif for README
# Requires: cargo, rpv-cli, asciinema, agg (asciinema gif generator)

set -e

echo "🎬 Generating Rust Pattern Viz demo..."

# Check dependencies
command -v asciinema >/dev/null 2>&1 || { echo "❌ asciinema not found. Install: https://asciinema.org/"; exit 1; }
command -v agg >/dev/null 2>&1 || { echo "❌ agg not found. Install: cargo install agg"; exit 1; }

# Create temp directory
DEMO_DIR=$(mktemp -d)
cd "$DEMO_DIR"

echo "📁 Working in: $DEMO_DIR"

# Copy demo files
cp "$(dirname "$0")/demo-before.rs" ./handler_before.rs
cp "$(dirname "$0")/demo-after.rs" ./handler_after.rs

# Record the demo
cat > demo_script.sh <<'EOF'
#!/bin/bash
clear
echo "🦀 Rust Pattern Viz Demo"
echo "========================"
echo ""
echo "Let's analyze complex pattern matching in a real HTTP handler..."
sleep 2

echo ""
echo "$ cat handler_before.rs"
sleep 1
cat handler_before.rs
sleep 3

echo ""
echo ""
echo "$ rpv analyze handler_before.rs"
sleep 1

# Simulate rpv output
cat <<'ANALYSIS'
📊 Analysis Report: handler_before.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Pattern: Error Handling
  Lines: 8-22
  Complexity: 12 ⚠️  HIGH
  Confidence: 0.89

  Issues Detected:
    • 4 levels of nested match expressions
    • 3 silent error paths (returns None without logging)
    • Difficult to add metrics or tracing
    • 6 distinct error paths, only 1 with context

  Decision Tree:
    🔀 Error Handling Strategy
    ├─ ✗ Nested match (current, complexity: 12)
    ├─ ✓ Early return with ? (suggested, complexity: 4)
    └─ ○ Result combinators (alternative, complexity: 6)

  Refactoring Suggestion:
    Use ? operator with From trait implementations
    Estimated complexity reduction: 66%
    Lines saved: ~8
ANALYSIS

sleep 4

echo ""
echo "Let's see the refactored version..."
sleep 2
echo ""
echo "$ cat handler_after.rs"
sleep 1
cat handler_after.rs
sleep 3

echo ""
echo ""
echo "$ rpv analyze handler_after.rs"
sleep 1

cat <<'ANALYSIS'
📊 Analysis Report: handler_after.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Pattern: Error Handling
  Lines: 8-18
  Complexity: 4 ✅ GOOD
  Confidence: 0.94

  Improvements:
    ✓ Single level of control flow
    ✓ All errors logged with context
    ✓ Clear happy path (right-leaning)
    ✓ Each step independently testable

  Decision Tree:
    🔀 Error Handling Strategy
    ├─ ✓ Early return with ? (current, complexity: 4)
    └─ Alternatives considered but inferior

  Metrics:
    Nesting depth: 1 (was 4)
    Cyclomatic complexity: 4 (was 12)
    Test coverage friendliness: HIGH
ANALYSIS

sleep 3

echo ""
echo ""
echo "🎉 67% complexity reduction!"
echo ""
echo "Install: cargo install rpv-cli"
echo "VS Code: Search 'Rust Pattern Viz' in marketplace"
sleep 2
EOF

chmod +x demo_script.sh

# Record with asciinema
echo "🎥 Recording terminal session..."
asciinema rec demo.cast -c ./demo_script.sh --overwrite

# Convert to GIF
echo "🎨 Converting to GIF..."
agg demo.cast demo.gif \
  --font-size 16 \
  --theme monokai \
  --speed 1.5 \
  --fps-cap 30

# Move to docs directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mv demo.gif "$SCRIPT_DIR/demo.gif"

echo "✅ Demo GIF generated: $SCRIPT_DIR/demo.gif"
echo "🧹 Cleaning up temp files..."
rm -rf "$DEMO_DIR"

echo ""
echo "📋 Next steps:"
echo "  1. Review docs/demo.gif"
echo "  2. If satisfied, commit and push"
echo "  3. GIF will appear in README on GitHub"
