# Demo Assets

This directory contains assets for the README demo GIF.

## Files

- `demo-before.rs` - Complex nested pattern matching example (real-world HTTP handler)
- `demo-after.rs` - Refactored version with flat error handling
- `demo-script.sh` - Automated script to generate demo.gif
- `demo.gif` - The actual animated demo (generated, not committed to keep repo size down)

## Generating the Demo

```bash
# Prerequisites
cargo install agg  # asciinema GIF generator
brew install asciinema  # or apt-get install asciinema

# Generate demo.gif
./demo-script.sh
```

The script will:
1. Display the complex "before" code
2. Run `rpv analyze` showing high complexity (score: 12)
3. Display the refactored "after" code
4. Run `rpv analyze` showing improved complexity (score: 4)
5. Generate an animated GIF showing the entire flow

## Manual Demo Alternative

If automated generation fails, create the demo manually:

1. Open terminal with good contrast theme (Monokai recommended)
2. Set font size to 16pt for readability
3. Record with QuickTime/OBS:
   - Show `cat demo-before.rs`
   - Run `rpv analyze demo-before.rs` (paste pre-written output if CLI not ready)
   - Show `cat demo-after.rs`
   - Run `rpv analyze demo-after.rs`
   - End with "67% complexity reduction!" message
4. Convert to GIF with ffmpeg or online tool
5. Optimize GIF size (target: <2MB for fast GitHub loading)

## Design Principles

The demo follows these principles for maximum impact on r/rust:

- **Real-world code** - Not toy examples, actual HTTP error handling
- **Clear before/after** - Visual contrast between complex and simple
- **Quantified improvement** - "67% complexity reduction" is concrete
- **Quick comprehension** - <10 seconds to understand value
- **Professional quality** - Clean terminal, readable fonts, smooth transitions

## Updating the Demo

When updating:
1. Keep examples realistic (no `foo`/`bar`)
2. Maintain complexity scores accuracy
3. Test GIF loads quickly on slow connections
4. Verify text is readable at GitHub's default size
5. Ensure GIF loops smoothly
