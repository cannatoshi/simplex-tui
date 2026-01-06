# Legal Information / Rechtliche Informationen

> **SimpleX TUI** - Legal Documentation Overview

---

## English

### Legal Documents

This project maintains the following legal documentation:

| Document | Description |
|----------|-------------|
| [LICENSE](LICENSE) | GNU Affero General Public License v3.0 (AGPL-3.0) |
| [TRADEMARK.md](TRADEMARK.md) | Trademark notice and third-party trademark information |
| [DISCLAIMER.md](DISCLAIMER.md) | Liability disclaimer and limitation of warranties |

### Quick Summary

- **License:** This software is licensed under AGPL-3.0. You may use, modify, and distribute it under the terms of this license.
- **Trademarks:** "SimpleX" and "SimpleX Chat" are trademarks of SimpleX Chat Ltd. This project is **not affiliated with or endorsed by** SimpleX Chat Ltd.
- **Liability:** This software is provided "AS IS" without warranty. See [DISCLAIMER.md](DISCLAIMER.md) for full details.

---

### Third-Party Software

This project uses the following third-party software:

#### SimpleX Chat CLI (AGPL-3.0)

SimpleX TUI requires the **SimpleX Chat CLI** as a backend. The CLI is:

| Software | Description | License | Source |
|----------|-------------|---------|--------|
| **simplex-chat** | SimpleX Chat command-line client | AGPL-3.0 | [simplex-chat](https://github.com/simplex-chat/simplex-chat) |

**Important Notes:**

1. **Separate Software:** SimpleX TUI is a frontend that communicates with SimpleX Chat CLI via WebSocket. We do not modify or redistribute SimpleX Chat CLI.

2. **Source Code Availability:** The complete source code for SimpleX Chat is available at:
   - https://github.com/simplex-chat/simplex-chat

3. **Our Contribution:** SimpleX TUI is an independent terminal interface that uses the public WebSocket API of SimpleX Chat CLI.

#### Rust Dependencies

| Component | License | Usage |
|-----------|---------|-------|
| ratatui | MIT | TUI framework |
| crossterm | MIT | Terminal backend |
| tokio | MIT | Async runtime |
| tokio-tungstenite | MIT | WebSocket client |
| serde / serde_json | MIT / Apache-2.0 | JSON serialization |

---

### Contact

For legal inquiries: [GitHub Issues](https://github.com/cannatoshi/simplex-tui/issues)

For SimpleX trademark inquiries: chat@simplex.chat

For SimpleX licensing questions: https://github.com/simplex-chat/simplex-chat/blob/stable/LICENSE

---

## Deutsch

### Rechtliche Dokumente

| Dokument | Beschreibung |
|----------|--------------|
| [LICENSE](LICENSE) | GNU Affero General Public License v3.0 (AGPL-3.0) |
| [TRADEMARK.md](TRADEMARK.md) | Markenrechtliche Hinweise |
| [DISCLAIMER.md](DISCLAIMER.md) | Haftungsausschluss |

### Kurzzusammenfassung

- **Lizenz:** AGPL-3.0
- **Markenrecht:** "SimpleX" ist eine Marke der SimpleX Chat Ltd. Dieses Projekt ist **nicht mit SimpleX Chat Ltd verbunden**.
- **Haftung:** "WIE BESEHEN" ohne Gewaehrleistung.

---

### Drittanbieter-Software

#### SimpleX Chat CLI (AGPL-3.0)

SimpleX TUI benoetigt den **SimpleX Chat CLI** als Backend:

| Software | Beschreibung | Lizenz | Quelle |
|----------|--------------|--------|--------|
| **simplex-chat** | Kommandozeilen-Client | AGPL-3.0 | [simplex-chat](https://github.com/simplex-chat/simplex-chat) |

**Wichtige Hinweise:**

1. **Separate Software:** SimpleX TUI ist ein Frontend, das ueber WebSocket mit SimpleX Chat CLI kommuniziert.

2. **Quellcode-Verfuegbarkeit:** Der komplette Quellcode ist verfuegbar unter:
   - https://github.com/simplex-chat/simplex-chat

---

### Kontakt

Rechtliche Anfragen: [GitHub Issues](https://github.com/cannatoshi/simplex-tui/issues)

SimpleX Markenrecht: chat@simplex.chat

---

*Last updated: January 2026*
