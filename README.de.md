<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | **Deutsch** | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Ein in Rust geschriebener PHP-FPM-Pool-Monitor (ein Port von `fpm-monitor.c`) mit
einem interaktiven Terminal-Dashboard nach dem Vorbild von
[Ember](https://github.com/alexandre-daubois/ember). Er wird als einzelne,
abhängigkeitsfreie Binärdatei ausgeliefert und läuft unter **Linux, macOS und
Windows** auf **18 Zielarchitekturen**. Die Oberfläche ist über `--lang` in 22
Sprachen verfügbar (Standard: Englisch), und diese Dokumentation ist in 22
Sprachen übersetzt.

<p align="center">
  <img src="docs/screenshot.de.animated.svg" alt="fpm-monitor TUI — interaktives Dashboard (Deutsch)" width="700">
</p>

## Funktionen

- **CLI-Ausgabe**: Pool-Tabelle (pool, type, workers, running, idle, backlog,
  max_children, max_requests, Speicher) mit Details pro Worker (`-v`: PID, Status, RSS).
- **TUI-Dashboard** (`-t`): interaktive Oberfläche mit 4 Tabs.
  - **Überwachung**: Echtzeitstatus der Pools und ihrer Worker (Speicher pro Pool).
  - **Diagramme**: ein großes Diagramm pro Pool (Ember-Stil) mit `←`/`→`-Untertabs:
    `running`, `workers`, `idle`, `backlog` (immer sichtbar, auch wenn leer), dazu
    gestrichelte Schwellenwertlinien: `max_children` (grau), `max_requests` (rot),
    `min_spare` (rosa) und `max_spare` (violett) im `idle`-Panel.
  - **Protokolle**: pro Pool die neuesten Zeilen des PHP-Logs (`error_log`), der
    langsamen Abfragen (`slowlog`) und des **Access-Logs** (`access.log`, farbige
    HTTP-Codes), in Untertabs.
  - **Konfigurationen**: aus den Dateien gelesene Direktiven (`global` + Pools) in Untertabs.
- Automatische Konfigurationserkennung (übliche Orte + `include`-Direktive).
- Keine Netzwerkaufrufe: alles wird direkt aus `/proc` gelesen (Prozesse, RSS,
  Status, Annahmewarteschlange der TCP-Sockets). Unter macOS und BSD werden dieselben
  Daten aus `ps` und `netstat` gelesen; eine FPM-Statuskonfiguration (`pm.status_path`)
  ist nicht erforderlich.
- **Portabel von Grund auf**: statische musl-Binärdateien für 10 Linux-Architekturen,
  glibc-Builds für die übrigen sowie macOS und Windows. Keine Laufzeitabhängigkeit,
  keine passende libc-Version nötig (außer den dokumentierten glibc-Builds).

## Anforderungen

- **Rust 1.78+**, um aus dem Quellcode zu kompilieren (MSRV in der CI getestet).
- **Linux**: Für echte Daten wird `/proc` verwendet. Bei TCP-Sockets wird die
  Annahmewarteschlange aus `/proc/net/tcp` gelesen; bei Unix-Sockets wird nur
  `listen.backlog` als Maximum angezeigt.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: kein `/proc`; Prozesse und
  Sockets werden mit `ps` und `netstat` gelesen.
- **Windows**: Die CLI lässt sich bauen und ausführen (`--help`, `--mock`), aber
  PHP-FPM ist unter Windows nicht verfügbar, daher liefert die Live-Prozesserkennung
  einen ausdrücklichen Fehler.

## Installation

### Vorgebaute Binärdateien

Jede [Release](https://github.com/filoucrackeur/fpm-monitor/releases) enthält rohe
Binärdateien für alle unterstützten Ziele, ein Archiv pro Ziel (`.tar.gz` unter Unix,
`.zip` unter Windows), `.deb`/`.rpm`-Pakete und eine Homebrew-Formel.

Die Archive sind eigenständig: Jedes enthält die ausführbare Datei `fpm-monitor`,
`LICENSE` und `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Oder verwenden Sie direkt die rohe Binärdatei:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Unterstützte Plattformen und Architekturen

| Plattform | Architekturen | Rust-Ziele | Verteilung |
| --------- | ------------- | ---------- | ---------- |
| Linux (statisches musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | Binärdatei, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | Binärdatei, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | Binärdatei, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI-Image |

### Aus dem Quellcode

```sh
cargo build --release
# die Binärdatei liegt unter target/release/fpm-monitor
```

### Cross-Kompilierung

Die CI verwendet [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild), um
alle Linux-Binärdateien von einem einzigen x86_64-Runner aus zu erzeugen:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# die Binärdatei liegt unter target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Ersetzen Sie das Ziel durch ein beliebiges Triple aus der obigen Tabelle. Für die
Bereitstellung in einem `php:*-fpm-alpine`-Container:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux-Pakete (Red Hat / Debian)

`.deb`- und `.rpm`-Pakete sind an jede Release für amd64, arm64, armhf, armel, i386,
riscv64, ppc64el, ppc64, s390x, loong64 und sparc64 angehängt. Die musl-Pakete haben
keine externen Abhängigkeiten und funktionieren auf jeder aktuellen Version von RHEL,
Fedora, CentOS, Debian oder Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Ersetzen Sie auf anderen Architekturen `amd64`/`x86_64` durch den passenden
  Paketnamen (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`,
  `s390x`, `loong64`, `sparc64`).

Um die Archive und Pakete lokal aus einem vorhandenen `target/`-Verzeichnis neu zu
erstellen:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # benötigt nfpm im PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Ein Multi-Arch-Image wird in der GitHub Container Registry für `linux/amd64`,
`linux/arm64`, `linux/ppc64le` und `linux/riscv64` veröffentlicht:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` ermöglicht es dem Monitor, die PHP-FPM-Prozesse aus dem `/proc` des Hosts
zu lesen; binden Sie Ihre FPM-Pool-Konfiguration mit `-v` ein, damit die Pools erkannt
werden (die üblichen `/etc`-Pfade werden ebenfalls durchsucht).

## Verwendung

```
fpm-monitor [OPTIONS]

Optionen:
  -c, --config <PATH>     Zu analysierende php-fpm.conf-Datei oder Pool-Verzeichnis
  -v, --verbose           Details zu jedem Worker anzeigen (PID, Status, RSS)
      --color             Farbe erzwingen
      --no-color          Farbe deaktivieren
  -t, --tui               Interaktives Dashboard (Tabs, `←`/`→`-Untertabs)
      --interval <SEC>    Aktualisierungsintervall des TUI (Standard: 1)
      --lang <LANG>       Sprache der Oberfläche: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (Standard: en)
      --mock              Demodaten (lokaler Test)
  -h, --help              Diese Hilfe anzeigen
```

### Sprache der Oberfläche

`--lang` akzeptiert `en` (US, Standard), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
(die Form `--lang=fr` wird ebenfalls akzeptiert). Sie gilt sowohl für das
TUI-Dashboard als auch für die CLI-Ausgabe (Kopfzeilen, Zusammenfassung, Legende)
und für `--help`.

### TUI-Dashboard

Das TUI füllt das gesamte Terminal (dynamische Größenanpassung) und bleibt
reaktionsfähig: Die Daten werden im Hintergrund im konfigurierten Intervall
aktualisiert (`--interval`, standardmäßig 1 s); selbst wenn ein `/proc`-Lesezugriff
langsam ist, läuft die Darstellung weiter.

| Taste            | Aktion                                    |
| ---------------- | ----------------------------------------- |
| `1` – `4`, `Tab` | Tab wechseln                              |
| `←` / `→`        | Untertabs durchlaufen (Pools/Konfiguration) |
| `↑` / `↓`        | Ansicht „Überwachung“ scrollen            |
| `q`, `Ctrl-C`    | Beenden                                   |

## Projektstruktur

| Datei             | Rolle                                                |
| ----------------- | ---------------------------------------------------- |
| `src/main.rs`     | CLI, Optionen, Orchestrierung                        |
| `src/config.rs`   | Konfigurationserkennung und -analyse                 |
| `src/proc.rs`     | Lesen von `/proc` (Worker, RSS, Status, TCP-Backlog) |
| `src/data.rs`     | Zusammenführen der Daten zu Tabellenzeilen           |
| `src/logs.rs`     | Lesen des PHP-Logs und der langsamen Abfragen        |
| `src/render.rs`   | Textausgabe (CLI-Tabelle)                            |
| `src/tui.rs`      | Interaktives Dashboard (4 Tabs, Diagramme)           |
| `src/i18n.rs`     | Lokalisierung (22 Sprachen, `--lang`)                 |
| `src/term.rs`     | Farberkennung / Stile                                |
| `tests/cli.rs`    | End-to-End-CLI-Tests (`--help`, `--mock`, i18n)      |
| `.github/workflows` | CI (Tests, Coverage, 18 Builds, QEMU-Smoke-Tests)  |

## Entwicklung

```sh
cargo build          # Build (Debug)
cargo test           # Unit- + Integrationstests
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # Formatierung
```

Die CI führt die Testsuite unter Linux, macOS und Windows auf der MSRV (Rust 1.78)
aus, veröffentlicht das Coverage-Badge, kompiliert die 18 Release-Ziele
cross-platform und führt die Binärdateien fremder Architekturen unter QEMU aus.

## Lizenz

[MIT](LICENSE)
