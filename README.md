# wr

A fast, cross-platform alternative to the Unix `watch` command, written in Rust.

## Overview

**wr - Because every keystroke counts 🦀**

**wr** (short for **watchr**) repeatedly executes a specified command at regular intervals and displays the output in real-time. Unlike the traditional Unix `watch` utility, `wr` works seamlessly across Linux, macOS, and Windows, providing a consistent monitoring experience everywhere.

## Features

- ⚡ **Fast and lightweight** - Minimal overhead with just two dependencies
- 🔄 **Periodic command execution** - Run any command at customizable intervals  
- 🖥️ **Cross-platform** - Works on Linux, macOS, and Windows
- 📺 **Clean output** - Automatically clears screen between runs
- ⌨️ **Ultra-short command** - Just 2 characters to type vs 5 for `watch`
- 🦀 **Memory safe** - Built with Rust for reliability and performance

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/yourusername/wr/releases).

### From Crates.io

```bash
cargo install wr
```

### From Source

```bash
git clone https://github.com/yourusername/wr.git
cd wr
cargo build --release
```

The binary will be available at `target/release/wr`.

## Usage

Basic syntax:

```bash
wr [OPTIONS] <COMMAND>...
```

### Examples

**Monitor disk usage every 2 seconds (default):**

```bash
wr df -h
```

**Check processes every 5 seconds:**

```bash
wr -n 5 ps aux
```

**Monitor network connections:**

```bash
wr netstat -an
```

**Watch log file changes:**

```bash
wr tail /var/log/myapp.log
```

**Monitor system memory:**

```bash
wr free -h
```

### Options

- `-n, --interval <SECONDS>` - Set the update interval in seconds (default: 2)
- `-h, --help` - Print help information
- `-V, --version` - Print version information

## Why wr?

**Extremely short**: At just 2 characters, `wr` is 60% shorter than `watch` - perfect for commands you run dozens of times per day.

**Cross-platform**: Windows users finally get a native `watch` equivalent without needing WSL or complex setups.

**Modern reliability**: Built with Rust's memory safety guarantees and excellent cross-platform support.

**Familiar**: If you know Unix `watch`, you already know `wr`.

**Smart naming**: Short for `watchr` - a watcher written in Rust.

## Building

Requirements:

- Rust 1.70 or later

### Clone the repository

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/wr.git
   ```

2. Navigate to the directory:

   ```bash
   cd wr
   ```

3. Build for development:

   ```bash
   cargo build
   ```

4. Build optimized release:

   ```bash
   cargo build --release
   ```

5. Run tests:

   ```bash
   cargo test
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/wr.git`
3. Create a feature branch: `git checkout -b feature-name`
4. Make your changes and add tests
5. Run tests: `cargo test`
6. Submit a pull request

## Roadmap

Future enhancements being considered:

- [ ] **Diff highlighting** - Highlight changes between successive runs
- [ ] **Output logging** - Save command output to files
- [ ] **JSON output** - Structured output format for automation
- [ ] **Signal handling** - Graceful shutdown with Ctrl+C
- [ ] **Color themes** - Customizable color schemes
- [ ] **Multiple commands** - Watch several commands simultaneously

## Performance

`wr` is designed to be lightweight and efficient:

- **Minimal dependencies**: Only `clap` for argument parsing and `crossterm` for terminal control
- **Low memory footprint**: No heavy runtime or garbage collection
- **Fast startup**: Rust's zero-cost abstractions mean no performance penalty for safety

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original Unix `watch` utility
- Built with the excellent Rust ecosystem tools: [`clap`](https://github.com/clap-rs/clap) and [`crossterm`](https://github.com/crossterm-rs/crossterm)
- Thanks to the Rust community for creating such powerful, lightweight crates
