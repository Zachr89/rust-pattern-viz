#!/bin/bash
#
# Manual demo GIF creation (for when automated recording doesn't work)
# This provides step-by-step instructions for manual recording

cat << 'EOF'
📹 Manual Demo GIF Creation Guide
==================================

Since automated recording can be tricky, here's how to create the demo GIF manually:

## Step 1: Install Tools

```bash
# Install asciinema (terminal recorder)
brew install asciinema  # macOS
# or
apt install asciinema   # Ubuntu/Debian

# Install agg (asciinema to gif converter)
cargo install --git https://github.com/asciinema/agg
```

## Step 2: Create Demo Code

Create a file `demo_example.rs` with this content:

```rust
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
        Ok(None) => String::from("No value"),
        Err(e) => format!("Error: {}", e),
    }
}

fn check_option(opt: Option<&str>) {
    if let Some(value) = opt {
        println!("Found: {}", value);
    }
}
```

## Step 3: Record Session

```bash
# Start recording
asciinema rec demo.cast

# Now perform these actions (naturally, with pauses):

# 1. Show the code
clear
echo "📝 Complex nested pattern example:"
cat demo_example.rs
# (pause 2 seconds)

# 2. Run the analyzer
echo ""
echo "🔍 Analyzing patterns..."
rpv analyze demo_example.rs --output-format markdown
# (pause 2 seconds)

# 3. Generate SVG
echo ""
echo "🎨 Generating visualization..."
rpv analyze demo_example.rs --output-format svg -o output.svg
echo "✅ Done! Check output.svg"
# (pause 1 second)

# 4. Exit recording
# Press Ctrl+D or type 'exit'
```

## Step 4: Convert to GIF

```bash
agg --fps 10 --speed 1.0 --theme monokai demo.cast demo.gif
```

## Step 5: Optimize Size

If the GIF is > 5MB:

```bash
# Reduce FPS
agg --fps 8 --speed 1.2 --theme monokai demo.cast demo.gif

# Or use gifsicle to compress
brew install gifsicle
gifsicle -O3 --colors 128 demo.gif -o demo_optimized.gif
```

## Step 6: Add to README

Add this line to README.md after the title:

```markdown
![Rust Pattern Viz Demo](demo.gif)
```

---

💡 Tips:
  - Keep the demo under 15 seconds
  - Use clear, simple examples
  - Pause briefly between steps
  - Test the GIF loops smoothly

EOF
