# Support

Need help with SimpleX TUI? Here's how to get support.

---

## Quick Links

| Resource | Description |
|----------|-------------|
| [README](README.md) | Project overview and quick start |
| [Documentation](DEVELOPMENT.md) | Development guide |
| [FAQ](#frequently-asked-questions) | Common questions |
| [Troubleshooting](#troubleshooting) | Common issues |

---

## Getting Help

### 1. Check Documentation First

- [README.md](README.md) - Installation and usage
- [DEVELOPMENT.md](DEVELOPMENT.md) - Development setup
- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute

### 2. Search Existing Issues

Check if your question has already been answered:
- [Open Issues](https://github.com/cannatoshi/simplex-tui/issues)
- [Closed Issues](https://github.com/cannatoshi/simplex-tui/issues?q=is%3Aissue+is%3Aclosed)

### 3. Ask in Discussions

For questions and help:
- [GitHub Discussions](https://github.com/cannatoshi/simplex-tui/discussions)

### 4. Report a Bug

Found a bug? Please report it:
- [Bug Report Template](https://github.com/cannatoshi/simplex-tui/issues/new?template=bug_report.md)

### 5. Security Issues

For security vulnerabilities:
- [Security Policy](SECURITY.md)
- [Private Vulnerability Report](https://github.com/cannatoshi/simplex-tui/security/advisories/new)

---

## Frequently Asked Questions

### General

**Q: What is SimpleX TUI?**

A: SimpleX TUI is a terminal user interface for SimpleX Chat. It provides a keyboard-driven interface to send and receive messages using the SimpleX protocol.

**Q: Is SimpleX TUI affiliated with SimpleX Chat Ltd?**

A: No. SimpleX TUI is an independent, community-driven project. See [TRADEMARK.md](TRADEMARK.md) for details.

**Q: What license is SimpleX TUI under?**

A: AGPL-3.0. See [LICENSE](LICENSE) for the full text.

### Installation

**Q: What are the system requirements?**

A: 
- Rust 1.92+ (for building from source)
- SimpleX Chat CLI installed and running
- A terminal emulator with UTF-8 and color support

**Q: How do I install SimpleX TUI?**

A:
```bash
git clone https://github.com/cannatoshi/simplex-tui.git
cd simplex-tui
cargo build --release
./target/release/simplex-tui
```

**Q: How do I update SimpleX TUI?**

A:
```bash
cd simplex-tui
git pull
cargo build --release
```

### Usage

**Q: How do I connect to SimpleX CLI?**

A: Start SimpleX CLI with WebSocket enabled, then start SimpleX TUI:
```bash
# Terminal 1
simplex-chat -p 5225

# Terminal 2
./simplex-tui
```

**Q: What keyboard shortcuts are available?**

A: See the [README.md](README.md#keyboard-shortcuts) for a full list.

**Q: Can I use the mouse?**

A: Yes! Click on contacts to select them, click buttons in modals, and more.

### Troubleshooting

**Q: SimpleX TUI can't connect to SimpleX CLI**

A: 
1. Make sure SimpleX CLI is running with `-p 5225`
2. Check if port 5225 is in use: `ss -tlnp | grep 5225`
3. Try restarting SimpleX CLI

**Q: The terminal looks broken after exiting**

A: Run `reset` or `stty sane` to restore your terminal.

**Q: Messages are not showing up**

A: 
1. Check WebSocket connection status in the status bar
2. Try refreshing with `r` key
3. Restart both SimpleX CLI and TUI

---

## Troubleshooting

### Connection Issues

#### WebSocket Connection Failed
```bash
# 1. Check if SimpleX CLI is running
ps aux | grep simplex-chat

# 2. Check if port is listening
ss -tlnp | grep 5225

# 3. Start SimpleX CLI
simplex-chat -p 5225

# 4. Restart SimpleX TUI
./simplex-tui
```

#### Connection Drops Frequently

- Check your system resources
- Ensure SimpleX CLI is stable
- Check for network issues

### Build Issues

#### Rust Version Too Old
```bash
# Update Rust
rustup update stable

# Verify version (need 1.92+)
rustc --version
```

#### Compilation Errors
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Update dependencies
cargo update
```

### Display Issues

#### Broken Characters

- Ensure your terminal supports UTF-8
- Try a different terminal emulator
- Check your locale settings: `locale`

#### Colors Not Working

- Ensure your terminal supports 256 colors
- Check TERM environment variable: `echo $TERM`
- Try: `export TERM=xterm-256color`

#### Terminal Broken After Exit
```bash
# Reset terminal
reset

# Or
stty sane

# Or close and reopen terminal
```

### Performance Issues

#### Slow Startup

- First build is slower (compiling dependencies)
- Use release build: `cargo build --release`

#### High CPU Usage

- This may indicate a busy loop
- Please report as a bug with details

---

## SimpleX CLI Issues

For issues with SimpleX Chat CLI itself (not SimpleX TUI):

- [SimpleX Chat Issues](https://github.com/simplex-chat/simplex-chat/issues)
- [SimpleX Documentation](https://simplex.chat/docs/)

---

## Contact

- **GitHub Issues:** [simplex-tui/issues](https://github.com/cannatoshi/simplex-tui/issues)
- **GitHub Discussions:** [simplex-tui/discussions](https://github.com/cannatoshi/simplex-tui/discussions)

---

*Last updated: January 2026*
