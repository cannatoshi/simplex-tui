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
- [ROADMAP.md](ROADMAP.md) - Planned features and timeline

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

**Q: What platforms are supported?**

A: Linux, macOS, and Windows are supported. The TUI runs in any terminal with UTF-8 and 256-color support.

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

**Q: What is Panic Mode?**

A: Press `p` to instantly clear the screen for privacy. Useful if someone unexpectedly looks at your screen.

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

### Radio Transport (Planned v0.5.0+)

**Q: What is the radio transport feature?**

A: An optional feature that allows sending encrypted messages over LoRa radio instead of the internet. When Tor/internet is available, SimpleX TUI uses it. When offline, it can fall back to radio communication with contacts who also have radio hardware.

**Q: Do I need special hardware for radio features?**

A: Yes. Supported hardware includes:

| Hardware | Description | Status |
|----------|-------------|--------|
| uConsole AIO V2 | Integrated LoRa + RTL-SDR + GPS | Primary target |
| RNode | Reticulum-compatible LoRa device | Planned |
| USB LoRa Modules | SX1262/SX1276 based | Planned |

**Q: What is the range of LoRa communication?**

A: Range depends on environment, antenna, and power settings:

| Environment | Metric | Imperial |
|-------------|--------|----------|
| Urban (buildings) | 2-5 km | 1-3 miles |
| Suburban | 8-15 km | 5-10 miles |
| Rural / Open | 15-30 km | 10-20 miles |
| Line of sight (hilltop) | 50+ km | 30+ miles |

**Q: Is radio transmission legal?**

A: ISM band transmission (868 MHz EU / 915 MHz US) is legal in most countries within power and duty cycle limits. However:

- ⚠️ Regulations vary by country
- ⚠️ Some countries have additional restrictions
- ⚠️ You are solely responsible for compliance

See [DISCLAIMER.md](DISCLAIMER.md) for detailed regulatory information.

**Q: What is the difference between Reticulum mode and custom protocol mode?**

A: 

| Feature | Reticulum Mode | Custom Protocol Mode |
|---------|----------------|---------------------|
| Mesh interoperability | ✅ Yes | ❌ No |
| Works with Nomad Network | ✅ Yes | ❌ No |
| Lightweight | ⚠️ Moderate | ✅ Very |
| Closed group privacy | ⚠️ Moderate | ✅ High |
| Setup complexity | ⚠️ Higher | ✅ Lower |

**Reticulum Mode:** Use if you want to communicate with the wider Reticulum mesh network, including Nomad Network, Sideband, and MeshChat users.

**Custom Protocol Mode:** Use if you only need to communicate within a closed group and want maximum simplicity and privacy.

**Q: Can I use both Reticulum and custom protocol?**

A: Yes, but not simultaneously with the same contact. Each contact can be configured to use either Reticulum or the custom protocol.

**Q: Do radio messages have the same encryption as SimpleX?**

A: Yes. The radio transport uses the same cryptographic primitives:

- X25519 ECDH for key exchange
- ChaCha20-Poly1305 for encryption
- Ed25519 for signatures
- Ephemeral keys for forward secrecy

**Q: Can my radio transmissions be tracked?**

A: Yes. Unlike Tor, radio transmissions can be located using direction-finding equipment. If this is a concern:

- Keep transmissions short
- Vary your transmission location
- Use Tor as your primary transport when available

**Q: What is RTL-SDR monitoring?**

A: An optional feature (requires RTL-SDR hardware) that lets you monitor the radio spectrum around you. This provides situational awareness of other radio activity in your area.

**Q: Can I use amateur radio frequencies?**

A: If you hold a valid amateur radio license, you may use frequencies and power levels according to your license class. However:

- ⚠️ Encrypted communications may be prohibited on amateur bands
- ⚠️ Check your local amateur radio regulations
- ⚠️ You are responsible for compliance

### Plugin Architecture (Planned v0.4.0+)

**Q: What is the plugin architecture for?**

A: The plugin architecture is primarily designed for extended infrastructure testing scenarios. When operating multiple SimpleX clients that exchange messages automatically – for instance, stress testing your own SMP servers or validating message delivery across a distributed setup – you can run several TUI instances in parallel. Each instance connects to a separate SimpleX CLI backend and visualizes the message flow in real time.

**Q: What can I do with plugins?**

A:
- Run multiple TUI instances for parallel testing
- Monitor automated test conversations visually
- Hook into events and trigger custom actions
- Feed data into external monitoring tools (InfluxDB, Grafana)
- Create custom visualizations

**Q: Do I need plugins for normal use?**

A: No. Plugins are for advanced users who want to extend functionality or run automated tests. Normal messaging works without any plugins.

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

### Radio Transport Issues (v0.5.0+)

#### LoRa Module Not Detected
```bash
# Check if serial device is present
ls -la /dev/ttyUSB* /dev/ttyACM*

# Check permissions
sudo usermod -a -G dialout $USER
# Log out and log back in

# Check dmesg for device info
dmesg | grep -i tty
```

#### No Radio Communication

1. Verify both parties have radio hardware connected
2. Check that you're on the same frequency
3. Verify antenna is connected properly
4. Check signal strength indicator in TUI
5. Try moving to a location with better line of sight

#### Reticulum Not Connecting
```bash
# Check if Reticulum daemon is running
rnsd --version

# Start Reticulum daemon
rnsd

# Check Reticulum status
rnstatus
```

#### RTL-SDR Not Working
```bash
# Check if RTL-SDR is detected
rtl_test

# Install RTL-SDR drivers if needed (Debian/Ubuntu)
sudo apt install rtl-sdr

# Blacklist default DVB driver
echo 'blacklist dvb_usb_rtl28xxu' | sudo tee /etc/modprobe.d/blacklist-rtl.conf
sudo modprobe -r dvb_usb_rtl28xxu
```

---

## SimpleX CLI Issues

For issues with SimpleX Chat CLI itself (not SimpleX TUI):

- [SimpleX Chat Issues](https://github.com/simplex-chat/simplex-chat/issues)
- [SimpleX Documentation](https://simplex.chat/docs/)

## Reticulum Issues

For issues with Reticulum Network Stack (not SimpleX TUI):

- [Reticulum Issues](https://github.com/markqvist/Reticulum/issues)
- [Reticulum Documentation](https://reticulum.network/manual/)

---

## Contact

- **GitHub Issues:** [simplex-tui/issues](https://github.com/cannatoshi/simplex-tui/issues)
- **GitHub Discussions:** [simplex-tui/discussions](https://github.com/cannatoshi/simplex-tui/discussions)

---

*Last updated: January 2026*
