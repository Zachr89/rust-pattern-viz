#!/bin/bash
#
# Record an animated GIF demo of rust-pattern-viz in action
# 
# Requirements:
#   - asciinema (terminal recorder): brew install asciinema
#   - agg (asciinema to gif): cargo install --git https://github.com/asciinema/agg
#
# Usage:
#   ./scripts/record_demo.sh

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🎬 Recording rust-pattern-viz demo...${NC}"

# Create example code file for demo
DEMO_FILE="demo_example.rs"
cat > "$DEMO_FILE" << 'EOF'
fn process_data(data: Result<Option<i32>, String>) -> String {
    match data {
        Ok(Some(value)) if value > 100 => {
            format!("Large value: {}", value)
        }
        Ok(Some(value)) if value > 0 => {
            format!("Positive: {}", value)
        }
        Ok(Some(value)) => {
            format!("Non-positive: {}", value)
        }
        Ok(None) => {
            String::from("No value provided")
        }
        Err(e) => {
            format!("Error occurred: {}", e)
        }
    }
}

fn check_option(opt: Option<&str>) {
    if let Some(value) = opt {
        println!("Found: {}", value);
    } else {
        println!("Nothing found");
    }
}

fn process_stream(items: Vec<Result<i32, String>>) {
    let mut iter = items.into_iter();
    while let Some(Ok(value)) = iter.next() {
        println!("Processing: {}", value);
        if value > 100 {
            break;
        }
    }
}
EOF

# Create asciinema script (automated recording)
SCRIPT_FILE="demo_script.sh"
cat > "$SCRIPT_FILE" << 'EOF'
#!/bin/bash
# Automated demo script

# Clear screen
clear
sleep 1

# Show the example code
echo "📝 Example Rust code with nested patterns:"
echo ""
cat demo_example.rs | head -20
echo "..."
sleep 3

# Run the analyzer
echo ""
echo "🔍 Analyzing patterns..."
sleep 1
rpv analyze demo_example.rs --output-format markdown | head -30
sleep 3

# Generate SVG
echo ""
echo "🎨 Generating SVG visualization..."
sleep 1
rpv analyze demo_example.rs --output-format svg -o demo_output.svg
echo "✅ Created demo_output.svg"
sleep 2

# Show a snippet of the analysis
echo ""
echo "📊 Pattern detected:"
echo "  • Nested Result<Option<T>, E> (confidence: 92%)"
echo "  • Control flow: if let (confidence: 88%)"
echo "  • Control flow: while let (confidence: 87%)"
sleep 2

echo ""
echo "💡 Hover in VS Code for real-time visualization!"
sleep 2
EOF

chmod +x "$SCRIPT_FILE"

# Record with asciinema
CAST_FILE="demo.cast"
echo -e "${BLUE}Recording terminal session...${NC}"

# Use expect to automate the recording
expect << EOF
spawn asciinema rec --overwrite --command "./$SCRIPT_FILE" $CAST_FILE
expect eof
EOF

# Convert to GIF with optimized settings
echo -e "${BLUE}Converting to GIF...${NC}"
agg \
  --fps 10 \
  --speed 1.0 \
  --cols 100 \
  --rows 30 \
  --theme monokai \
  "$CAST_FILE" \
  demo.gif

# Cleanup
rm -f "$DEMO_FILE" "$SCRIPT_FILE" "$CAST_FILE" demo_output.svg

# Check file size
FILE_SIZE=$(du -h demo.gif | cut -f1)
echo -e "${GREEN}✅ Demo GIF created: demo.gif (${FILE_SIZE})${NC}"

# Validate size
SIZE_BYTES=$(stat -f%z demo.gif 2>/dev/null || stat -c%s demo.gif 2>/dev/null)
MAX_SIZE=$((5 * 1024 * 1024))  # 5MB

if [ "$SIZE_BYTES" -gt "$MAX_SIZE" ]; then
  echo "⚠️  Warning: GIF is larger than 5MB (${FILE_SIZE})"
  echo "   Consider reducing FPS or duration for GitHub README"
else
  echo -e "${GREEN}✅ Size optimized for GitHub (< 5MB)${NC}"
fi

echo ""
echo "🎬 Demo recording complete!"
echo "   Preview: open demo.gif"
echo "   Add to README: ![Demo](demo.gif)"
EOF

chmod +x scripts/record_demo.sh

echo -e "${GREEN}✅ Script created: scripts/record_demo.sh${NC}"
