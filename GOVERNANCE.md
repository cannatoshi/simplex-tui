# Project Governance

This document describes the governance model for SimpleX TUI.

---

## Overview

SimpleX TUI is an open-source project maintained by individual contributors. This document outlines how the project is organized and how decisions are made.

---

## Project Leadership

### Maintainer

| Role | Person | GitHub |
|------|--------|--------|
| **Lead Maintainer** | Sascha Daemgen | [@cannatoshi](https://github.com/cannatoshi) |

The maintainer is responsible for:

- Setting project direction and roadmap
- Reviewing and merging pull requests
- Managing releases
- Ensuring code quality
- Community management
- Security response

---

## Decision Making

### Day-to-Day Decisions

- Made by the maintainer
- Guided by project goals and community feedback

### Significant Changes

For significant changes (new features, breaking changes, architectural decisions):

1. **Proposal:** Open a GitHub Issue or Discussion
2. **Discussion:** Community feedback period
3. **Decision:** Maintainer makes final decision
4. **Implementation:** Via pull request

### What Counts as Significant?

- Breaking API changes
- New major features
- Dependency changes
- Security-related changes
- Changes to governance

---

## Roles

### Maintainer

- Full commit access
- Release authority
- Final decision authority
- Security response

### Contributors

Anyone who contributes to the project:

- Code contributions
- Documentation
- Bug reports
- Feature suggestions
- Community support

### How to Become a Contributor

1. Fork the repository
2. Make a contribution
3. Submit a pull request
4. Follow [CONTRIBUTING.md](CONTRIBUTING.md)

---

## Code of Conduct

All participants must follow our [Code of Conduct](CODE_OF_CONDUCT.md).

Violations can be reported via GitHub Issues.

---

## Releases

### Release Process

1. Version bump in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag
4. Push release

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR:** Breaking changes
- **MINOR:** New features (backwards compatible)
- **PATCH:** Bug fixes (backwards compatible)

During alpha/beta:
- `0.x.y` - API may change
- `1.0.0` - First stable release

---

## Communication

### Official Channels

| Channel | Purpose |
|---------|---------|
| [GitHub Issues](https://github.com/cannatoshi/simplex-tui/issues) | Bug reports, feature requests |
| [GitHub Discussions](https://github.com/cannatoshi/simplex-tui/discussions) | Questions, ideas, community |
| [GitHub Pull Requests](https://github.com/cannatoshi/simplex-tui/pulls) | Code contributions |

### Language

- Primary language: English
- German also accepted

---

## Licensing

- Project license: AGPL-3.0
- Contributions must be compatible with AGPL-3.0
- Contributors retain copyright to their contributions
- By contributing, you agree to license under AGPL-3.0

See [LICENSE](LICENSE) for details.

---

## Changes to Governance

This governance document may be updated by the maintainer. Significant changes will be announced via GitHub.

---

## Acknowledgments

This governance model is inspired by other open-source projects and adapted for our needs.

---

*Last updated: January 2026*
