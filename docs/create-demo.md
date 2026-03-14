# Creating the Demo GIF

This guide explains how to create the animated demo GIF for the README once the codebase is implemented.

## Prerequisites

Install required tools:

```bash
# macOS
brew install asciinema agg gifsicle imagemagick

# Linux (Ubuntu/Debian)
sudo apt-get install asciinema gifsicle imagemagick
cargo install --git https://github.com/asciinema/agg

# Verify installations
asciinema --version
agg --version
gifsicle --version
convert --version
```

## Recording Process

### Step 1: Record Terminal Session

```bash
# Start recording
asciinema rec demo-recording.cast

# In the recording session, run the demo script:
bash docs/demo-script.sh

# Exit when done (Ctrl+D or 'exit')
```

### Step 2: Convert to GIF

```bash
# Convert asciinema recording to GIF
agg \
  --fps 12 \
  --speed 1.0 \
  --font-size 16 \
  --theme monokai \
  demo-recording.cast \
  demo-raw.gif
```

### Step 3: Optimize GIF Size

```bash
# Resize if needed (to 800px width, maintaining aspect ratio)
convert demo-raw.gif -resize 800x600 demo-resized.gif

# Optimize with gifsicle (lossy compression to stay under 2MB)
gifsicle \
  --optimize=3 \
  --lossy=80 \
  --colors 256 \
  --resize-width 800 \
  demo-resized.gif \
  -o docs/demo.gif

# Check file size
ls -lh docs/demo.gif
```

### Step 4: If Still Over 2MB

If the GIF is still too large, try these additional optimizations:

```bash
# More aggressive lossy compression
gifsicle \
  --optimize=3 \
  --lossy=100 \
  --colors 128 \
  --resize-width 700 \
  demo-resized.gif \
  -o docs/demo.gif

# Or reduce frame rate
agg \
  --fps 8 \
  --speed 1.2 \
  --font-size 14 \
  demo-recording.cast \
  demo-raw.gif
# Then repeat optimization steps
```

## What the Demo Should Show

The ideal demo GIF sequence:

1. **Before (5-8 seconds)**
   - Show `docs/demo-before.rs` with complex match expression
   - Highlight the nested patterns and guards
   - Show how it's hard to follow the logic flow

2. **Analysis (3-5 seconds)**
   - Run: `rpv analyze docs/demo-before.rs`
   - Show the tool detecting patterns
   - Display confidence scores appearing

3. **After (7-10 seconds)**
   - Show the generated visual diagram
   - Pan/zoom to show different branches
   - Highlight the clear decision tree structure

**Total duration**: 15-25 seconds
**Target file size**: < 2MB (preferably < 1.5MB)

## Manual Alternative (If Tools Not Available)

If you can't install the recording tools, you can create a series of screenshots and combine them:

```bash
# Take screenshots at each stage
# screenshot1.png - Complex code
# screenshot2.png - Tool running
# screenshot3.png - Visual output

# Combine into animated GIF
convert \
  -delay 300 screenshot1.png \
  -delay 200 screenshot2.png \
  -delay 400 screenshot3.png \
  -loop 0 \
  docs/demo.gif

# Optimize
gifsicle --optimize=3 --lossy=80 docs/demo.gif -o docs/demo.gif
```

## Quality Checklist

Before committing the demo.gif:

- [ ] File size is under 2MB
- [ ] Resolution is at least 800px wide
- [ ] Text is readable (font size 14+ pt)
- [ ] Shows all three stages (before, analysis, after)
- [ ] Duration is 15-30 seconds
- [ ] Loops smoothly
- [ ] Colors/theme match project aesthetic
- [ ] No sensitive information visible

## Testing the GIF in README

```bash
# Preview locally
open README.md  # macOS
xdg-open README.md  # Linux

# Or use a Markdown preview tool
grip README.md
# Visit http://localhost:6419
```

## Troubleshooting

**GIF too large?**
- Reduce colors: `--colors 128` or even `--colors 64`
- Lower frame rate: `--fps 8`
- Reduce dimensions: `--resize-width 700`
- More lossy compression: `--lossy=100`

**GIF too blurry?**
- Reduce lossy setting: `--lossy=60`
- Increase colors: `--colors 256`
- Record at higher resolution then downscale

**Text not readable?**
- Increase font size in agg: `--font-size 18`
- Use higher contrast theme: `--theme solarized-dark`
- Record at higher resolution (1024x768)

## References

- [asciinema documentation](https://asciinema.org/)
- [agg (asciinema GIF generator)](https://github.com/asciinema/agg)
- [gifsicle manual](https://www.lcdf.org/gifsicle/man.html)
- [ImageMagick convert](https://imagemagick.org/script/convert.php)
