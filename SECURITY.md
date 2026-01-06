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
- Radio transport protocol implementation (v0.5.0+)

### Out of Scope

- SimpleX Chat CLI (report to [simplex-chat](https://github.com/simplex-chat/simplex-chat/security))
- SimpleX protocol issues
- Reticulum Network Stack (report to [Reticulum](https://github.com/markqvist/Reticulum))
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

### Architecture (Current - v0.1.x)
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

## Radio Transport Security (Planned - v0.5.0+)

The planned radio transport layer will implement its own cryptographic stack for off-grid communication. This section documents the security model.

### Cryptographic Primitives

| Layer | Method | Purpose |
|-------|--------|---------|
| **Key Exchange** | X25519 ECDH | Establish shared secret |
| **Encryption** | ChaCha20-Poly1305 | Authenticated encryption |
| **Signatures** | Ed25519 | Message authentication |
| **Key Derivation** | HKDF-SHA256 | Derive session keys |
| **Forward Secrecy** | Ephemeral session keys | Protect past communications |

### Protocol Modes

#### Custom Protocol (Closed Groups)

- Direct implementation of cryptographic primitives in Rust
- Designed for isolated networks without mesh interoperability
- Full control over protocol behavior
- Audits and review welcome

#### Reticulum Mode (Mesh Interoperability)

- Uses Reticulum Network Stack encryption
- Curve25519 + AES-128 or ChaCha20
- Compatible with Nomad Network, Sideband, MeshChat
- Security depends on Reticulum implementation
- See: https://reticulum.network/manual/crypto.html

### Threat Model Comparison

| Threat | Tor Transport | Radio Transport |
|--------|---------------|-----------------|
| Content interception | ✅ Protected | ✅ Protected |
| Metadata analysis | ✅ Protected | ⚠️ Partial |
| Traffic analysis | ✅ Protected | ⚠️ Partial |
| Direction finding | ✅ Protected | ❌ Vulnerable |
| Replay attacks | ✅ Protected | ✅ Protected |
| Man-in-the-middle | ✅ Protected | ✅ Protected |
| Infrastructure dependency | ⚠️ Requires internet | ✅ None |
| Censorship resistance | ⚠️ Tor can be blocked | ✅ No central point |

### Radio-Specific Security Considerations

1. **Direction Finding**: Unlike Tor, radio transmissions can be located using direction-finding equipment. Users in sensitive situations should be aware of this limitation.

2. **Signal Analysis**: Even with encrypted content, transmission patterns (timing, frequency, duration) may reveal information about communication activity.

3. **Physical Security**: Radio hardware can be seized or tampered with. Consider physical security of your device.

4. **Regulatory Compliance**: Transmitting on certain frequencies may be illegal in your jurisdiction. Non-compliance could lead to legal consequences and confiscation of equipment.

5. **Shared Spectrum**: ISM bands are shared with other users. Interference is possible and your transmissions may be noticed.

### Key Management (Radio Transport)
```
┌─────────────────────────────────────────────────────────────────┐
│  Identity Key (Long-term)                                       │
│  ├── Generated once per installation                            │
│  ├── Ed25519 keypair                                            │
│  └── Used for signing and identity verification                 │
├─────────────────────────────────────────────────────────────────┤
│  Session Key (Ephemeral)                                        │
│  ├── Generated per conversation session                         │
│  ├── X25519 ECDH exchange                                       │
│  └── Provides forward secrecy                                   │
├─────────────────────────────────────────────────────────────────┤
│  Message Key (Per-message)                                      │
│  ├── Derived from session key via HKDF                          │
│  ├── Unique per message                                         │
│  └── ChaCha20-Poly1305 authenticated encryption                 │
└─────────────────────────────────────────────────────────────────┘
```

### Security Recommendations for Radio Users

1. **Use Tor as Primary**: Radio should be a fallback, not primary transport. Tor provides better metadata protection.

2. **Limit Transmission Time**: Shorter transmissions are harder to locate.

3. **Vary Location**: If possible, don't always transmit from the same location.

4. **Verify Contacts**: Use out-of-band verification for contact identity keys.

5. **Monitor Spectrum**: Use RTL-SDR monitoring to be aware of your RF environment.

6. **Secure Hardware**: Keep your radio hardware physically secure.

---

## Cryptographic Implementation Notes

### Dependencies (Planned v0.5.0+)

| Crate | Version | Audit Status |
|-------|---------|--------------|
| x25519-dalek | 2.x | ✅ Audited |
| chacha20poly1305 | 0.10.x | ✅ RustCrypto |
| ed25519-dalek | 2.x | ✅ Audited |
| hkdf | 0.12.x | ✅ RustCrypto |
| rand | 0.8.x | ✅ Audited |

### Implementation Principles

1. **No Custom Cryptography**: We use well-established, audited cryptographic libraries.

2. **Fail Secure**: On any cryptographic error, the operation fails rather than falling back to insecure mode.

3. **Constant Time**: Cryptographic operations use constant-time implementations where available.

4. **Secure Random**: All random values are generated using cryptographically secure random number generators.

5. **Memory Safety**: Rust's memory safety guarantees help prevent common vulnerabilities.

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
