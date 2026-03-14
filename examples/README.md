# Rust Pattern Viz Examples

This directory contains example Rust code files demonstrating various pattern matching scenarios that rust-pattern-viz can analyze and visualize.

## Files

### `config.rs`
Complex nested pattern matching example featuring:
- Result<T, E> error handling with match
- Optional value extraction with if let
- Enum variant destructuring with nested fields
- Multiple control flow branches

**Used in**: Main README visualization

### `error_handling.rs`
Idiomatic Rust error handling patterns:
- ? operator usage
- match for explicit error handling
- unwrap_or_else for fallback values
- Custom error types with From implementations

### `iterator_patterns.rs`
Iterator combinator patterns:
- .map() and .filter() chains
- .fold() for aggregation
- Pattern matching in closures
- while let with iterators

### `async_patterns.rs`
Async/await pattern matching:
- tokio::select! macro
- Pattern matching on futures
- Error handling in async contexts

## Generating Visualizations

```bash
# Analyze a single example
rpv analyze examples/config.rs --output-format svg -o config-viz.svg

# Analyze all examples
for file in examples/*.rs; do
    rpv analyze "$file" --output-format svg -o "${file%.rs}-viz.svg"
done

# Generate markdown report
rpv analyze examples/ --output-format markdown > analysis-report.md
```

## Recording Terminal Output

To create GIF demos for the README:

### Option 1: terminalizer

```bash
# Install terminalizer
npm install -g terminalizer

# Record session
terminalizer record demo -c terminalizer.yml

# Edit timing if needed
terminalizer play demo

# Generate GIF
terminalizer render demo -o demo.gif
```

### Option 2: asciinema + agg

```bash
# Install asciinema
brew install asciinema  # or apt-get install asciinema

# Record session
asciinema rec demo.cast

# Convert to GIF using agg
cargo install agg
agg demo.cast demo.gif
```

### Example Recording Script

```bash
#!/bin/bash
# Save as record-demo.sh

clear
echo "# Rust Pattern Viz Demo"
sleep 1

echo ""
echo "$ cat examples/config.rs"
sleep 0.5
bat examples/config.rs --style=plain | head -20
sleep 2

echo ""
echo "$ rpv analyze examples/config.rs"
sleep 0.5
rpv analyze examples/config.rs | head -30
sleep 3

echo ""
echo "$ rpv analyze examples/config.rs --output-format svg -o diagram.svg"
sleep 0.5
rpv analyze examples/config.rs --output-format svg -o diagram.svg
echo "✓ Generated diagram.svg"
sleep 2

echo ""
echo "$ open diagram.svg"
sleep 1
```

## Terminalizer Config

Save as `terminalizer.yml`:

```yaml
command: bash record-demo.sh
cwd: /path/to/rust-pattern-viz
cols: 120
rows: 30
repeat: 0
quality: 100
frameDelay: auto
maxIdleTime: 2000
frameBox:
  type: solid
  title: Rust Pattern Viz
  style:
    boxShadow: none
    margin: 0px
watermark:
  imagePath: null
  style:
    position: absolute
    right: 15px
    bottom: 15px
    width: 100px
    opacity: 0.9
```
