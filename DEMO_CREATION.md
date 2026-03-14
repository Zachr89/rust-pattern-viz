# Creating the Demo GIF

This guide explains how to create the animated demo GIF for the README that showcases rust-pattern-viz's core value proposition.

## Goal

Show the complete user journey in **under 5 seconds**:
1. User pastes Rust code into the web interface
2. Visualization appears instantly
3. User sees pattern analysis, decision trees, and confidence scores

## Requirements

- **Duration:** 4-5 seconds maximum
- **Format:** GIF (optimized for web, <2MB preferred)
- **Dimensions:** 1200x800px or 1440x900px (16:10 aspect ratio works well in READMEs)
- **Frame rate:** 15-20 fps (smooth but small file size)
- **Content:** Must show the "before" (empty input) and "after" (rich visualization) clearly

## Recommended Tools

### Option 1: ScreenToGif (Windows/Linux - Recommended)
- **Download:** https://www.screentogif.com/
- **Pros:** Free, easy trimming, built-in optimizer, title frames
- **File size:** Excellent (often <1MB with good quality)

### Option 2: LICEcap (macOS/Windows)
- **Download:** https://www.cockos.com/licecap/
- **Pros:** Extremely simple, real-time recording
- **Cons:** Limited editing (no trimming), larger files

### Option 3: Kap (macOS)
- **Download:** https://getkap.co/
- **Pros:** Modern UI, plugins for cropping/trimming
- **File size:** Good with optimization plugins

### Option 4: peek (Linux)
- **Download:** https://github.com/phw/peek
- **Pros:** Lightweight, native Wayland support
- **Package:** `sudo apt install peek` (Ubuntu/Debian)

## Step-by-Step Instructions

### 1. Prepare the Demo Environment

**Launch the web app:**
```bash
cd wasm/www
npm run start
# Opens http://localhost:8080
```

**Or use the deployed version** if available (replace in README.md).

**Prepare sample code** in a text editor (for quick copy-paste):
```rust
fn process_result(input: Result<String, String>) -> Option<usize> {
    if let Ok(data) = input {
        Some(data.len())
    } else {
        None
    }
}

fn iterate_numbers(nums: Vec<i32>) {
    for num in nums.iter() {
        println!("{}", num);
    }
}
```

This code demonstrates:
- `if let` control flow pattern
- `Result<T,E>` error handling
- Iterator pattern (`for` loop)
- Multiple decision nodes

### 2. Recording Workflow

**Frame 1 (0-1 sec):** Empty state
- Show the web interface with empty input textarea
- Cursor ready to paste

**Frame 2 (1-2 sec):** Paste code
- CTRL+V (or CMD+V) to paste the sample code
- Quick visual of code appearing in textarea

**Frame 3 (2-4 sec):** Visualization appears
- Show the SVG diagram rendering:
  - Pattern boxes (color-coded by confidence)
  - Decision nodes with control flow
  - Import analysis section
- Pan slightly to show full diagram (if needed)

**Frame 4 (4-5 sec):** Highlight key feature (optional)
- Hover over a pattern box to show details tooltip
- OR zoom in slightly on a high-confidence pattern

### 3. Recording Settings

**ScreenToGif Configuration:**
1. Open ScreenToGif → Recorder
2. Set recording area: 1200x800px (drag border to browser window)
3. Frame rate: 15 fps
4. Start recording
5. Perform the demo workflow (paste → see visualization)
6. Stop recording (usually 6-8 seconds raw)

**Editing:**
1. ScreenToGif → Editor opens automatically
2. Delete dead frames at start/end
3. Trim to exactly 4-5 seconds
4. Add optional title card (first frame): "Rust Pattern Viz - Instant Code Analysis"
5. File → Save As → GIF
6. Encoder: FFmpeg (best quality/size ratio)
7. Optimize: Remove duplicates, reduce colors to 128-256

**LICEcap Configuration:**
1. Drag window border to 1200x800px
2. FPS: 15
3. Click "Record" → perform demo → click "Stop"
4. Save as `demo.gif`
5. Use external tool (ezgif.com, gifsicle) to optimize if >2MB

### 4. Optimization (Critical for GitHub)

GitHub has size limits and slow-loading GIFs hurt engagement.

**Target:** <1.5MB for fast loading

**Tools:**

**Online (easiest):**
- Upload to https://ezgif.com/optimize
- Settings:
  - Optimization level: 100 (Lossy GIF)
  - Colors: 128
  - Lossy: 30-50
- Download optimized version

**Command-line (best quality):**
```bash
# Install gifsicle
brew install gifsicle  # macOS
sudo apt install gifsicle  # Linux

# Optimize
gifsicle -O3 --colors 128 --lossy=50 demo.gif -o demo-optimized.gif

# Resize if needed (if >2MB after optimization)
gifsicle --resize-width 1000 demo-optimized.gif -o demo-final.gif
```

### 5. Placement in README

Replace the placeholder in README.md:

```markdown
![Demo](demo.gif)
```

**Commit the GIF:**
```bash
git add demo.gif
git commit -m "Add animated demo GIF showing core functionality"
git push
```

GitHub will render it automatically. Verify by viewing the README on GitHub.

## Quality Checklist

Before finalizing, verify:

- [ ] Duration is 4-5 seconds (not longer)
- [ ] File size is <2MB (preferably <1.5MB)
- [ ] Text in visualization is readable (not blurry)
- [ ] Colors match the actual app (green for high confidence, etc.)
- [ ] The "value prop" is obvious (paste code → instant insights)
- [ ] No distracting UI elements (browser tabs, OS notifications)
- [ ] Smooth playback (no janky frame drops)

## Alternative: Static Screenshot with Annotations

If creating a GIF is difficult, use a high-quality screenshot instead:

1. Take screenshot of visualization output
2. Add annotations in Figma/Photoshop:
   - Arrow pointing to paste area: "1. Paste Rust code"
   - Arrow pointing to pattern boxes: "2. See patterns instantly"
   - Arrow pointing to confidence score: "3. Get confidence ratings"
3. Save as PNG (optimized)
4. Replace `![Demo](demo.gif)` with `![Demo](demo.png)`

**Note:** GIF is strongly preferred for the Growth agent's distribution strategy (r/rust, HN, TWiR) as it shows the "instant" aspect better.

## Tips for Maximum Impact

1. **Use realistic code** - Don't use toy examples; show real-world patterns (error handling, iterators)
2. **Highlight confidence scores** - Make sure color-coding is visible (green/yellow/orange boxes)
3. **Show multiple patterns** - Demonstrates comprehensive analysis (not just one pattern type)
4. **Keep UI clean** - Hide browser chrome, close unnecessary tabs
5. **Smooth pacing** - Don't rush; let each frame be clear for 0.5-1 second
6. **Test on mobile** - GIF should be readable on small screens (avoid tiny text)

## Troubleshooting

**GIF is too large (>2MB):**
- Reduce dimensions to 1000x667px or 800x600px
- Lower color count to 64-128 colors
- Increase lossy compression (up to 80 in gifsicle)
- Reduce frame rate to 12 fps
- Trim duration to 3-4 seconds

**Text is blurry:**
- Record at higher resolution (1920x1200), then scale down
- Use lossless recording initially, optimize only at the end
- Increase color count to 256 (trade-off with file size)

**Recording is janky:**
- Close other applications to free CPU
- Use lower frame rate (12 fps instead of 15)
- Record in smaller window, upscale later (not recommended for text)

**Can't capture smooth paste action:**
- Pre-type partial code, then finish typing instead of pasting
- Use keyboard shortcuts (more natural than mouse)
- Record multiple takes; ScreenToGif lets you delete bad frames

## Ready to Ship?

Once you have `demo.gif`:

1. Place in repo root: `rust-pattern-viz/demo.gif`
2. Verify README renders correctly on GitHub
3. Test file size: `ls -lh demo.gif` (should be <1.5MB)
4. Post to r/rust with GIF in description (Growth agent strategy)
5. Use GIF in HN "Show HN" post as first visual

The demo GIF is your most important marketing asset for this launch window. Invest time to make it compelling.
