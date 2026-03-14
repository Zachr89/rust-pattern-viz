# Demo Recording Guide for Rust Pattern Viz

This guide provides detailed instructions for creating the animated GIF demo that appears at the top of the README.

## Overview

The demo should showcase the core value proposition: **real-time pattern matching visualization in VS Code**. The 10-second recording will show a developer hovering over a nested enum match statement and seeing instant analysis.

## Prerequisites

### Software Required

- **VS Code** with Rust Pattern Viz extension installed
- **Screen recording tool**:
  - macOS: Built-in (Cmd+Shift+5) or QuickTime Player
  - Linux: `peek`, `SimpleScreenRecorder`, or `recordmydesktop`
  - Windows: Xbox Game Bar (Win+G) or ShareX
- **ffmpeg** for video-to-GIF conversion
- **gifsicle** for GIF optimization (optional but recommended)

### Installation

```bash
# macOS
brew install ffmpeg gifsicle

# Linux (Ubuntu/Debian)
sudo apt install ffmpeg gifsicle

# Linux (Fedora)
sudo dnf install ffmpeg gifsicle

# Windows (with Chocolatey)
choco install ffmpeg gifsicle
```

## Step 1: Prepare Example Code

Create a file `demo_example.rs` with visually interesting nested pattern matching:

```rust
// demo_example.rs
use std::fs::File;
use std::io::Read;

enum ApiResponse {
    Success { data: Vec<u8>, status: u16 },
    Error { code: u16, message: String },
    Redirect { location: String },
    Pending,
}

enum ParseResult {
    Valid(String),
    Invalid(String),
}

fn process_response(response: ApiResponse) -> Result<String, String> {
    match response {
        ApiResponse::Success { data, status } => {
            if status == 200 {
                match String::from_utf8(data) {
                    Ok(text) => Ok(text),
                    Err(e) => Err(format!("UTF-8 error: {}", e)),
                }
            } else {
                Err(format!("Unexpected status: {}", status))
            }
        }
        ApiResponse::Error { code, message } => {
            Err(format!("API error {}: {}", code, message))
        }
        ApiResponse::Redirect { location } => {
            Err(format!("Redirect to: {}", location))
        }
        ApiResponse::Pending => {
            Err("Still pending".to_string())
        }
    }
}

fn parse_config(content: &str) -> ParseResult {
    if let Some(stripped) = content.strip_prefix("config:") {
        ParseResult::Valid(stripped.to_string())
    } else {
        ParseResult::Invalid("Invalid format".to_string())
    }
}
```

**Why this example:**
- Multiple levels of nesting (outer match + inner match)
- Different pattern types (enums, destructuring, if let)
- Realistic use case (API response handling)
- Shows both successful and error paths
- Demonstrates confidence scoring on various patterns

## Step 2: Configure VS Code

1. **Open clean workspace:**
   ```bash
   code --new-window demo_example.rs
   ```

2. **Adjust settings for recording:**
   - Set font size to 16-18pt for readability
   - Use a high-contrast theme (Dark+ or Light+)
   - Hide unnecessary UI elements:
     - `View > Appearance > Hide Status Bar`
     - `View > Appearance > Hide Activity Bar` (optional)
   - Set zoom level: `Ctrl/Cmd +` until code is clearly visible

3. **Position code strategically:**
   - Place the outer `match response` statement near the center of the screen
   - Ensure the full match block is visible (may need to collapse other functions)

## Step 3: Recording Sequence

### Timeline (10 seconds total)

| Time | Action | Visual |
|------|--------|--------|
| 0-2s | Static view | Full file visible, cursor off-screen |
| 2s | Cursor enters | Mouse moves from left side |
| 3s | Hover on `match` | Cursor stops on the `match` keyword |
| 3-4s | Extension activates | Hover tooltip starts appearing (may have slight delay) |
| 4-8s | Show analysis | Tooltip fully visible with pattern breakdown and SVG diagram |
| 8-9s | Scroll diagram | Gently scroll within hover tooltip to show flow diagram details |
| 9-10s | Cursor exits | Mouse moves away, tooltip fades |

### Recording Tips

1. **Stabilize cursor:**
   - Practice the hover motion 2-3 times before recording
   - Move slowly and deliberately (fast movements look unprofessional)
   - Hold cursor steady for 4+ seconds on the keyword

2. **Timing the recording:**
   - Start recording 1 second before cursor movement (gives clean intro)
   - Stop recording 1 second after cursor exits (clean outro)
   - Total recording: ~12 seconds (trim to 10 in editing)

3. **Audio:**
   - Not needed (this is a silent GIF)
   - Mute system audio during recording

4. **Screen area:**
   - Record just the VS Code window (not full desktop)
   - Ensure no distracting background applications
   - Minimum resolution: 1280x720 (will be scaled to 800px wide)

### Example Recording Commands

**macOS (QuickTime Player):**
```bash
# Open QuickTime Player
# File > New Screen Recording
# Click "Options" > Show Mouse Clicks
# Select VS Code window
# Click record button
# Wait 12 seconds
# Stop recording
# Save as demo_recording.mov
```

**Linux (peek):**
```bash
# Install peek
sudo apt install peek

# Run peek
peek

# Select area covering VS Code window
# Click "Record"
# Perform demo actions
# Click "Stop"
# Save as demo_recording.gif (peek outputs GIF directly!)
```

**Windows (Xbox Game Bar):**
```powershell
# Press Win+G to open Game Bar
# Click "Capture" widget
# Click "Record" button (or Win+Alt+R)
# Perform demo actions for 12 seconds
# Press Win+Alt+R to stop
# Recording saved to Videos/Captures folder
```

## Step 4: Convert to Optimized GIF

### If you recorded a video file (MOV, MP4):

```bash
# Navigate to recording location
cd ~/Downloads  # or wherever your recording is

# Convert to GIF with optimized palette
ffmpeg -i demo_recording.mov \
  -vf "fps=15,scale=800:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" \
  -loop 0 \
  demo.gif

# Optimize file size (target: <2MB)
gifsicle -O3 --colors 128 --lossy=80 demo.gif -o demo_optimized.gif

# Check file size
ls -lh demo_optimized.gif
```

### Parameter Explanation:

- `fps=15` - 15 frames per second (smooth but not bloated)
- `scale=800:-1` - 800px wide, height auto-calculated
- `palettegen` - Creates optimized color palette from video
- `paletteuse` - Applies palette for best quality
- `-loop 0` - Infinite loop
- `--colors 128` - Reduces color palette (vs 256 default)
- `--lossy=80` - Allows 20% quality loss for smaller file size

### If file is still >2MB:

```bash
# Further reduce frame rate
ffmpeg -i demo_recording.mov \
  -vf "fps=10,scale=800:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" \
  -loop 0 \
  demo.gif

# Or reduce resolution
ffmpeg -i demo_recording.mov \
  -vf "fps=15,scale=600:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" \
  -loop 0 \
  demo.gif
```

## Step 5: Quality Check

Before committing, verify:

- [ ] File size <2MB (ideally <1.5MB for fast loading)
- [ ] Resolution 800px wide (or 600px if size-constrained)
- [ ] Animation loops smoothly
- [ ] Text in VS Code is readable
- [ ] Hover tooltip is fully visible
- [ ] SVG diagram details are clear
- [ ] No distracting cursor jitter
- [ ] No visible UI glitches or artifacts

### Preview the GIF:

```bash
# macOS
open demo.gif

# Linux
xdg-open demo.gif

# Windows
start demo.gif
```

Watch the loop 3-4 times. Ask yourself:
- Does this clearly demonstrate the tool's value?
- Would a first-time visitor understand what they're seeing?
- Is the quality good enough for a professional README?

## Step 6: Deploy

```bash
# Move to repository root
mv demo_optimized.gif /path/to/rust-pattern-viz/demo.gif

# Verify README references it correctly
cat README.md | head -5
# Should show: <img src="demo.gif" ...>

# Commit
cd /path/to/rust-pattern-viz
git add demo.gif README.md
git commit -m "Add animated demo GIF showing pattern matching visualization"
git push origin main

# Verify on GitHub
# Visit https://github.com/yourusername/rust-pattern-viz
# Check that GIF loads and loops properly
```

## Troubleshooting

### GIF doesn't loop on GitHub
- Ensure `-loop 0` was used in ffmpeg command
- Try re-uploading (sometimes GitHub caching causes issues)

### File size too large (>2MB)
- Reduce FPS to 10: `fps=10`
- Reduce resolution: `scale=600:-1`
- Increase lossy compression: `--lossy=90`
- Trim recording to exactly 10 seconds

### Text is blurry
- Use higher resolution source video (1920x1080+)
- Increase VS Code font size before recording
- Use `flags=lanczos` in scale filter (already included above)

### Cursor is too fast/jerky
- Re-record with slower, more deliberate movements
- Use a mouse instead of trackpad for smoother motion
- Enable "Show Mouse Clicks" in recording software

### Extension tooltip doesn't appear
- Check that extension is installed and activated
- Hover slightly longer (3+ seconds)
- Restart VS Code if extension is unresponsive
- Check LSP server logs: `View > Output > Rust Pattern Viz`

### Colors look washed out
- Use `palettegen` + `paletteuse` (shown in commands above)
- Avoid `--colors` values below 128
- Record with VS Code in high-contrast theme

## Alternative: Screenshot Slideshow

If screen recording proves difficult, create a GIF from static screenshots:

```bash
# Take 5 screenshots showing progression:
# 1. code_visible.png - Full code view
# 2. cursor_approaching.png - Cursor near match keyword  
# 3. tooltip_appearing.png - Tooltip starts to show
# 4. tooltip_full.png - Full analysis visible
# 5. diagram_detail.png - Close-up of SVG diagram

# Create GIF from screenshots
convert -delay 200 -loop 0 \
  code_visible.png \
  cursor_approaching.png \
  tooltip_appearing.png \
  -delay 300 tooltip_full.png \
  -delay 200 diagram_detail.png \
  demo.gif

# Optimize
gifsicle -O3 --colors 128 demo.gif -o demo.gif
```

**Note:** This is less impressive than real recording but acceptable as a fallback.

## Best Practices Summary

✅ **DO:**
- Use realistic, nested pattern matching code
- Record at high resolution, then downscale
- Keep cursor movements slow and deliberate
- Optimize file size aggressively (<2MB target)
- Test GIF on GitHub before announcing

❌ **DON'T:**
- Record full desktop (distracting, large file)
- Use unrealistic toy examples
- Rush through the demo (hover <3 seconds)
- Skip optimization (multi-MB GIFs load slowly)
- Forget to test on actual GitHub repository page

## Questions?

If you encounter issues not covered here, check:
- [ffmpeg documentation](https://ffmpeg.org/ffmpeg.html)
- [gifsicle manual](https://www.lcdf.org/gifsicle/man.html)
- VS Code extension development logs

Good luck! A great demo GIF can 10x your GitHub traffic. 🚀
