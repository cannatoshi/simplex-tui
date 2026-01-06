# Changelog

All notable changes to SimpleX TUI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned for v0.2.0
- 📁 File transfer support (send/receive)
- 👥 Group chat improvements
- 💬 Message reactions
- 📎 File attachment preview
- 🔔 Desktop notifications (optional)

### Planned for v0.3.0
- 🧅 Tor integration (SOCKS5 proxy)
- 🔐 Encryption status indicators
- ✅ Contact verification UI
- 🕐 Disappearing messages UI

### Planned for v0.4.0
- 🎨 Custom color themes
- ⌨️ Configurable keybindings
- 🔌 Plugin architecture
- 🧪 Multi-instance support for parallel testing

### Planned for v0.5.0 (Radio Transport - Experimental)
- 📡 LoRa radio transport layer
- 🔐 Curve25519/ChaCha20-Poly1305 encryption for radio
- 🌐 Reticulum/LXMF compatibility mode
- 🔒 Custom lightweight protocol for closed groups
- 🔄 Automatic transport fallback (Tor → Radio)
- 👤 Per-contact capability detection
- 📊 Signal strength indicator
- 🗺️ GPS integration (optional)
- 📻 RTL-SDR spectrum monitoring (optional)

---

## [0.1.1-alpha] - 2026-01-06

### Added
- ✨ Full UI/UX redesign with box-bordered buttons
- ✨ Address management workflow (Delete → Create → Refresh)
- ✨ Fullscreen modals for all dialogs
- ✨ Mouse click support throughout UI
- ✨ Animated status bar with typewriter effect
- ✨ Unread message counter badges
- ✨ Proper message routing to correct conversations

### Changed
- 🎨 Buttons now use box drawing characters (│ │)
- 🎨 Modals are fullscreen with centered content
- 🎨 Improved visual hierarchy and spacing
- ♻️ Refactored modal system for consistency

### Fixed
- 🐛 Messages now route to correct contact/conversation
- 🐛 Unread counters update properly
- 🐛 Address display after creation
- 🐛 Modal keyboard navigation

### Documentation
- 📖 Added ROADMAP.md with detailed development timeline
- 📖 Added radio transport milestones and hardware compatibility
- 📖 Added SECURITY.md with radio transport security model
- 📖 Added comprehensive radio disclaimers to DISCLAIMER.md
- 📖 Added radio dependencies and legal framework to LEGAL.md
- 📖 Added radio transport FAQ to SUPPORT.md
- 📖 Added third-party trademarks to TRADEMARK.md

---

## [0.1.0-alpha] - 2026-01-05

### Added
- 🎉 Initial release
- ✨ Terminal user interface using ratatui
- ✨ WebSocket connection to SimpleX CLI
- ✨ Contact list with selection
- ✨ Chat view with message history
- ✨ Message composition and sending
- ✨ Real-time message receiving
- ✨ Keyboard navigation (j/k, arrows, Tab)
- ✨ Basic modal dialogs
- ✨ Status bar with connection info
- ✨ Three-panel layout (contacts, chat, input)

### Technical
- 🔧 Async runtime with tokio
- 🔧 WebSocket client with tokio-tungstenite
- 🔧 TUI framework with ratatui + crossterm
- 🔧 JSON serialization with serde

---

## Version History

| Version | Date | Highlights |
|---------|------|------------|
| 0.1.1-alpha | 2026-01-06 | UI/UX redesign, address management, documentation overhaul |
| 0.1.0-alpha | 2026-01-05 | Initial release |

---

## Upcoming Releases

| Version | Target | Highlights |
|---------|--------|------------|
| 0.2.0 | Feb 2026 | File transfers, group chats |
| 0.3.0 | Apr 2026 | Tor integration, encryption indicators |
| 0.4.0 | Jul 2026 | Themes, plugins, multi-instance testing |
| 0.5.0 | Oct 2026 | Radio transport (LoRa, Reticulum) |
| 0.6.0 | Jan 2027 | Beta preparation, comprehensive testing |
| 1.0.0 | Mar 2027 | First stable release |

See [ROADMAP.md](ROADMAP.md) for detailed plans.

---

## Legend

| Emoji | Type |
|-------|------|
| 🎉 | Initial release |
| ✨ | New feature |
| 🎨 | UI/Style change |
| ♻️ | Refactor |
| 🐛 | Bug fix |
| 🔧 | Technical/Config |
| 📖 | Documentation |
| 🔒 | Security |
| ⚡ | Performance |
| 🗑️ | Deprecation |
| 💥 | Breaking change |
| 📡 | Radio transport |
| 🧅 | Tor/Privacy |
| 🔌 | Plugin system |

---

[Unreleased]: https://github.com/cannatoshi/simplex-tui/compare/v0.1.1-alpha...HEAD
[0.1.1-alpha]: https://github.com/cannatoshi/simplex-tui/compare/v0.1.0-alpha...v0.1.1-alpha
[0.1.0-alpha]: https://github.com/cannatoshi/simplex-tui/releases/tag/v0.1.0-alpha
```
