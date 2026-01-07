# Contributing to SimpleX TUI

First off, thank you for considering contributing to SimpleX TUI! 🎉

This document provides guidelines for contributing to the project.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Style Guidelines](#style-guidelines)

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---

## Getting Started

### Prerequisites

- Rust 1.92+ (stable)
- SimpleX Chat CLI installed and running
- Git

### Quick Setup
```bash
# Clone the repository
git clone https://github.com/cannatoshi/simplex-tui.git
cd simplex-tui

# Build
cargo build

# Run
cargo run
```

---

## How to Contribute

### Reporting Bugs

1. Check [existing issues](https://github.com/cannatoshi/simplex-tui/issues)
2. Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md)
3. Include reproduction steps and environment details

### Suggesting Features

1. Check [existing feature requests](https://github.com/cannatoshi/simplex-tui/issues?q=label%3Aenhancement)
2. Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md)
3. Explain the use case and proposed solution

### Code Contributions

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

## Development Setup

### Clone and Build
```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/simplex-tui.git
cd simplex-tui

# Add upstream remote
git remote add upstream https://github.com/cannatoshi/simplex-tui.git

# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Project Structure
```
simplex-tui/
├── src/
│   ├── main.rs           # Entry point
│   ├── app.rs            # Application state
│   ├── ui.rs             # UI rendering
│   ├── websocket.rs      # WebSocket client
│   ├── events.rs         # Event handling
│   └── types.rs          # Data types
├── Cargo.toml            # Dependencies
└── README.md             # Documentation
```

### Running Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check without building
cargo check
```

---

## Commit Guidelines

We use **Conventional Commits** with emojis for clear commit history.

### Format
```
<type>(<scope>): <description> <emoji>

[optional body]

[optional footer]
Signed-off-by: Your Name <your.email@example.com>
```

### Types

| Type | Emoji | Description |
|------|-------|-------------|
| `feat` | ✨ | New feature |
| `fix` | 🐛 | Bug fix |
| `docs` | 📖 | Documentation |
| `style` | 🎨 | Code style (formatting) |
| `refactor` | ♻️ | Code refactoring |
| `perf` | ⚡ | Performance improvement |
| `test` | 🧪 | Adding tests |
| `chore` | 🔧 | Maintenance tasks |
| `ci` | 👷 | CI/CD changes |
| `security` | 🔒 | Security fixes |

### Examples
```bash
# Feature
git commit -s -m "feat(ui): Add contact search functionality ✨"

# Bug fix
git commit -s -m "fix(websocket): Handle reconnection on timeout 🐛"

# Documentation
git commit -s -m "docs(readme): Update installation instructions 📖"

# Refactor
git commit -s -m "refactor(app): Extract message handling to module ♻️"
```

### Developer Certificate of Origin (DCO)

All commits must be signed off to certify you have the right to submit the code:
```bash
# Sign off commits with -s flag
git commit -s -m "Your commit message"

# This adds:
# Signed-off-by: Your Name <your.email@example.com>
```

By signing off, you agree to the [DCO](https://developercertificate.org/):
```
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution is maintained
    indefinitely and may be redistributed consistent with this
    project or the open source license(s) involved.
```

---

## Pull Request Process

### Before Submitting

1. ✅ Fork and create a feature branch
2. ✅ Make your changes
3. ✅ Run `cargo fmt` and `cargo clippy`
4. ✅ Test your changes
5. ✅ Sign off your commits (`git commit -s`)
6. ✅ Update documentation if needed

### Submitting

1. Push to your fork
2. Create a Pull Request against `main`
3. Fill out the PR template
4. Wait for review

### Review Process

- Maintainers will review your PR
- Address any requested changes
- Once approved, your PR will be merged

### After Merge
```bash
# Update your fork
git checkout main
git pull upstream main
git push origin main

# Delete feature branch
git branch -d your-feature-branch
```

---

## Style Guidelines

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write descriptive variable names
- Add comments for complex logic

### Code Examples
```rust
// Good: Descriptive names
fn render_contact_list(contacts: &[Contact], selected: usize) -> Paragraph {
    // ...
}

// Bad: Unclear names
fn render(c: &[Contact], s: usize) -> Paragraph {
    // ...
}
```

### Documentation

- Add doc comments for public functions
- Include examples where helpful
- Keep comments up to date
```rust
/// Renders the contact list widget.
///
/// # Arguments
///
/// * `contacts` - Slice of contacts to display
/// * `selected` - Index of currently selected contact
///
/// # Returns
///
/// A styled Paragraph widget ready for rendering
fn render_contact_list(contacts: &[Contact], selected: usize) -> Paragraph {
    // ...
}
```

---

## Questions?

- 💬 [GitHub Discussions](https://github.com/cannatoshi/simplex-tui/discussions)
- 🐛 [GitHub Issues](https://github.com/cannatoshi/simplex-tui/issues)

---

Thank you for contributing! 💙

*Last updated: January 2026*
