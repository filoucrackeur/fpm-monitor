<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | **Deutsch**

[![CI](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml) [![Release](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/php-fpm-monitor?sort=semver)](https://github.com/filoucrackeur/php-fpm-monitor/releases) [![codecov](https://img.shields.io/codecov/c/github/filoucrackeur/php-fpm-monitor)](https://codecov.io/github/filoucrackeur/php-fpm-monitor) [![Maintainability](https://img.shields.io/codeclimate/maintainability/filoucrackeur/php-fpm-monitor)](https://codeclimate.com/github/filoucrackeur/php-fpm-monitor) [![License](https://img.shields.io/github/license/filoucrackeur/php-fpm-monitor)](LICENSE) ![Static](https://img.shields.io/badge/static-musl-blue) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/php-fpm-monitor)](https://github.com/filoucrackeur/php-fpm-monitor/commits/main)

PHP-FPM-Pool-Monitor in Rust (ein Port von `fpm-monitor.c`) mit einem interaktiven
Terminal-Dashboard, das von [Ember](https://github.com/alexandre-daubois/ember)
inspiriert ist. Die Oberfläche ist über `--lang` in 8 Sprachen verfügbar
(Standard: Englisch).

<p align="center">
  <img src="docs/screenshot.de.animated.svg" alt="fpm-monitor TUI interaktives Dashboard (Deutsch)" width="700">
</p>

## Funktionen

- **CLI-Ausgabe**: Pool-Tabelle (pool, type, workers, running, idle, backlog,
  max_children, max_requests, Speicher) mit Worker-Details (`-v`: PID, Status, RSS).
- **TUI-Dashboard** (`-t`): interaktive Oberfläche mit 4 Tabs.
  - **Überwachung**: Echtzeitstatus der Pools und ihrer Worker (Speicher pro Pool).
  - **Diagramme**: ein großes Diagramm pro Pool (Ember-Stil) mit ← → Untertabs:
    `running`, `workers`, `idle`, `backlog` (immer sichtbar, auch wenn leer), sowie
    gestrichelte Schwellenlinien: `max_children` (grau), `max_requests` (rot),
    `min_spare` (rosa) und `max_spare` (violett) auf dem `idle`-Panel.
  - **Logs**: pro Pool die letzten Zeilen des PHP-Protokolls (`error_log`), der
    langsamen Abfragen (`slowlog`) und des **Access-Logs** (`access.log`, HTTP-Codes
    eingefärbt), in Untertabs.
  - **Konfigurationen**: aus den Dateien gelesene Direktiven (`global` + Pools) in Untertabs.
- Automatische Konfigurationserkennung (übliche Orte + `include`-Direktive).
- Keine Netzwerkaufrufe: alles wird direkt aus `/proc` gelesen (Prozesse, RSS,
  Status, Warteschlange der TCP-Socket-Annahme).

## Anforderungen

- Rust (stable) zum Kompilieren.
- **Linux mit `/proc`** für echte Daten. Keine FPM-Statuskonfiguration
  (`pm.status_path`) erforderlich: Die Daten stammen aus `/proc` und der
  Konfiguration. Bei TCP-Sockets wird der Backlog aus `/proc/net/tcp`
  (Annahmewarteschlange) gelesen; bei Unix-Sockets wird nur `listen.backlog` als
  Maximum angezeigt.

## Installation

```sh
cargo build --release
# die Binärdatei liegt unter target/release/fpm-monitor
```

Für die Bereitstellung in einem `php:*-fpm-alpine`-Container (musl, aarch64):

```sh
RUSTC=$HOME/.cargo/bin/rustc ~/.cargo/bin/cargo build --release --target aarch64-unknown-linux-musl
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux-Pakete (Red Hat / Debian)

Jede [Release](https://github.com/filoucrackeur/php-fpm-monitor/releases) enthält statische (musl)
`.deb`- und `.rpm`-Pakete. Ohne externe Abhängigkeiten funktionieren sie auf
jeder aktuellen RHEL-, Fedora-, CentOS-, Debian- oder Ubuntu-Version (x86_64 und ARM64).

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor_<version>_amd64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Verwenden Sie auf ARM64 stattdessen die `arm64`-Pakete.

### macOS (Homebrew)

Jede Release erzeugt und hängt eine gebrauchsfertige Formel an:

```sh
brew install https://github.com/filoucrackeur/php-fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

Um sie als Tap zu hosten, legen Sie die Datei unter
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb` ab und führen dann aus:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

## Verwendung

```
fpm-monitor [OPTIONEN]

Optionen:
  -c, --config <PFAD>    php-fpm.conf-Datei oder Pool-Verzeichnis zur Analyse
  -v, --verbose          Worker-Details anzeigen (PID, Status, RSS)
      --color            Farbe erzwingen
      --no-color         Farbe deaktivieren
  -t, --tui              Interaktives Dashboard (Tabs, ← → Untertabs)
      --interval <SEK>   TUI-Aktualisierungsintervall (Standard 1)
      --lang <SPRACHE>   Oberflächensprache: en, fr, zh, ar, es, it, ja, de
                         (Standard: en)
      --mock             Demodaten (lokaler Test)
  -h, --help             Diese Hilfe anzeigen
```

### Oberflächensprache

`--lang` akzeptiert `en` (US, Standard), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`
(die Form `--lang=fr` wird ebenfalls akzeptiert). Dies gilt für das TUI-Dashboard
sowie für die CLI-Ausgabe (Kopfzeilen, Zusammenfassung, Legende) und `--help`.

### TUI-Dashboard

Das TUI füllt das gesamte Terminal (dynamische Größenanpassung) und bleibt
reaktiv: Die Daten werden im Hintergrund im eingestellten Intervall
(`--interval`, Standard 1 s) aktualisiert; selbst wenn eine `/proc`-Leseoperation
langsam ist, läuft die Darstellung weiter.

| Taste           | Aktion                              |
| --------------- | ----------------------------------- |
| `1` – `4`, `Tab` | Tab wechseln                      |
| `←` / `→`       | Untertabs durchsuchen (Pools/Konfig)|
| `↑` / `↓`       | Überwachungsansicht scrollen        |
| `q`, `Ctrl-C`   | Beenden                             |

## Projektstruktur

| Datei              | Rolle                                     |
| ------------------ | ----------------------------------------- |
| `src/main.rs`      | CLI, Optionen, Orchestrierung             |
| `src/config.rs`    | Konfigurationserkennung und Parsing       |
| `src/proc.rs`      | `/proc`-Lesen (Worker, RSS, Status, TCP-Backlog) |
| `src/data.rs`      | Zusammenführen der Daten zu Tabellenzeilen|
| `src/logs.rs`      | PHP-Protokoll und langsame Abfragen lesen |
| `src/render.rs`    | Textausgabe (CLI-Tabelle)                 |
| `src/tui.rs`       | Interaktives Dashboard (4 Tabs, Diagramme)|
| `src/i18n.rs`      | Lokalisierung (8 Sprachen, `--lang`)      |
| `src/term.rs`      | Farberkennung / Stile                     |

## Entwicklung

```sh
cargo build          # kompilieren (debug)
cargo test           # Unit-Tests ausführen
cargo clippy         # lint
cargo fmt --check    # Formatierung
```

## Lizenz

[MIT](LICENSE)
