# Roadmap

This document outlines the planned development for SimpleX TUI.

> ⚠️ **Note:** This roadmap is subject to change based on community feedback and priorities.

---

## Vision

SimpleX TUI aims to be the best terminal interface for SimpleX Chat, providing:

- 🚀 Fast, keyboard-driven workflow
- 🔒 Privacy-focused design
- 🎨 Beautiful terminal aesthetics
- ♿ Accessible to all users
- 📡 Resilient communication with optional radio transport

---

## Current Status

**Version:** 0.1.1-alpha  
**Stage:** Early Development  
**Stability:** Alpha (expect bugs)

---

## 2026 Roadmap

### Q1 2026: Foundation

#### v0.2.0 - File Transfers & Groups
**Target:** February 2026

- [ ] 📁 File transfer support (send/receive)
- [ ] 👥 Group chat improvements
- [ ] 💬 Message reactions
- [ ] 📎 File attachment preview
- [ ] 🔔 Desktop notifications (optional)

#### v0.2.x - Polish
**Target:** March 2026

- [ ] 🐛 Bug fixes from community feedback
- [ ] ⚡ Performance optimizations
- [ ] 📖 Improved documentation
- [ ] 🧪 Basic test coverage

---

### Q2 2026: Privacy & Security

#### v0.3.0 - Privacy Features
**Target:** April 2026

- [ ] 🧅 Tor integration (SOCKS5 proxy)
- [ ] 🔐 Encryption status indicators
- [ ] ✅ Contact verification UI
- [ ] 🕐 Disappearing messages UI
- [ ] 📊 Connection quality indicator

#### v0.3.x - Hardening
**Target:** May-June 2026

- [ ] 🔒 Security audit (self)
- [ ] 🛡️ Input sanitization review
- [ ] 📝 Security documentation
- [ ] 🧪 Security-focused tests

---

### Q3 2026: Customization & Plugin Architecture

#### v0.4.0 - Theming & Configuration
**Target:** July 2026

- [ ] 🎨 Custom color themes
- [ ] ⌨️ Configurable keybindings
- [ ] 📁 Config file support (~/.config/simplex-tui/)
- [ ] 🔤 Font/Unicode options
- [ ] 🌍 Internationalization (i18n) foundation

#### v0.4.x - Search, Navigation & Plugins
**Target:** August-September 2026

- [ ] 🔍 Message search
- [ ] 📜 Chat history navigation
- [ ] 🏷️ Contact tags/groups
- [ ] ⭐ Favorite contacts
- [ ] 📌 Pinned conversations
- [ ] 🔌 Plugin architecture foundation
- [ ] 🧪 Multi-instance support for parallel testing

---

### Q4 2026: Radio Transport (Experimental)

#### v0.5.0 - LoRa Foundation
**Target:** October 2026

- [ ] 📡 Serial interface to LoRa module
- [ ] 🔐 Curve25519 key exchange over radio
- [ ] 🔒 ChaCha20-Poly1305 encryption for radio transport
- [ ] 📨 Basic encrypted message transport
- [ ] 🔄 Automatic transport fallback (Tor → Radio)
- [ ] 👤 Per-contact capability detection
- [ ] 📊 Signal strength indicator

#### v0.5.x - Protocol Options
**Target:** November 2026

- [ ] 🌐 Reticulum/LXMF compatibility mode
- [ ] 🔒 Custom lightweight protocol for closed groups
- [ ] 📊 Link quality display
- [ ] 🗺️ GPS-aware contact distance (optional)
- [ ] 📡 Multi-channel support

---

### 2027 Roadmap

### Q1 2027: Stability & Release

#### v0.6.0 - Beta Preparation
**Target:** January 2027

- [ ] 🧪 Comprehensive test suite
- [ ] 📖 Complete documentation
- [ ] 🐛 Bug bash and fixes
- [ ] ⚡ Performance profiling
- [ ] ♿ Accessibility review

#### v1.0.0 - Stable Release
**Target:** March 2027

- [ ] 🎉 First stable release
- [ ] 📦 Package manager submissions
  - [ ] Homebrew (macOS)
  - [ ] AUR (Arch Linux)
  - [ ] crates.io
- [ ] 🔄 Stable API
- [ ] 📖 User manual
- [ ] 🌍 Multiple languages

---

### Q2 2027: Advanced Radio Features (Post-Stable)

#### v1.1.0 - SDR Integration (Optional)
**Target:** May 2027

- [ ] 📻 RTL-SDR spectrum monitoring integration
- [ ] 📈 Waterfall display in TUI
- [ ] 🔍 Signal detection alerts
- [ ] 📡 Multi-band awareness
- [ ] 🎯 Direction finding assistance (experimental)

#### v1.2.0 - Mesh Expansion
**Target:** July 2027

- [ ] 🔁 Store-and-forward for offline contacts
- [ ] 🌉 Bridge mode (radio ↔ internet gateway)
- [ ] 📍 GPS-based routing optimization
- [ ] 🗺️ Network topology visualization

---

## Future Ideas (Post 1.0)

These are ideas for future development, not committed:

### Core Features
- 🖼️ Image preview in terminal (sixel/kitty)
- 🎤 Voice message playback
- 📹 Video call notifications
- 🤖 Bot/automation support
- 📊 Usage statistics (local only)

### Radio & Hardware
- 📡 Support for additional LoRa modules (RNode, Meshtastic devices)
- 🔋 Battery-optimized mode for portable operation
- 📻 HF packet radio support (experimental)
- 🛰️ Satellite link support (Iridium, etc.)
- 🔌 GPIO integration for status LEDs

### Platforms
- 📦 Flatpak package
- 📦 Snap package
- 📦 Debian/Ubuntu package
- 📦 RPM package
- 🪟 Windows binary releases
- 🍇 Raspberry Pi optimized builds

### Integrations
- 🔗 tmux integration
- 🔗 SSH-friendly mode
- 🔗 Screen reader support
- 🔗 CLI mode (non-interactive)
- 📊 InfluxDB/Grafana export for monitoring
- 🔗 Prometheus metrics endpoint

### Testing & Development
- 🧪 Automated multi-client test harness
- 📊 Message delivery analytics
- 🔬 Protocol debugging tools
- 📈 Performance benchmarking suite

---

## Hardware Compatibility (Radio Features)

The radio transport layer (v0.5.0+) will support:

| Hardware | Status | Notes |
|----------|--------|-------|
| **uConsole AIO V2** | Primary target | LoRa + RTL-SDR + GPS integrated |
| **RNode** | Planned | Reticulum-native device |
| **USB LoRa Modules** | Planned | SX1262/SX1276 based |
| **Meshtastic Devices** | Evaluation | Possible bridge mode |

### Range Expectations

| Environment | Metric | Imperial |
|-------------|--------|----------|
| Urban (buildings) | 2-5 km | 1-3 miles |
| Suburban | 8-15 km | 5-10 miles |
| Rural / Open | 15-30 km | 10-20 miles |
| Line of sight (hilltop) | 50+ km | 30+ miles |

---

## How to Contribute

Want to help achieve these goals?

1. Check [CONTRIBUTING.md](CONTRIBUTING.md)
2. Look for issues labeled `good first issue`
3. Join discussions on GitHub
4. Report bugs and suggest features

### Priority Areas for Contributors

| Area | Difficulty | Impact |
|------|------------|--------|
| File Transfer UI | Medium | High |
| Group Chat Support | Hard | High |
| Theme System | Easy | Medium |
| Vim Keybindings | Easy | Medium |
| Documentation | Easy | High |
| Radio Protocol Testing | Hard | High |
| Reticulum Integration | Medium | Medium |

---

## Feedback

Have ideas for the roadmap?

- 💬 [GitHub Discussions](https://github.com/cannatoshi/simplex-tui/discussions)
- ✨ [Feature Requests](https://github.com/cannatoshi/simplex-tui/issues/new?template=feature_request.md)

---

## Progress Tracking

Track our progress:

- [GitHub Milestones](https://github.com/cannatoshi/simplex-tui/milestones)
- [Project Board](https://github.com/cannatoshi/simplex-tui/projects)

---

*Last updated: January 2026*
```
