# Security Policy

---

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

---

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in SimpleX TUI, please report it responsibly.

### Preferred Method: GitHub Private Vulnerability Reporting

1. Go to our [Security Advisories](https://github.com/cannatoshi/simplex-tui/security/advisories)
2. Click **"Report a vulnerability"**
3. Fill out the form with details

### Alternative: Email

If GitHub reporting is not possible, create a private issue or contact via GitHub.

---

## What to Include

Please provide as much information as possible:

- **Description** of the vulnerability
- **Steps to reproduce** the issue
- **Affected versions**
- **Potential impact** assessment
- **Suggested fix** (if any)

---

## Response Timeline

| Phase | Timeframe |
|-------|-----------|
| Acknowledgment | Within 48 hours |
| Initial Assessment | Within 5 business days |
| Status Update | Every 7 days |
| Fix Development | Depends on severity |
| Public Disclosure | After fix is released |

---

## Severity Classification

| Severity | Description | Response |
|----------|-------------|----------|
| **Critical** | Remote code execution, data breach | Immediate action |
| **High** | Authentication bypass, privilege escalation | Priority fix |
| **Medium** | Information disclosure, DoS | Scheduled fix |
| **Low** | Minor issues, hardening | Future release |

---

## Scope

### In Scope

- SimpleX TUI application code
- Build and deployment scripts
- Documentation that could lead to security issues
- Dependencies with known vulnerabilities

### Out of Scope

- SimpleX Chat CLI (report to [simplex-chat](https://github.com/simplex-chat/simplex-chat/security))
- SimpleX protocol issues
- Social engineering attacks
- Physical attacks
- Issues in user's environment

---

## Safe Harbor

We support responsible security research. If you:

- Act in good faith
- Avoid privacy violations
- Avoid data destruction
- Do not exploit issues beyond verification
- Report promptly

We will:

- ✅ Not pursue legal action
- ✅ Work with you to understand the issue
- ✅ Credit you in the advisory (if desired)
- ✅ Keep you informed of our progress

---

## Security Best Practices for Users

### System Security
```bash
# Keep SimpleX TUI updated
cd ~/simplex-tui
git pull
cargo build --release

# Keep SimpleX CLI updated
# Follow SimpleX Chat update instructions
```

### Configuration Security

- Run SimpleX CLI with minimal permissions
- Use Tor for enhanced privacy
- Keep your system and dependencies updated
- Review logs for suspicious activity

### Data Security

- Backup your SimpleX data directory regularly
- Use encrypted storage for sensitive data
- Secure your WebSocket connection (localhost only by default)

---

## Known Security Considerations

### Architecture
```
┌─────────────────┐     WebSocket      ┌─────────────────┐
│  SimpleX TUI    │◄──────────────────►│  SimpleX CLI    │
│  (This project) │     localhost      │  (Backend)      │
└─────────────────┘                    └─────────────────┘
```

### Important Notes

1. **No Encryption Handling**: SimpleX TUI does NOT handle encryption. All cryptographic operations are performed by SimpleX Chat CLI.

2. **Local WebSocket**: By default, WebSocket runs on localhost only. Exposing it to a network is NOT recommended.

3. **No Key Storage**: SimpleX TUI does NOT store or access private keys.

4. **Trust Model**: Security depends on SimpleX Chat CLI. We are a frontend only.

---

## Acknowledgments

We thank all security researchers who help improve SimpleX TUI.

### Hall of Fame

*No vulnerabilities reported yet. Be the first responsible disclosure!*

---

## Contact

- **Security Issues**: [GitHub Private Vulnerability Reporting](https://github.com/cannatoshi/simplex-tui/security/advisories)
- **General Issues**: [GitHub Issues](https://github.com/cannatoshi/simplex-tui/issues)

---

*Last updated: January 2026*
