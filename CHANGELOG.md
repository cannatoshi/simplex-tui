# Changelog

All notable changes to SimpleX TUI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- File transfer support
- Group chat improvements
- Message search
- Custom themes

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
| 0.1.1-alpha | 2026-01-06 | UI/UX redesign, address management |
| 0.1.0-alpha | 2026-01-05 | Initial release |

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

---

[Unreleased]: https://github.com/cannatoshi/simplex-tui/compare/v0.1.1-alpha...HEAD
[0.1.1-alpha]: https://github.com/cannatoshi/simplex-tui/compare/v0.1.0-alpha...v0.1.1-alpha
[0.1.0-alpha]: https://github.com/cannatoshi/simplex-tui/releases/tag/v0.1.0-alpha
