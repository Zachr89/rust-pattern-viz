# Demo Recording Script Guide

This directory contains materials for creating the animated demo GIF shown in the main README.

## Files

- **demo-script.sh** - Automated script to record terminal session showing tool usage
- **demo-before.rs** - Example Rust code with complex pattern matching (before analysis)
- **demo-after.rs** - Same code with comments showing detected patterns (after analysis)
- **create-demo.md** - Detailed instructions for generating the demo.gif
- **demo.gif** - The final animated demo (created by running demo-script.sh)

## Quick Start

Once the core codebase is implemented:

```bash
# 1. Install recording tools (one-time setup)
brew install asciinema agg gifsicle  # macOS
# or
sudo apt-get install asciinema gifsicle  # Linux
cargo install --git https://github.com/asciinema/agg

# 2. Run the demo script
bash docs/demo-script.sh

# 3. The script will:
#    - Record the terminal session
#    - Convert to GIF
#    - Optimize to < 2MB
#    - Save as docs/demo.gif

# 4. Verify the output
ls -lh docs/demo.gif
open docs/demo.gif  # macOS
xdg-open docs/demo.gif  # Linux
```

## What Gets Recorded

The demo shows three stages:

1. **Before**: Display complex Rust code with nested match expressions
2. **Analysis**: Run `rpv analyze` showing pattern detection in progress
3. **After**: Display the visual decision tree diagram

Total duration: ~20 seconds, file size: <2MB

## Customization

Edit `demo-script.sh` to customize:

- **Timing**: Adjust `sleep` durations between commands
- **Colors**: Change terminal color scheme
- **Text**: Modify displayed code or analysis output
- **Speed**: Adjust typing speed simulation

## Manual Recording

If you prefer manual recording:

```bash
# Start recording
asciinema rec demo.cast

# Manually run commands from demo-script.sh
cat docs/demo-before.rs
rpv analyze docs/demo-before.rs
# ... etc

# Stop recording (Ctrl+D)

# Convert to GIF
agg --fps 12 demo.cast demo.gif
gifsicle --optimize=3 --lossy=80 demo.gif -o docs/demo.gif
```

## Troubleshooting

**Script fails with "command not found"?**
- Install missing tools (see create-demo.md)
- Ensure `rpv` binary is built: `cargo build --release`
- Add to PATH: `export PATH="$PWD/target/release:$PATH"`

**GIF too large?**
- Run optimization: `gifsicle --optimize=3 --lossy=100 --colors 128 docs/demo.gif -o docs/demo.gif`
- See create-demo.md for advanced optimization techniques

**Want different content?**
- Edit `demo-before.rs` with your own example
- Modify `demo-script.sh` commands
- Adjust timing/pacing

## References

- Main guide: [create-demo.md](create-demo.md)
- Recording tool: [asciinema](https://asciinema.org/)
- GIF generator: [agg](https://github.com/asciinema/agg)
- Optimizer: [gifsicle](https://www.lcdf.org/gifsicle/)
