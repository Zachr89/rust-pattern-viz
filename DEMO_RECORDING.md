# Demo GIF Recording Guide

This guide explains how to record the demo GIF for the rust-pattern-viz README.

## Goal

Create a **15-second animated GIF** showing the core value proposition:
1. Developer types a match expression
2. Visualizer highlights branches in real-time
3. Catches an exhaustiveness bug

## Requirements

### Software Needed

- **Screen Recorder**: 
  - Mac: [Kap](https://getkap.co/) (free, easy GIF export)
  - Windows: [ScreenToGif](https://www.screentogif.com/) (free)
  - Linux: [Peek](https://github.com/phw/peek) (free)
  
- **VS Code** with rust-pattern-viz extension installed
- **Rust project** (can use this repo as the example)

### Screen Setup

- **Resolution**: 1920x1080 or higher
- **Recording Area**: Focus on VS Code editor only (no desktop chrome)
- **Font Size**: Increase to 16-18pt for readability
- **Theme**: Use a high-contrast theme (e.g., Dark+ or One Dark Pro)
- **Zoom**: 125-150% if recording at 1920x1080

## Recording Script

### Scene 1: Typing a Match Expression (0:00 - 0:05)

**Setup:**
```rust
// Start with this incomplete code visible in editor:
enum Color {
    Red,
    Green,
    Blue,
    Yellow,  // This will be the "missing" arm
}

fn describe_color(color: Color) -> &'static str {
    match color {
        // Cursor starts here
    }
}
```

**Action:**
Type slowly and deliberately:
```rust
        Color::Red => "red",
        Color::Green => "green",
        Color::Blue => "blue",
```

**Timing:** 5 seconds

**Tips:**
- Type at human speed (not too fast, not too slow)
- Pause briefly after each line for visual clarity
- Show cursor movement

### Scene 2: Visualizer Highlighting (0:05 - 0:10)

**Action:**
1. Hover over the `describe_color` function signature
2. The hover tooltip should appear showing:
   - Pattern type: "Match Expression"
   - Detected branches: Red, Green, Blue
   - Confidence: ~0.75 (incomplete pattern)
   - **Warning**: "Exhaustiveness: Missing arm for Color::Yellow"

**Timing:** 5 seconds (hold hover for full 5s)

**Tips:**
- Position mouse precisely over function name
- Let tooltip fully animate in
- Keep tooltip visible for 3-4 seconds
- Make sure warning text is clearly visible

### Scene 3: Catching the Bug (0:10 - 0:15)

**Action:**
1. Move cursor to end of match expression
2. Add the missing arm:
   ```rust
           Color::Yellow => "yellow",
   ```
3. Hover again to show updated analysis:
   - Confidence now ~0.95
   - ✅ "Exhaustiveness: Complete"

**Timing:** 5 seconds

**Tips:**
- Show the "before" and "after" clearly
- The confidence score change is important
- End with the complete, correct code visible

## Export Settings

### GIF Optimization

- **Frame Rate**: 15 FPS (sufficient for UI interactions)
- **Size**: Max 800px width (resize from source resolution)
- **Quality**: Medium-high (balance file size vs clarity)
- **Loop**: Infinite
- **Target File Size**: < 5MB (GitHub inline rendering)

### Using Kap (Mac)

```bash
# After recording:
1. Click "Export"
2. Choose "GIF" format
3. Set width to 800px
4. Set FPS to 15
5. Enable "Optimize for size"
6. Export
```

### Using ScreenToGif (Windows)

```
1. File > Save As > GIF
2. Set Width: 800px (maintain aspect ratio)
3. Encoder: FFmpeg
4. Quality: 90
5. Frame rate: 15 FPS
6. Save
```

### Using Peek (Linux)

```bash
# Peek auto-optimizes, just:
1. Set framerate to 15
2. Record
3. Save as GIF
4. Manually resize if needed:
   convert demo-large.gif -resize 800x demo.gif
```

## Post-Processing

### Optimize File Size

If GIF is > 5MB, use [gifsicle](https://github.com/kohler/gifsicle):

```bash
# Install
brew install gifsicle  # Mac
apt-get install gifsicle  # Linux

# Optimize
gifsicle -O3 --colors 256 demo.gif -o demo-optimized.gif

# If still too large, reduce colors:
gifsicle -O3 --colors 128 demo.gif -o demo-optimized.gif
```

### Add Captions (Optional)

Use [gifcaption](https://github.com/avinassh/gifcaption) or manually:

```bash
# Install ImageMagick
brew install imagemagick

# Add title frame
convert demo.gif -coalesce \
  -gravity North -pointsize 24 -fill white \
  -annotate +0+10 "rust-pattern-viz: Real-time Pattern Analysis" \
  -layers optimize demo-titled.gif
```

## File Placement

Place the final GIF in:
```
assets/demo.gif
```

Then update README.md to remove the placeholder:
```markdown
## Demo

Watch rust-pattern-viz catch a pattern matching bug in real-time:

![rust-pattern-viz demo](./assets/demo.gif)

*15 seconds: typing → analysis → bug detection*
```

## Quality Checklist

Before committing the GIF, verify:

- [ ] All text is readable at 800px width
- [ ] Hover tooltip is fully visible
- [ ] Timing feels natural (not rushed or slow)
- [ ] File size < 5MB
- [ ] Loops smoothly (no jarring cuts)
- [ ] Shows clear value proposition (catches real bug)
- [ ] Colors/contrast work on both light and dark backgrounds
- [ ] No sensitive information visible (file paths, etc.)

## Alternative: Video Approach

If GIF file size is problematic, consider:

1. **Record MP4 video** instead (better compression)
2. **Upload to YouTube** (unlisted)
3. **Link from README**:
   ```markdown
   [![Demo Video](./assets/demo-thumbnail.png)](https://youtu.be/YOUR_VIDEO_ID)
   ```

## Automation Script

For quick iteration, use this script:

```bash
#!/bin/bash
# record-demo.sh

echo "Starting demo recording in 3 seconds..."
sleep 3

# Use QuickTime Player on Mac
osascript <<EOF
tell application "QuickTime Player"
    activate
    new screen recording
end tell
EOF

echo "Recording started. Follow the script in DEMO_RECORDING.md"
echo "Press Ctrl+C when done, then:"
echo "1. Stop QuickTime recording"
echo "2. Save as demo-raw.mov"
echo "3. Run: ./convert-demo.sh"
```

```bash
#!/bin/bash
# convert-demo.sh

if [ ! -f "demo-raw.mov" ]; then
    echo "Error: demo-raw.mov not found"
    exit 1
fi

echo "Converting to GIF..."
ffmpeg -i demo-raw.mov -vf "fps=15,scale=800:-1:flags=lanczos" \
    -c:v gif demo.gif

echo "Optimizing..."
gifsicle -O3 --colors 256 demo.gif -o assets/demo.gif

echo "Done! File size:"
ls -lh assets/demo.gif

echo "Cleaning up..."
rm demo-raw.mov demo.gif
```

## Tips for Best Results

1. **Practice First**: Record 2-3 takes before the final version
2. **Clean Environment**: Close all other apps, hide menu bars
3. **Steady Hands**: Use mouse, not trackpad
4. **Clear Intent**: Every action should be deliberate and visible
5. **Test on Mobile**: View the GIF on a phone to ensure readability

## Need Help?

If you're recording the demo and get stuck:
1. Open an issue: "Help needed: Demo GIF recording"
2. Share your attempt (even if imperfect)
3. Tag @maintainer for feedback

The community will help iterate until we have a perfect demo!
