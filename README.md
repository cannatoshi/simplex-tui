# SimpleX TUI

Terminal UI for SimpleX Chat - Built for Cyberdecks

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)

## Features

- Clean terminal interface for SimpleX messaging
- Keyboard-driven navigation (vim-style)
- Optimized for small screens (Clockwork Pi uConsole, Raspberry Pi)
- Panic button for emergency LUKS operations (planned)

## Installation
```bash
git clone https://github.com/cannatoshi/simplex-tui
cd simplex-tui
cargo build --release
./target/release/simplex-tui
```

## Keyboard

| Key | Action |
|-----|--------|
| `Tab` | Switch panel |
| `j/k` | Navigate up/down |
| `Enter` | Select / Send |
| `Esc` | Back |
| `i` | Add contact |
| `c` | Create link |
| `?` | Help |
| `q` | Quit |
| `F12` | Panic mode |

## Roadmap

- [ ] SimpleX WebSocket API integration
- [ ] Real contact/message sync
- [ ] File transfer
- [ ] LUKS panic button
- [ ] ARM64 builds for Raspberry Pi

## License

MIT
