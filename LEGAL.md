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
- **Radio Features:** Radio transmission features (planned v0.5.0+) are subject to local regulations. Users are solely responsible for compliance.

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

#### Reticulum Network Stack (MIT) - Planned v0.5.0+

The optional Reticulum integration for mesh network interoperability:

| Software | Description | License | Source |
|----------|-------------|---------|--------|
| **Reticulum** | Cryptography-based networking stack | MIT | [Reticulum](https://github.com/markqvist/Reticulum) |
| **LXMF** | Lightweight Extensible Message Format | MIT | [LXMF](https://github.com/markqvist/LXMF) |

**Important Notes:**

1. **Optional Dependency:** Reticulum is only required for mesh network interoperability mode. The custom protocol mode does not require Reticulum.

2. **Separate Project:** Reticulum is developed independently by Mark Qvist. We are not affiliated with the Reticulum project.

3. **License Compatibility:** MIT license is compatible with AGPL-3.0.

#### Rust Dependencies (Current)

| Component | License | Usage |
|-----------|---------|-------|
| ratatui | MIT | TUI framework |
| crossterm | MIT | Terminal backend |
| tokio | MIT | Async runtime |
| tokio-tungstenite | MIT | WebSocket client |
| serde / serde_json | MIT / Apache-2.0 | JSON serialization |

#### Rust Dependencies (Planned v0.5.0+ Radio Transport)

| Component | License | Usage |
|-----------|---------|-------|
| serialport | MPL-2.0 | Serial communication with LoRa modules |
| x25519-dalek | BSD-3-Clause | X25519 ECDH key exchange |
| chacha20poly1305 | MIT / Apache-2.0 | Authenticated encryption |
| ed25519-dalek | BSD-3-Clause | Ed25519 signatures |
| hkdf | MIT / Apache-2.0 | Key derivation function |
| rand | MIT / Apache-2.0 | Cryptographic random number generation |

**License Compatibility Notes:**

- All listed licenses are compatible with AGPL-3.0
- BSD-3-Clause, MIT, and Apache-2.0 are permissive licenses
- MPL-2.0 (serialport) is a weak copyleft license compatible with AGPL-3.0

---

### Radio Transmission Legal Notice

#### Regulatory Framework

Radio transmission is regulated by national and international law. The following is provided for informational purposes only:

| Jurisdiction | Primary Regulation | Authority |
|--------------|-------------------|-----------|
| International | ITU Radio Regulations | ITU |
| European Union | Radio Equipment Directive 2014/53/EU | EU Commission |
| Germany | Telekommunikationsgesetz (TKG) | BNetzA |
| United States | 47 CFR Part 15 | FCC |
| United Kingdom | Wireless Telegraphy Act 2006 | Ofcom |

#### User Responsibility

By using radio transmission features, users agree that:

1. They have researched and understand applicable regulations
2. They will only transmit on frequencies they are authorized to use
3. They will comply with power limits and duty cycle restrictions
4. They accept all legal responsibility for their transmissions
5. They will not hold the developers liable for regulatory violations

See [DISCLAIMER.md](DISCLAIMER.md) for complete radio transmission disclaimers.

---

### Contact

For legal inquiries: [GitHub Issues](https://github.com/cannatoshi/simplex-tui/issues)

For SimpleX trademark inquiries: chat@simplex.chat

For SimpleX licensing questions: https://github.com/simplex-chat/simplex-chat/blob/stable/LICENSE

For Reticulum licensing questions: https://github.com/markqvist/Reticulum/blob/master/LICENSE

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
- **Funkfunktionen:** Funkuebertragungsfunktionen (geplant v0.5.0+) unterliegen lokalen Vorschriften. Nutzer sind allein fuer die Einhaltung verantwortlich.

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

#### Reticulum Network Stack (MIT) - Geplant v0.5.0+

Die optionale Reticulum-Integration fuer Mesh-Netzwerk-Interoperabilitaet:

| Software | Beschreibung | Lizenz | Quelle |
|----------|--------------|--------|--------|
| **Reticulum** | Kryptographie-basierter Netzwerk-Stack | MIT | [Reticulum](https://github.com/markqvist/Reticulum) |
| **LXMF** | Leichtgewichtiges erweiterbares Nachrichtenformat | MIT | [LXMF](https://github.com/markqvist/LXMF) |

**Wichtige Hinweise:**

1. **Optionale Abhaengigkeit:** Reticulum wird nur fuer den Mesh-Netzwerk-Interoperabilitaetsmodus benoetigt.

2. **Separates Projekt:** Reticulum wird unabhaengig von Mark Qvist entwickelt. Wir sind nicht mit dem Reticulum-Projekt verbunden.

#### Rust-Abhaengigkeiten (Aktuell)

| Komponente | Lizenz | Verwendung |
|------------|--------|------------|
| ratatui | MIT | TUI-Framework |
| crossterm | MIT | Terminal-Backend |
| tokio | MIT | Async-Runtime |
| tokio-tungstenite | MIT | WebSocket-Client |
| serde / serde_json | MIT / Apache-2.0 | JSON-Serialisierung |

#### Rust-Abhaengigkeiten (Geplant v0.5.0+ Funktransport)

| Komponente | Lizenz | Verwendung |
|------------|--------|------------|
| serialport | MPL-2.0 | Serielle Kommunikation mit LoRa-Modulen |
| x25519-dalek | BSD-3-Clause | X25519 ECDH Schluesselaustausch |
| chacha20poly1305 | MIT / Apache-2.0 | Authentifizierte Verschluesselung |
| ed25519-dalek | BSD-3-Clause | Ed25519 Signaturen |
| hkdf | MIT / Apache-2.0 | Schluesselableitungsfunktion |
| rand | MIT / Apache-2.0 | Kryptographische Zufallszahlengenerierung |

---

### Rechtlicher Hinweis zur Funkuebertragung

#### Regulatorischer Rahmen

Funkuebertragung wird durch nationales und internationales Recht geregelt:

| Rechtsraum | Primaere Regulierung | Behoerde |
|------------|---------------------|----------|
| International | ITU-Funkvorschriften | ITU |
| Europaeische Union | Funkanlagenrichtlinie 2014/53/EU | EU-Kommission |
| Deutschland | Telekommunikationsgesetz (TKG) | BNetzA |
| Oesterreich | Telekommunikationsgesetz (TKG 2021) | RTR |
| Schweiz | Fernmeldegesetz (FMG) | BAKOM |

#### Verantwortung des Nutzers

Durch die Nutzung von Funkuebertragungsfunktionen stimmen Nutzer zu, dass:

1. Sie die geltenden Vorschriften recherchiert haben und verstehen
2. Sie nur auf Frequenzen senden, zu deren Nutzung sie berechtigt sind
3. Sie Leistungsgrenzen und Tastverhaeltnis-Beschraenkungen einhalten
4. Sie alle rechtliche Verantwortung fuer ihre Uebertragungen uebernehmen
5. Sie die Entwickler nicht fuer Regulierungsverstoesse haftbar machen

Siehe [DISCLAIMER.md](DISCLAIMER.md) fuer vollstaendige Haftungsausschluesse zur Funkuebertragung.

---

### Kontakt

Rechtliche Anfragen: [GitHub Issues](https://github.com/cannatoshi/simplex-tui/issues)

SimpleX Markenrecht: chat@simplex.chat

SimpleX Lizenzfragen: https://github.com/simplex-chat/simplex-chat/blob/stable/LICENSE

Reticulum Lizenzfragen: https://github.com/markqvist/Reticulum/blob/master/LICENSE

---

*Last updated: January 2026*
