# wr

**wr - Because every keystroke counts 🦀**

A fast, cross-platform alternative to the Unix `watch` command, written in Rust.

## Overview

**wr** (short for **watchr**) repeatedly executes a command at regular intervals and displays the output in real-time. Unlike the traditional Unix `watch` utility, `wr` works seamlessly across Linux, macOS, and Windows.

## Installation

### From Source

git clone <https://github.com/yourusername/wr.git>
cd wr
cargo build --release

text

### Usage

wr [OPTIONS] -- <COMMAND>...

text

**Note:** The `--` separator is required before the command.

### Examples

Monitor disk usage every 2 seconds (default)

wr -- df -h
Check processes every 5 seconds

wr -n 5 -- ps aux
Monitor memory usage

wr -- free -h
Watch directory contents

wr -- ls -la /tmp

text

### Options

- `-n, --interval <SECONDS>` - Set update interval (default: 2)
- `-h, --help` - Show help
- `-V, --version` - Show version

## Why wr?

- **Ultra-short**: Just 2 characters vs 5 for `watch`
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Memory safe**: Built with Rust
- **Familiar**: Drop-in replacement for Unix `watch`

## Current Limitations

- Requires `--` separator before commands
- Long-running commands (like `ping`, `tail -f`) not supported in this MVP
- Use `Ctrl+C` to exit

## Dependencies

[dependencies]
clap = { version = "4.0", features = ["derive"] }
crossterm = "0.27"
ctrlc = "3.4"

text

## License

MIT License - see [LICENSE](LICENSE) for details.

---
*Made with 🦀 by the Rust community*