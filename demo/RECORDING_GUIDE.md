# Demo GIF Recording Guide

This guide walks you through creating the perfect demo GIF for the rust-pattern-viz README.

## Goal
Create a <2MB animated GIF showing:
1. Match expression in VSCode
2. Live highlighting when hovering
3. Enum destructuring visualization
4. Pattern analysis popup

## Prerequisites

### Install the Extension
```bash
# Build the extension
cd vscode-extension
npm install
npm run compile
code --install-extension rust-pattern-viz-*.vsix
```

### Install Recording Tools

**macOS:**
```bash
brew install ffmpeg gifsicle
```

**Ubuntu/Debian:**
```bash
sudo apt install ffmpeg gifsicle
```

**Windows:**
- Download ffmpeg from https://ffmpeg.org/download.html
- Download gifsicle from https://www.lcdf.org/gifsicle/

## Recording Steps

### 1. Prepare VSCode
- Open `demo/example.rs` in VSCode
- Set zoom to 150% (View → Appearance → Zoom In) for better visibility
- Use a light theme (Light+ recommended) for clarity
- Set window size to 1200x800px
- Close all sidebars (Cmd+B to toggle)
- Show only the editor pane

### 2. Recording Sequence (10 seconds)

**Seconds 0-2:** Show the code
- Display the `process_message` function with the match expression visible
- Keep cursor still for 1 second

**Seconds 3-5:** Trigger visualization  
- Slowly move cursor to hover over the `match` keyword
- Wait for the hover popup to appear
- Hold still for 2 seconds to show the analysis

**Seconds 6-8:** Show details
- Scroll slightly in the hover popup (if needed) to show full visualization
- Keep steady

**Seconds 9-10:** Context
- Move cursor away to dismiss hover
- Show the full match expression one more time

### 3. Screen Recording

**macOS (Built-in):**
1. Press Cmd+Shift+5
2. Select "Record Selected Portion"
3. Frame the VSCode window (1200x800 area)
4. Click Record
5. Perform the sequence above
6. Click Stop in menu bar
7. Save as `demo/recording.mp4`

**Windows (Xbox Game Bar):**
1. Press Win+G to open Game Bar
2. Click Record button
3. Perform the sequence
4. Press Win+Alt+R to stop
5. Find in Videos/Captures folder
6. Move to `demo/recording.mp4`

**Linux (SimpleScreenRecorder):**
```bash
sudo apt install simplescreenrecorder
# Use GUI to record specific window
```

### 4. Generate GIF

```bash
cd demo
chmod +x create_demo_gif.sh
./create_demo_gif.sh
```

The script will:
- Extract frames at 10fps (smooth but compact)
- Scale to 600px width (readable on GitHub)
- Optimize color palette
- Compress with gifsicle
- Target <2MB file size

### 5. Verify Quality

Check the generated `pattern-viz-demo.gif`:
- ✅ File size under 2MB
- ✅ Text is readable
- ✅ Hover popup is visible
- ✅ Animation is smooth
- ✅ Colors look good

If size is too large:
- Trim the recording to 8 seconds
- Reduce scale to 500px: edit script, change `scale=600` to `scale=500`
- Increase compression: change `--lossy=80` to `--lossy=90`

## Alternative: Manual GIF Creation

If the script doesn't work, use online tools:

### Option A: CloudConvert
1. Upload `recording.mp4` to https://cloudconvert.com/mp4-to-gif
2. Settings:
   - Frame rate: 10 fps
   - Width: 600px
   - Quality: High
3. Download and rename to `pattern-viz-demo.gif`

### Option B: EZGIF
1. Go to https://ezgif.com/video-to-gif
2. Upload recording.mp4
3. Set:
   - Size: 600px width
   - Frame rate: 10
   - Method: Gifsicle
4. Click "Convert to GIF"
5. Optimize: Click "Optimize" tab → Compression level 35
6. Download

## Tips for Best Results

### Timing
- **Too fast:** Users can't read the visualization
- **Too slow:** File size bloats
- **Sweet spot:** 10 seconds total, 2-3 seconds showing hover

### Framing
- Show only the editor (no sidebars, terminal)
- Center the match expression in frame
- Leave space for hover popup to appear

### Performance
- Close other apps while recording
- Disable notifications
- Use a solid color desktop background

### Color Scheme
- Light themes work better than dark for GIFs
- Higher contrast = better compression
- Avoid animated backgrounds/wallpapers

## Troubleshooting

**"ffmpeg: command not found"**
- Install ffmpeg using package manager (see Prerequisites)

**GIF is too large (>2MB)**
- Shorten recording to 8 seconds
- Reduce scale: `scale=500:-1`
- Increase lossy compression: `--lossy=90`

**GIF is blurry**
- Increase recording resolution before converting
- Use `scale=700:-1` for larger output
- Reduce compression: `--lossy=60`

**Colors look washed out**
- Use a light theme during recording
- Try `palettegen=stats_mode=full` instead of `diff`
- Reduce lossy compression

**Hover popup doesn't appear**
- Ensure extension is installed and activated
- Check that `example.rs` is saved
- Hover directly over the `match` keyword
- Wait 500ms for popup to trigger

## Final Checklist

Before committing:
- [ ] GIF shows match expression clearly
- [ ] Hover visualization is visible
- [ ] Text in popup is readable
- [ ] File size under 2MB
- [ ] Smooth animation (no jarring jumps)
- [ ] Light theme used
- [ ] Only editor visible (no clutter)
- [ ] 10 seconds or less duration

Once satisfied:
```bash
git add demo/pattern-viz-demo.gif
git commit -m "Add demo GIF showing pattern matching visualization"
```
