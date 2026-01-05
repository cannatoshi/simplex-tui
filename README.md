# SimpleX TUI

## Secure Terminal Messenger for Cyberdeck Systems

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20ARM64%2Fx86__64-lightgrey.svg)](https://kernel.org/)
[![Status](https://img.shields.io/badge/Status-Alpha-orange.svg)](#status)
[![Tor](https://img.shields.io/badge/Tor-Supported-7D4698.svg)](https://www.torproject.org/)
[![SimpleX](https://img.shields.io/badge/SimpleX-Compatible-00A896.svg)](https://simplex.chat/)
[![Security](https://img.shields.io/badge/Security-Hardened-red.svg)](#security-model)
[![Target](https://img.shields.io/badge/Target-Clockwork%20Pi%20uConsole-cyan.svg)](https://www.clockworkpi.com/)

A minimal, security-focused terminal interface for SimpleX Chat, designed for deployment on hardened cyberdeck systems like the Clockwork Pi uConsole. Part of the upcoming **SentinelX Security Suite** for journalists, whistleblowers, activists, and anyone requiring maximum operational security.

**This is not just a chat client** - it's a complete covert communication system with plausible deniability, hidden activation, and emergency data destruction.

> **Version:** 0.1.0-alpha (January 2026)  
> **Status:** Active Development  
> **Target Hardware:** Clockwork Pi uConsole, Raspberry Pi 4/5, any Linux terminal  
> **Part of:** [SentinelX Security Suite](#sentinelx-security-suite)

### Terminal Interface
```
┌──────────────────────────────────────────────────────────────────────────────┐
│ ▸ Support            │ Support  21:25                                        │
│                      │ First Cyberdeck TUI 4 SimpleX CLI Interface           │
│                      │                                                       │
│                      │ You  21:27                                            │
│                      │ Nice Work                                             │
│                      │                                                       │
│                      │ Support  21:35                                        │
│                      │ :) :) :) :)                                           │
│                      │                                                       │
│                      │ Support  22:51                                        │
│                      │ let`s code ...                                        │
│                      │                                                       │
│                      │ You  22:51                                            │
│                      │ yeah xD                                               │
│                      │                                                       │
│ [i] Add [r] Refresh  │ ● 📩 Support                                          │
└──────────────────────────────────────────────────────────────────────────────┘
```

*Real-time messaging via WebSocket - no polling, instant message delivery*

---

> ⚠️ **ALPHA SOFTWARE - SECURITY FEATURES IN DEVELOPMENT**
>
> This project is in active development. Core messaging works, security features are being implemented.
> 
> ✅ **What works:** Real-time WebSocket messaging, contact list, chat history, message sending, auto-scroll, panic button UI  
> 🚧 **In progress:** Stealth mode, wipe code, systemd sandboxing, USB lockdown  
> 📋 **Planned:** Hidden volumes, dead man's switch, hardware security module integration

---

## 🔒 What's New in v0.1.0-alpha

This release implements the **core messaging foundation**:

### Real-Time WebSocket Integration
- **Push Event Architecture** - Messages appear instantly without polling
- **Bidirectional Communication** - Send and receive via WebSocket API
- **Contact Management** - Load and switch between contacts
- **Chat History** - Load last 50 messages per contact
- **Auto-Scroll** - New messages always visible at bottom

### Security Foundation
- **F12 Panic Button** - Emergency response system (UI ready)
- **Minimal Attack Surface** - No browser, no Electron, pure Rust
- **Terminal-Only** - Works in TTY, no X11/Wayland required
- **SSH Compatible** - Manage remotely via Tor hidden service

### Technical Implementation
- **Async Tokio Runtime** - Non-blocking WebSocket handling
- **Thread-Safe Channels** - mpsc for UI ↔ WebSocket communication
- **Ratatui TUI Framework** - Modern terminal UI rendering
- **50ms Refresh Rate** - Smooth, responsive interface

---

## Table of Contents

### Overview
1. [Project Vision](#project-vision)
2. [Why Terminal UI?](#why-terminal-ui)
3. [SentinelX Security Suite](#sentinelx-security-suite)

### Security Architecture
4. [Security Model](#security-model)
5. [Plausible Deniability System](#plausible-deniability-system)
6. [Panic System (F12)](#panic-system-f12)
7. [Wipe Code System](#wipe-code-system)
8. [Hidden Volume Support](#hidden-volume-support)

### System Hardening
9. [Systemd Sandboxing](#systemd-sandboxing)
10. [USB Port Lockdown](#usb-port-lockdown)
11. [Kernel Hardening](#kernel-hardening)
12. [Memory Protection](#memory-protection)

### Installation
13. [Prerequisites](#prerequisites)
14. [Quick Start](#quick-start)
15. [Building from Source](#building-from-source)
16. [Hardened Installation](#hardened-installation)

### Usage
17. [Basic Operation](#basic-operation)
18. [Keyboard Shortcuts](#keyboard-shortcuts)
19. [Stealth Mode](#stealth-mode)

### Development
20. [Architecture](#architecture)
21. [Roadmap](#roadmap)
22. [Contributing](#contributing)
23. [Related Projects](#related-projects)
24. [License](#license)

---

## Project Vision

### Target Hardware

| Device | Description | Use Case |
|--------|-------------|----------|
| **Clockwork Pi uConsole** | 5" IPS, CM4, QWERTY keyboard, 4G LTE | Primary target - portable secure comms |
| **Raspberry Pi 5** | Desktop replacement | Stationary secure terminal |
| **Raspberry Pi 4** | Proven reliability | Field deployment |
| **Raspberry Pi Zero 2W** | Minimal footprint | Disposable/hidden deployment |
| **Any Linux Device** | x86_64 or ARM64 | Flexibility |

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CLOCKWORK PI uCONSOLE - PRIMARY TARGET DEVICE                              │
│  ─────────────────────────────────────────────                              │
│                                                                             │
│  • 5-inch IPS display (1280x720)                                            │
│  • Raspberry Pi CM4 compute module                                          │
│  • Full QWERTY keyboard - no touchscreen needed                             │
│  • 4G LTE module support - mobile connectivity                              │
│  • Battery-powered - 4-6 hours operation                                    │
│  • Compact form factor - fits in cargo pocket                               │
│  • Looks like a gaming device - plausible cover story                       │
│                                                                             │
│  Perfect for: Journalists in hostile regions, activists, whistleblowers     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Why Terminal UI?

| Feature | GUI Apps | SimpleX TUI |
|---------|----------|-------------|
| Attack Surface | Large (browser, Electron) | Minimal (pure Rust) |
| Dependencies | Hundreds of packages | ~10 crates |
| Resource Usage | 500MB+ RAM | <20MB RAM |
| Remote Access | Complex (VNC, RDP) | Native SSH |
| GUI Required | Yes (X11/Wayland) | No (TTY works) |
| Plausible Deniability | Difficult | **Native** |
| Visual Observation | Easy to identify | Looks like normal terminal |
| Startup Time | 5-30 seconds | <1 second |

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  WHY TERMINAL-BASED?                                                        │
│                                                                             │
│  • Minimal attack surface - no browser engine, no JavaScript runtime        │
│  • Works over SSH - manage remotely via Tor hidden service                  │
│  • Low resource usage - runs on Raspberry Pi Zero 2W (512MB RAM)            │
│  • No GUI dependencies - works in pure TTY, no X11/Wayland required         │
│  • Keyboard-driven - fast operation, no touchscreen/mouse needed            │
│  • LOOKS LIKE A NORMAL TERMINAL - ultimate plausible deniability            │
│  • Instant stealth switch - messenger vanishes in <100ms                    │
│  • Scriptable - integrate with automated security systems                   │
│  • Screen-reader compatible - accessibility for visually impaired           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## SentinelX Security Suite

SimpleX TUI is a core component of **SentinelX** - a comprehensive security suite for high-risk users requiring uncompromising operational security.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         SENTINELX COMPONENTS                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐              │
│  │  SimpleX TUI    │  │  SMP Monitor    │  │  Infrastructure │              │
│  │  ─────────────  │  │  ─────────────  │  │  ─────────────  │              │
│  │  Secure chat    │  │  Server health  │  │  Tor-only SMP   │              │
│  │  client for     │  │  monitoring &   │  │  & XFTP servers │              │
│  │  cyberdecks     │  │  stress testing │  │  on Raspberry Pi│              │
│  │                 │  │                 │  │                 │              │
│  │ [THIS PROJECT]  │  │  [v0.1.11]      │  │  [Tutorial]     │              │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘              │
│           │                    │                    │                       │
│           └────────────────────┼────────────────────┘                       │
│                                │                                            │
│                                ▼                                            │
│                    ┌───────────────────────┐                                │
│                    │   Hardened Platform   │                                │
│                    │   ─────────────────   │                                │
│                    │   • LUKS encryption   │                                │
│                    │   • Systemd sandbox   │                                │
│                    │   • USB lockdown      │                                │
│                    │   • Kernel hardening  │                                │
│                    │   • Wipe code support │                                │
│                    │   • Hidden volumes    │                                │
│                    └───────────────────────┘                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

| Component | Repository | Status | Description |
|-----------|------------|--------|-------------|
| **SimpleX TUI** | This repo | v0.1.0-alpha | Secure terminal messenger |
| **SMP Monitor** | cannatoshi/simplex-smp-monitor | v0.1.11 | Server monitoring & testing |
| **Infrastructure** | cannatoshi/simplex-smp-xftp-via-tor | Stable | Tor-only server setup guide |

---

## Security Model

### Threat Model

SimpleX TUI is designed to protect against:

| Threat | Protection |
|--------|------------|
| **Nation-state network surveillance** | All traffic via Tor, no clearnet leaks |
| **Physical device seizure** | LUKS encryption, wipe code, hidden volumes |
| **Coerced access (rubber-hose)** | Wipe code destroys data, appears to unlock |
| **Traffic analysis** | Tor + self-hosted servers, no metadata |
| **Visual observation** | Stealth mode, instant SSH switch |
| **Border crossing inspection** | Plausible deniability, decoy data |
| **Evil maid attacks** | Secure boot, USB lockdown, tamper detection |
| **Forensic analysis** | Memory wipe, no swap, encrypted everything |

### Defense Layers

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  Layer 1: PLAUSIBLE DENIABILITY                                             │
│  ─────────────────────────────────                                          │
│  • Device boots to normal SSH terminal                                      │
│  • Messenger invisible until secret trigger entered                         │
│  • No visible simplex processes or files                                    │
│  • F12 → SSH switches to normal terminal instantly                          │
│  • Fake command history displayed on switch                                 │
│  • Device looks like normal development machine                             │
│                                                                             │
│  Layer 2: NETWORK ISOLATION                                                 │
│  ─────────────────────────────                                              │
│  • All traffic routed through Tor (mandatory)                               │
│  • Self-hosted SMP/XFTP as Tor v3 hidden services                           │
│  • No DNS queries, no IP exposure, no ISP visibility                        │
│  • Closed user group - servers invisible to discovery                       │
│                                                                             │
│  Layer 3: CRYPTOGRAPHIC PROTECTION                                          │
│  ─────────────────────────────────                                          │
│  • SimpleX double-ratchet encryption (Signal protocol)                      │
│  • Post-quantum resistant key exchange (ML-KEM)                             │
│  • Perfect forward secrecy                                                  │
│  • No user identifiers - not even random IDs                                │
│  • LUKS full-disk encryption with Argon2 KDF                                │
│                                                                             │
│  Layer 4: SYSTEM HARDENING                                                  │
│  ─────────────────────────────                                              │
│  • Systemd sandboxing (no Firejail/AppArmor needed)                         │
│  • USB ports disabled at kernel level                                       │
│  • Kernel hardening parameters                                              │
│  • No core dumps, no swap (or encrypted swap)                               │
│  • Secure boot chain verification                                           │
│                                                                             │
│  Layer 5: EMERGENCY RESPONSE                                                │
│  ──────────────────────────────                                             │
│  • F12 PANIC button - instant threat response                               │
│  • SSH stealth mode - messenger vanishes in <100ms                          │
│  • NUKE option - cryptographic key destruction                              │
│  • Wipe code - duress password triggers destruction                         │
│  • Dead man's switch (planned)                                              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Plausible Deniability System

The core security concept: **Your device looks like a NORMAL development machine.** The messenger is COMPLETELY INVISIBLE until activated with a secret trigger that only you know.

### What an Adversary Sees

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  user@uconsole:~$ ls -la                                                    │
│  total 48                                                                   │
│  drwxr-xr-x 12 user user 4096 Jan  5 22:00 .                                │
│  drwxr-xr-x  3 root root 4096 Dec 15 10:00 ..                               │
│  -rw-r--r--  1 user user  220 Dec 15 10:00 .bash_logout                     │
│  -rw-r--r--  1 user user 3526 Dec 15 10:00 .bashrc                          │
│  drwxr-xr-x  8 user user 4096 Jan  5 21:00 projects                         │
│  drwxr-xr-x  3 user user 4096 Jan  3 14:00 scripts                          │
│  user@uconsole:~$ htop                                                      │
│  [Normal processes - no simplex visible]                                    │
│  user@uconsole:~$ _                                                         │
│                                                                             │
│              >>> LOOKS LIKE A NORMAL DEVELOPMENT MACHINE <<<                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**What's hidden:**
- No messenger binary visible in PATH
- No `.simplex` directory (encrypted/hidden)
- No suspicious processes in `ps` or `htop`
- Normal bash history with developer commands
- Device boots directly to SSH terminal

### Secret Activation System (Planned v0.2.0)

The messenger only appears when you enter **YOUR** secret formula. You define this during initial setup:

| Method | Example | What It Looks Like |
|--------|---------|-------------------|
| **Hidden Command** | `cd /tmp && ls -la \| grep -v total` | Normal directory navigation |
| **Typo Sequence** | `sl` `sl` `sl` (three times) | User can't type properly |
| **Git Command** | `git status --porcelain=v2 --branch` | Normal dev workflow |
| **Environment Var** | `export DEBUG_LEVEL=7 && ./build.sh` | Debugging a build |
| **Keyboard Shortcut** | `Ctrl+Shift+Alt+M` | No visible command |

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  ═══════════════════════════════════════════════════════════════════════    │
│      YOU DEFINE YOUR OWN TRIGGER DURING SETUP - ONLY YOU KNOW IT            │
│  ═══════════════════════════════════════════════════════════════════════    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Activation Flow

```
   Device Boot
       │
       ▼
   ┌─────────────────────────────────────────────────────────────────────┐
   │                    NORMAL SSH TERMINAL                              │
   │              (auto-login, looks like dev machine)                   │
   └───────────────────────────┬─────────────────────────────────────────┘
                               │
                               │ User enters SECRET TRIGGER
                               ▼
   ┌─────────────────────────────────────────────────────────────────────┐
   │                     SIMPLEX TUI LAUNCHES                            │
   │              (full-screen, replaces terminal)                       │
   └───────────────────────────┬─────────────────────────────────────────┘
                               │
                               │ User presses F12 → PANIC → [4] SSH
                               ▼
   ┌─────────────────────────────────────────────────────────────────────┐
   │                INSTANT SWITCH TO SSH TERMINAL                       │
   │           (messenger vanishes, normal prompt returns)               │
   └─────────────────────────────────────────────────────────────────────┘
```

---

## Panic System (F12)

The PANIC system provides **graduated emergency response options**. Press `F12` at any time to access:

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                              ⚠️  PANIC MODE                               ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║   [1] LOCK    - Immediately lock screen, require password                 ║
║                 Use when: Someone approaches unexpectedly                 ║
║                 Action: Screen blanks, password prompt appears            ║
║                 Response time: < 100ms                                    ║
║                                                                           ║
║   [2] CLOSE   - Close application, clear terminal scrollback              ║
║                 Use when: Need to hide activity quickly                   ║
║                 Action: TUI exits, bash history cleared                   ║
║                 Response time: < 200ms                                    ║
║                                                                           ║
║   [3] NUKE    - Destroy encryption keys, wipe message database            ║
║                 Use when: Device seizure is IMMINENT                      ║
║                 Action: Cryptographic destruction of all data             ║
║                 Response time: < 3 seconds                                ║
║                 ⚠️  IRREVERSIBLE - ALL DATA PERMANENTLY LOST              ║
║                                                                           ║
║   [4] SSH     - Switch to normal SSH terminal (STEALTH MODE)              ║
║                 Use when: Someone is watching/approaching                 ║
║                 Action: Messenger vanishes, fake prompt appears           ║
║                 Response time: < 100ms                                    ║
║                 Result: Device looks like normal dev machine              ║
║                                                                           ║
║   [Esc] Cancel - Return to messenger                                      ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

### SSH Stealth Mode Details

When you press `F12` → `[4] SSH`:

1. Messenger window **INSTANTLY** disappears
2. Terminal scrollback is cleared
3. A **fake command history** is displayed:

```
user@uconsole:~/projects$ make clean
rm -rf build/
user@uconsole:~/projects$ make -j4
[  5%] Building C object src/main.c.o
[ 15%] Building C object src/utils.c.o
^C
user@uconsole:~/projects$ _
```

4. Normal bash prompt ready for commands
5. `ps`/`htop` shows **NO** simplex processes
6. Re-enter secret trigger to return to messenger

**Adversary sees:** Developer who just interrupted a build  
**Reality:** Messenger is backgrounded, ready to resume

---

## Wipe Code System

**Planned for v0.2.0**

Duress password that appears to unlock normally but **destroys all data**:

### Use Case: Coerced Access

> You are forced to provide your password at gunpoint, border crossing, or during detention.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  NORMAL PASSWORD:     "correct-horse-battery-staple"                        │
│                       → Unlocks system normally                             │
│                       → Full access to messages                             │
│                                                                             │
│  WIPE CODE:           "correct-horse-battery-staple!"                       │
│                       ↑ Note the exclamation mark (you choose the suffix)   │
│                                                                             │
│                       → Appears to unlock normally                          │
│                       → Shows fake/empty message history                    │
│                       → Silently triggers LUKS re-encryption                │
│                       → Original data becomes UNRECOVERABLE                 │
│                       → Adversary believes they have full access            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Technical Implementation

**LUKS Volume Configuration:**
- Keyslot 0: Normal password → Decrypts real data
- Keyslot 1: Wipe code → Triggers destruction sequence

**Destruction Sequence (< 5 seconds):**
1. Destroy Keyslot 0 (normal password)
2. Generate new random master key
3. Re-encrypt volume header with random key
4. Discard random key (never stored)
5. Mount decoy volume with fake data
6. Original data is now **CRYPTOGRAPHICALLY DESTROYED**

**What adversary sees:**
- Device unlocks normally
- Empty or decoy messages visible
- No indication that destruction occurred
- Forensic analysis finds only encrypted garbage

**What happens to your data:**
- Without the original master key, data is **GONE FOREVER**
- Even you cannot recover it
- This is by design - true plausible deniability

---

## Hidden Volume Support

**Planned for v0.3.0**

Alternative to destruction: maintain **two separate encrypted volumes** (VeraCrypt-style):

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       OUTER VOLUME (Decoy)                                  │
│                       Password: "summer-vacation-2024"                      │
│                                                                             │
│  Contains: Family photos, recipes, normal documents                         │
│  Purpose: Satisfy adversary's demand for access                             │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    HIDDEN VOLUME (Real)                             │    │
│  │                    Password: "correct-horse-battery-staple"         │    │
│  │                                                                     │    │
│  │  Contains: SimpleX database, real messages, sensitive data          │    │
│  │  Location: Inside free space of outer volume                        │    │
│  │  Detection: Forensically INDISTINGUISHABLE from free space          │    │
│  │                                                                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
```

| Scenario | Action | Result |
|----------|--------|--------|
| Adversary demands password | Give outer volume password | Sees decoy data |
| Adversary satisfied | Hidden volume untouched | Real data safe |
| Forensic analysis | Cannot detect hidden volume | Appears as free space |

---

## Systemd Sandboxing

**Planned for v0.2.0**

We use **native systemd sandboxing** instead of Firejail/AppArmor:

### Why Systemd Sandboxing?

| Feature | Firejail/AppArmor | Systemd Sandboxing |
|---------|-------------------|-------------------|
| Extra Software | ✗ Required | ✓ Built-in |
| Kernel Integration | Userspace | Native kernel namespaces |
| Attack Surface | Larger | Minimal |
| Configuration | Separate profile files | Single unit file |
| Maintenance | Extra updates needed | Comes with systemd |
| Compatibility | May conflict | Always works |

### Service Unit File

```ini
# /etc/systemd/system/simplex-tui.service

[Unit]
Description=SimpleX TUI Secure Messenger
After=network-online.target tor.service
Wants=network-online.target

[Service]
Type=simple
User=simplex
Group=simplex
ExecStart=/opt/simplex-tui/simplex-tui
Restart=on-failure
RestartSec=5

# ═══════════════════════════════════════════════════════════════════════════
#                         SECURITY SANDBOXING
# ═══════════════════════════════════════════════════════════════════════════

# Filesystem isolation
ProtectSystem=strict
ProtectHome=yes
PrivateTmp=yes
ReadWritePaths=/home/simplex/.simplex

# Network isolation (only localhost for simplex-chat WebSocket)
PrivateNetwork=no
RestrictAddressFamilies=AF_UNIX AF_INET AF_INET6

# Device access (none needed)
PrivateDevices=yes
DevicePolicy=closed

# Kernel protection
ProtectKernelTunables=yes
ProtectKernelModules=yes
ProtectKernelLogs=yes
ProtectControlGroups=yes
ProtectHostname=yes
ProtectClock=yes

# Process isolation
NoNewPrivileges=yes
PrivateUsers=yes

# System call filtering
SystemCallArchitectures=native
SystemCallFilter=@system-service
SystemCallFilter=~@privileged @resources @mount @swap @reboot

# Memory protection
MemoryDenyWriteExecute=yes
LockPersonality=yes
RestrictRealtime=yes
RestrictSUIDSGID=yes

# Capability dropping (no capabilities needed)
CapabilityBoundingSet=
AmbientCapabilities=

# Resource limits
MemoryMax=256M
TasksMax=32

[Install]
WantedBy=multi-user.target
```

### What This Blocks

| Attack Vector | Protection |
|---------------|-----------|
| Filesystem access | Only `~/.simplex` writable |
| Device access | No device nodes accessible |
| Kernel modification | Completely blocked |
| Privilege escalation | NoNewPrivileges, no capabilities |
| Memory exploits | W^X enforced |
| Process injection | Isolated namespaces |
| Resource exhaustion | Strict limits |

---

## USB Port Lockdown

**Planned for v0.2.0**

Disable USB ports at **kernel level** to prevent:
- BadUSB / USB Rubber Ducky attacks
- Evil Maid USB attacks
- Unknown device exploitation
- Data exfiltration via USB

### Kernel Parameter Method (Recommended)

```bash
# /etc/default/grub
GRUB_CMDLINE_LINUX="usbcore.authorized_default=0"

# Apply changes
sudo update-grub
sudo reboot
```

### Runtime Disabling

```bash
# Disable USB autodetection
echo 0 | sudo tee /sys/bus/usb/drivers_autoprobe

# Deauthorize all USB devices
for dev in /sys/bus/usb/devices/*/authorized; do
    echo 0 | sudo tee "$dev" 2>/dev/null
done
```

### Module Blacklisting

```bash
# /etc/modprobe.d/usb-blacklist.conf

# Block USB storage (flash drives, external HDDs)
blacklist usb-storage
blacklist uas

# Block USB input (keyboards, mice - if using built-in only)
blacklist usbhid

# Block USB network adapters
blacklist cdc_ether
blacklist rndis_host

# Block USB serial (potential modem attacks)
blacklist usbserial
blacklist ftdi_sio
blacklist ch341
blacklist cp210x
```

### Selective USB Allowlist

For devices that need specific USB devices (e.g., uConsole keyboard):

```bash
# /etc/udev/rules.d/99-usb-whitelist.rules

# Default: deny all USB
ACTION=="add", SUBSYSTEM=="usb", ATTR{authorized}="0"

# Allow only internal keyboard (by vendor:product ID)
ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="1234", ATTR{idProduct}=="5678", ATTR{authorized}="1"
```

### What This Blocks

| Attack | Without Protection | With USB Lockdown |
|--------|-------------------|-------------------|
| BadUSB/Rubber Ducky | Full system access | Device ignored |
| Evil Maid keylogger | Captures passwords | Device rejected |
| USB drive malware | Auto-executes | Drive not mounted |
| USB network adapter | MITM possible | Adapter disabled |
| Data exfiltration | Copy to USB drive | USB storage blocked |

---

## Kernel Hardening

**Planned for v0.2.0**

### Sysctl Parameters

```bash
# /etc/sysctl.d/99-sentinelx-hardening.conf

# ═══════════════════════════════════════════════════════════════════════════
#                         KERNEL POINTER PROTECTION
# ═══════════════════════════════════════════════════════════════════════════

# Hide kernel pointers from unprivileged users
kernel.kptr_restrict=2

# Restrict kernel log access
kernel.dmesg_restrict=1

# ═══════════════════════════════════════════════════════════════════════════
#                         PROCESS PROTECTION
# ═══════════════════════════════════════════════════════════════════════════

# Disable magic SysRq key (prevents keyboard-based attacks)
kernel.sysrq=0

# Maximum ptrace restriction (prevents process memory inspection)
kernel.yama.ptrace_scope=3

# Disable unprivileged BPF (prevents kernel exploits)
kernel.unprivileged_bpf_disabled=1

# Restrict unprivileged user namespaces
kernel.unprivileged_userns_clone=0

# ═══════════════════════════════════════════════════════════════════════════
#                         NETWORK HARDENING
# ═══════════════════════════════════════════════════════════════════════════

# SYN flood protection
net.ipv4.tcp_syncookies=1

# Reverse path filtering (prevents IP spoofing)
net.ipv4.conf.all.rp_filter=1
net.ipv4.conf.default.rp_filter=1

# Disable ICMP redirects (prevents MITM)
net.ipv4.conf.all.accept_redirects=0
net.ipv4.conf.default.accept_redirects=0
net.ipv6.conf.all.accept_redirects=0
net.ipv6.conf.default.accept_redirects=0

# Don't send ICMP redirects
net.ipv4.conf.all.send_redirects=0
net.ipv4.conf.default.send_redirects=0

# Ignore ICMP echo requests (ping)
net.ipv4.icmp_echo_ignore_all=1

# ═══════════════════════════════════════════════════════════════════════════
#                         MEMORY PROTECTION
# ═══════════════════════════════════════════════════════════════════════════

# Disable core dumps (prevents memory forensics)
fs.suid_dumpable=0

# Disable core dump via kernel pattern
kernel.core_pattern=|/bin/false

# Minimum mmap address (prevents null pointer exploits)
vm.mmap_min_addr=65536

# Maximum ASLR (Address Space Layout Randomization)
kernel.randomize_va_space=2

# ═══════════════════════════════════════════════════════════════════════════
#                         MODULE PROTECTION
# ═══════════════════════════════════════════════════════════════════════════

# Restrict module loading after boot (ENABLE WITH CAUTION)
# kernel.modules_disabled=1
```

### Apply Immediately

```bash
sudo sysctl --system
```

---

## Memory Protection

### Disable Swap (or Encrypt It)

Swap can contain sensitive data fragments:

```bash
# Option A: Disable swap completely
sudo swapoff -a
sudo sed -i '/swap/d' /etc/fstab

# Option B: Encrypted swap (random key each boot)
# /etc/crypttab
swap    /dev/sdX    /dev/urandom    swap,cipher=aes-xts-plain64,size=256

# /etc/fstab
/dev/mapper/swap    none    swap    sw    0    0
```

### Memory Locking

SimpleX TUI uses `mlock()` to prevent sensitive data from being swapped:

```rust
// Planned implementation
use libc::{mlockall, MCL_CURRENT, MCL_FUTURE};

unsafe {
    mlockall(MCL_CURRENT | MCL_FUTURE);
}
```

### Secure Memory Deallocation

```rust
// Zero memory before deallocation
fn secure_zero<T>(data: &mut T) {
    unsafe {
        std::ptr::write_volatile(data as *mut T, std::mem::zeroed());
    }
}
```

---

## Prerequisites

| Requirement | Version | Notes |
|-------------|---------|-------|
| **Rust** | 1.75+ | For building from source |
| **simplex-chat** | Latest | CLI with WebSocket API |
| **Tor** | Latest | For .onion server connections |
| **Linux** | Kernel 5.x+ | ARM64 or x86_64 |

### simplex-chat Setup

SimpleX TUI connects to `simplex-chat` running in WebSocket mode:

```bash
# Start simplex-chat with WebSocket API
simplex-chat -p 5225

# TUI connects to ws://127.0.0.1:5225
```

---

## Quick Start

### Option A: Pre-Built Binary (Recommended)

```bash
# Download latest release
wget https://github.com/cannatoshi/simplex-tui/releases/download/v0.1.0-alpha/simplex-tui-linux-arm64
chmod +x simplex-tui-linux-arm64

# Start simplex-chat in Terminal 1
simplex-chat -p 5225

# Start TUI in Terminal 2
./simplex-tui-linux-arm64
```

### Option B: Build from Source

```bash
# Clone repository
git clone https://github.com/cannatoshi/simplex-tui.git
cd simplex-tui

# Build release binary
cargo build --release

# Start simplex-chat in Terminal 1
simplex-chat -p 5225

# Start TUI in Terminal 2
./target/release/simplex-tui
```

---

## Building from Source

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Clone and Build

```bash
git clone https://github.com/cannatoshi/simplex-tui.git
cd simplex-tui
cargo build --release
```

### Cross-Compile for ARM64 (from x86_64)

```bash
# Add ARM64 target
rustup target add aarch64-unknown-linux-gnu

# Install cross-compiler
sudo apt install gcc-aarch64-linux-gnu

# Build for ARM64
cargo build --release --target aarch64-unknown-linux-gnu
```

---

## Basic Operation

### Starting SimpleX TUI

**Terminal 1:** Start SimpleX Chat WebSocket server:
```bash
simplex-chat -p 5225
```

**Terminal 2:** Start the TUI:
```bash
./simplex-tui
```

### Interface Overview

```
┌────────────────────────┬─────────────────────────────────────────────────────┐
│                        │                                                     │
│  CONTACT LIST          │  MESSAGE VIEW                                       │
│  ─────────────         │  ────────────                                       │
│                        │                                                     │
│  ▸ Contact 1 (3)       │  Contact 1  14:32                                   │
│    Contact 2           │  Hey, are you there?                                │
│    Contact 3 (1)       │                                                     │
│                        │  You  14:33                                         │
│  (3) = unread count    │  Yes, what's up?                                    │
│                        │                                                     │
│                        │  Contact 1  14:35                                   │
│                        │  Check the latest news                              │
│                        │                                                     │
├────────────────────────┼─────────────────────────────────────────────────────┤
│ [i] Add [r] Refresh    │  Type message...                                    │
│ [?] Help               │                                                     │
├────────────────────────┴─────────────────────────────────────────────────────┤
│ ● Connected - Real-time!                                                     │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Keyboard Shortcuts

### Normal Mode

| Key | Action |
|-----|--------|
| `Tab` | Cycle panels (Contacts → Chat → Input) |
| `j` / `↓` | Navigate down / Scroll messages down |
| `k` / `↑` | Navigate up / Scroll messages up |
| `Enter` | Select contact / Enter input mode |
| `r` | Refresh contacts and chat history |
| `?` | Toggle help modal |
| `q` | Quit application |
| `F12` | **PANIC MODE** |

### Input Mode

| Key | Action |
|-----|--------|
| `Enter` | Send message |
| `Esc` | Exit input mode |
| `Tab` | Switch to next panel |
| `Backspace` | Delete character |

### Panic Mode (F12)

| Key | Action |
|-----|--------|
| `1` | LOCK - Lock screen |
| `2` | CLOSE - Exit and clear history |
| `3` | NUKE - Destroy all data |
| `4` | SSH - Switch to stealth terminal |
| `Esc` | Cancel - Return to messenger |

---

## Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                      SIMPLEX TUI (This Project)                     │   │
│   │   ┌─────────────────────────────────────────────────────────────┐   │   │
│   │   │  UI Thread (Main)              │  WebSocket Thread (Tokio)  │   │   │
│   │   │  ─────────────────             │  ────────────────────────  │   │   │
│   │   │  • Ratatui rendering           │  • Async connection mgmt   │   │   │
│   │   │  • Keyboard input              │  • Command serialization   │   │   │
│   │   │  • State management            │  • Response parsing        │   │   │
│   │   │  • 50ms refresh loop           │  • Push event detection    │   │   │
│   │   └──────────────┬──────────────────────────────┬───────────────┘   │   │
│   │                  │      mpsc::channel           │                   │   │
│   │                  │    (thread-safe comm)        │                   │   │
│   │                  └──────────────────────────────┘                   │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                      │                                      │
│                                      │ WebSocket ws://127.0.0.1:5225        │
│                                      ▼                                      │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                      SIMPLEX-CHAT CLI (-p 5225)                     │   │
│   │   • Haskell-based SimpleX Chat client                               │   │
│   │   • WebSocket API mode (no terminal UI)                             │   │
│   │   • Manages cryptographic keys                                      │   │
│   │   • Handles SMP protocol                                            │   │
│   │   • Database: ~/.simplex/simplex_v1_chat.db                         │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                      │                                      │
│                                      │ Tor SOCKS5 (if .onion servers)       │
│                                      ▼                                      │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                         TOR DAEMON (9050)                           │   │
│   │   • Routes all SMP traffic through Tor network                      │   │
│   │   • Provides anonymity and censorship resistance                    │   │
│   │   • Connects to .onion SMP/XFTP servers                             │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                      │                                      │
│                                      ▼                                      │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │                    YOUR SMP/XFTP SERVERS                            │   │
│   │   • Self-hosted Tor v3 hidden services                              │   │
│   │   • No public IP addresses                                          │   │
│   │   • Zero-knowledge message routing                                  │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### WebSocket Event Flow

```
COMMAND FLOW (User sends message):
┌──────────────┐    ┌───────────────┐    ┌──────────────┐
│   UI Thread  │───▶│ mpsc::channel │───▶│  WS Thread   │
│              │    │               │    │              │
│ app.send_msg │    │ cmd: "@user   │    │ ws.send()    │
│              │    │      hello"   │    │              │
└──────────────┘    └───────────────┘    └──────────────┘

RESPONSE FLOW (Message received):
┌──────────────┐    ┌───────────────┐    ┌──────────────┐
│  WS Thread   │───▶│ mpsc::channel │───▶│   UI Thread  │
│              │    │               │    │              │
│ ws.recv()    │    │ SimplexEvent  │    │ app.messages │
│ parse_json() │    │ ::NewMessage  │    │   .push()    │
└──────────────┘    └───────────────┘    └──────────────┘
```

### Event Types

| Event | Source | Description |
|-------|--------|-------------|
| `contactsList` | Response | List of contacts |
| `chatItems` | Response | Chat history |
| `newChatItems` | **Push** | Real-time incoming message |

**Key Insight:** Push events have **no `corrId` field** - this is how we detect real-time messages vs command responses.

---

## Roadmap

### ✅ v0.1.0-alpha (January 2026) - COMPLETED

- [x] Real-time WebSocket messaging
- [x] Contact list and selection
- [x] Chat history loading
- [x] Message sending
- [x] Auto-scroll to new messages
- [x] F12 Panic button (UI)
- [x] Basic color scheme
- [x] Help modal

### 🚧 v0.2.0 - Stealth & Security (Q1 2026)

- [ ] **Stealth Mode**
  - [ ] Secret activation trigger system
  - [ ] Auto-start to SSH terminal
  - [ ] Fake command history on SSH switch
  - [ ] Process hiding

- [ ] **Wipe Code System**
  - [ ] LUKS keyslot integration
  - [ ] Duress password detection
  - [ ] Silent data destruction
  - [ ] Decoy volume mounting

- [ ] **Systemd Sandboxing**
  - [ ] Service unit file
  - [ ] Namespace isolation
  - [ ] Capability dropping
  - [ ] Syscall filtering

- [ ] **USB Port Lockdown**
  - [ ] Kernel parameter configuration
  - [ ] Module blacklisting
  - [ ] Selective allowlist support

- [ ] **Kernel Hardening**
  - [ ] Sysctl hardening parameters
  - [ ] Memory protection
  - [ ] Core dump prevention

### 📋 v0.3.0 - Advanced Security (Q2 2026)

- [ ] **Hidden Volume Support**
  - [ ] Outer/inner volume architecture
  - [ ] Plausible deniability for storage
  - [ ] Automatic decoy content

- [ ] **Dead Man's Switch**
  - [ ] Configurable inactivity timeout
  - [ ] Automatic data destruction
  - [ ] Network heartbeat option

- [ ] **Group Chat Support**
  - [ ] Group message parsing
  - [ ] Member list display
  - [ ] Group notifications

### 📋 v0.4.0 - Hardware Integration (Q3 2026)

- [ ] **Hardware Security Module**
  - [ ] YubiKey integration
  - [ ] Hardware key storage
  - [ ] 2FA for activation

- [ ] **Clockwork Pi Optimization**
  - [ ] Screen dimming controls
  - [ ] Battery status display
  - [ ] LTE indicator
  - [ ] Hardware button mapping

- [ ] **File Transfer**
  - [ ] XFTP integration
  - [ ] Encrypted file sending
  - [ ] Progress indicator

### 📋 Future

- [ ] Multiple chat tabs
- [ ] Message search
- [ ] Contact management (add/remove)
- [ ] Notification sounds (optional)
- [ ] Localization (i18n)
- [ ] Plugin system

---

## Contributing

Contributions are welcome! This is a security-critical project, so please:

1. **Security First** - All PRs reviewed for security implications
2. **Minimal Dependencies** - Avoid adding new crates unless necessary
3. **Documentation** - Update docs for any new features
4. **Testing** - Add tests for security-critical code

### Priority Areas

| Area | Difficulty | Impact |
|------|------------|--------|
| Stealth mode implementation | Hard | Critical |
| Systemd service file | Medium | High |
| USB lockdown documentation | Easy | High |
| Wipe code system | Hard | Critical |
| Test coverage | Medium | Medium |

---

## Related Projects

- **[SimpleX Chat](https://github.com/simplex-chat/simplex-chat)** - The SimpleX Chat application
- **[SimpleX SMP Monitor](https://github.com/cannatoshi/simplex-smp-monitor)** - Server monitoring & testing
- **[SimpleX Private Infrastructure](https://github.com/cannatoshi/simplex-smp-xftp-via-tor-on-rpi-hardened)** - Tor-only server setup guide

---

## License

This project is licensed under the **GNU Affero General Public License v3.0** (AGPL-3.0).

See [LICENSE](LICENSE) for the full license text.

---

## Disclaimer

This software is provided "AS IS" without warranty of any kind. The authors are not responsible for any damages or issues arising from its use.

**This tool is designed for legitimate privacy protection.** Do not use it for illegal activities.

> **Note:** This project is **not affiliated with or endorsed by SimpleX Chat Ltd**.

---

## Acknowledgments

- **SimpleX Team** - For creating the most private messenger
- **Ratatui** - Excellent TUI framework for Rust
- **Tokio** - Async runtime that makes WebSocket handling elegant
- **The Privacy Community** - For feedback and testing

---

<p align="center">
  <sub>Built for those who need privacy most: journalists, activists, whistleblowers.</sub><br>
  <sub>Part of the SentinelX Security Suite.</sub>
</p>

<p align="center">
  <sub><strong>In Code We Trust</strong></sub>
</p>
