#!/bin/bash
# Script to create demo GIF for rust-pattern-viz README
# Target: <2MB file size for fast GitHub loading

set -e

echo "🎬 Rust Pattern Viz Demo GIF Creator"
echo "===================================="
echo ""
echo "This script helps create an optimized demo GIF showing:"
echo "  1. Match expression in code"
echo "  2. Live highlighting of executed arm"
echo "  3. Enum destructuring step-by-step"
echo ""

# Check dependencies
command -v ffmpeg >/dev/null 2>&1 || { echo "❌ ffmpeg required. Install: brew install ffmpeg / apt install ffmpeg"; exit 1; }
command -v gifsicle >/dev/null 2>&1 || { echo "⚠️  gifsicle recommended for optimization. Install: brew install gifsicle / apt install gifsicle"; }

DEMO_DIR="$(cd "$(dirname "$0")" && pwd)"
OUTPUT_GIF="$DEMO_DIR/pattern-viz-demo.gif"
TEMP_DIR="$DEMO_DIR/temp_frames"

echo "📝 Step 1: Record your screen"
echo "----------------------------"
echo "Record a ~10 second video showing:"
echo "  - Open VSCode with rust-pattern-viz extension installed"
echo "  - Show the demo example file (demo/example.rs)"
echo "  - Hover over a match expression to trigger visualization"
echo "  - Show the hover popup with pattern analysis"
echo ""
echo "Save the recording as: $DEMO_DIR/recording.mp4"
echo ""
read -p "Press Enter once you have recording.mp4 ready..."

if [ ! -f "$DEMO_DIR/recording.mp4" ]; then
    echo "❌ recording.mp4 not found in $DEMO_DIR"
    exit 1
fi

echo ""
echo "🎞️  Step 2: Converting to optimized GIF"
echo "--------------------------------------"

# Create temp directory
mkdir -p "$TEMP_DIR"

# Extract frames at 10fps for smaller file size
echo "Extracting frames at 10fps..."
ffmpeg -i "$DEMO_DIR/recording.mp4" -vf "fps=10,scale=600:-1:flags=lanczos" -y "$TEMP_DIR/frame_%04d.png" 2>&1 | grep -v "frame=" || true

# Convert to GIF with palette optimization
echo "Generating color palette..."
ffmpeg -i "$DEMO_DIR/recording.mp4" -vf "fps=10,scale=600:-1:flags=lanczos,palettegen=stats_mode=diff" -y "$TEMP_DIR/palette.png" 2>&1 | grep -v "frame=" || true

echo "Creating GIF with optimized palette..."
ffmpeg -i "$DEMO_DIR/recording.mp4" -i "$TEMP_DIR/palette.png" -lavfi "fps=10,scale=600:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle" -y "$OUTPUT_GIF" 2>&1 | grep -v "frame=" || true

# Further optimize with gifsicle if available
if command -v gifsicle >/dev/null 2>&1; then
    echo "Optimizing with gifsicle..."
    gifsicle -O3 --lossy=80 -o "${OUTPUT_GIF}.optimized" "$OUTPUT_GIF"
    mv "${OUTPUT_GIF}.optimized" "$OUTPUT_GIF"
fi

# Clean up
rm -rf "$TEMP_DIR"

# Check file size
SIZE=$(du -h "$OUTPUT_GIF" | cut -f1)
SIZE_BYTES=$(du -b "$OUTPUT_GIF" | cut -f1)
MAX_SIZE=$((2 * 1024 * 1024))  # 2MB

echo ""
echo "✅ GIF created: $OUTPUT_GIF"
echo "   File size: $SIZE"

if [ "$SIZE_BYTES" -gt "$MAX_SIZE" ]; then
    echo ""
    echo "⚠️  WARNING: File size exceeds 2MB target!"
    echo "   Consider:"
    echo "   - Shortening the recording (aim for 8-10 seconds)"
    echo "   - Reducing scale further: scale=500:-1 instead of 600"
    echo "   - Increasing lossy compression: --lossy=90"
    echo ""
    echo "   Re-run after adjustments."
else
    echo "   ✓ Under 2MB target - ready for README!"
fi

echo ""
echo "📋 Next steps:"
echo "   1. Review the GIF: open $OUTPUT_GIF"
echo "   2. If satisfied, commit: git add demo/pattern-viz-demo.gif"
echo "   3. The README.md already references this file"
echo ""
