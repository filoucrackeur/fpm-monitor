<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | **Svenska** | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

PHP-FPM-poolövervakare skriven i Rust (en port av `fpm-monitor.c`), med en
interaktiv terminalpanel inspirerad av [Ember](https://github.com/alexandre-daubois/ember).
Den levereras som en enda beroendefri binär och körs på **Linux, macOS och
Windows**, över **18 målarkitekturer**. Gränssnittet finns på 22 språk
(standard: engelska) via `--lang`, och den här dokumentationen är översatt till
22 språk.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — interaktiv panel (engelska)" width="700">
</p>

## Funktioner

- **CLI-utdata**: pooltabell (pool, typ, workers, running, idle, backlog,
  max_children, max_requests, minne) med detaljer per worker (`-v`: pid,
  tillstånd, RSS).
- **TUI-panel** (`-t`): interaktivt gränssnitt med 4 flikar.
  - **Övervakning**: realtidsstatus för poolerna och deras workers (minne per
    pool).
  - **Grafer**: ett stort diagram per pool (Ember-stil) med `←`/`→` underflikar:
    `running`, `workers`, `idle`, `backlog` (visas alltid, även tomma), samt
    streckade tröskellinjer: `max_children` (grå), `max_requests` (röd),
    `min_spare` (rosa) och `max_spare` (lila) i `idle`-panelen.
  - **Loggar**: per pool de senaste raderna i PHP-loggen (`error_log`),
    långsamma frågor (`slowlog`) och **åtkomstloggen** (`access.log`,
    färgkodade HTTP-statuskoder), i underflikar.
  - **Konfigurationer**: direktiv som läses från filerna (`global` + pooler) i
    underflikar.
- **Webbpanel** (`--watch`): samma fyra vyer i webbläsaren, serverade på
  `http://127.0.0.1:8471` av en HTTP-server utan beroenden. Porten ställs in med
  `--port`; data samplas i bakgrunden vid `--interval` och sidan behåller
  60 historikpunkter för diagrammen.
- Automatisk konfigurationsupptäckt (vanliga platser + `include`-direktivet).
- Inga nätverksanrop: allt läses direkt från `/proc` (processer, RSS, tillstånd,
  TCP-socketens acceptkö). På macOS och BSD läses samma data från `ps` och
  `netstat`; ingen FPM-statuskonfiguration (`pm.status_path`) behövs.
- **Portabel av design**: statiska musl-binärer för 10 Linux-arkitekturer,
  glibc-byggen för resten, plus macOS och Windows. Inga körtidsberoenden, ingen
  libc-version att matcha (förutom de dokumenterade glibc-byggena).

## Krav

- **Rust 1.78+** för att kompilera från källkoden (MSRV testas i CI).
- **Linux**: `/proc` används för verklig data. För TCP-sockets läses acceptkön
  från `/proc/net/tcp`; för unix-sockets visas endast `listen.backlog` som
  maximum.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: inget `/proc`; processer
  och sockets läses med `ps` och `netstat`.
- **Windows**: CLI:t bygger och körs (`--help`, `--mock`), men PHP-FPM finns
  inte på Windows, så live-upptäckten av processer returnerar ett explicit fel.

## Installation

### Färdigbyggda binärer

Varje [utgåva](https://github.com/filoucrackeur/fpm-monitor/releases) innehåller
råa binärer för alla mål som stöds, ett arkiv per mål (`.tar.gz` på Unix, `.zip`
på Windows), `.deb`/`.rpm`-paket och en Homebrew-formel.

Arkiven är fristående: varje arkiv innehåller den körbara filen `fpm-monitor`,
`LICENSE` och `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Eller använd den råa binären direkt:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Plattformar och arkitekturer som stöds

| Plattform | Arkitekturer | Rust-mål | Distribution |
| --------- | ------------ | -------- | ------------ |
| Linux (statisk musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | binär, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | binär, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | binär, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI-avbildning |

### Från källkoden

```sh
cargo build --release
# binären finns i target/release/fpm-monitor
```

### Korskompilering

CI använder [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) för
att producera alla Linux-binärer från en enda x86_64-runner:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# binären finns i target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Byt ut målet mot valfri trippel från tabellen ovan. Så här distribuerar du i en
`php:*-fpm-alpine`-container:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux-paket (Red Hat / Debian)

`.deb`- och `.rpm`-paket bifogas varje utgåva för amd64, arm64, armhf, armel,
i386, riscv64, ppc64el, ppc64, s390x, loong64 och sparc64. Musl-paketen har inga
externa beroenden och fungerar på alla nyare versioner av RHEL, Fedora, CentOS,
Debian eller Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- På andra arkitekturer byter du ut `amd64`/`x86_64` mot motsvarande paketnamn
  (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`, `s390x`,
  `loong64`, `sparc64`).

För att bygga om arkiven och paketen lokalt från en befintlig `target/`-katalog:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # kräver nfpm i PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

En multiark-avbildning publiceras på GitHub Container Registry för
`linux/amd64`, `linux/arm64`, `linux/ppc64le` och `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` låter övervakaren läsa PHP-FPM-processerna från värdens `/proc`;
montera din FPM-poolkonfiguration med `-v` så att poolerna hittas (de vanliga
`/etc`-sökvägarna genomsöks också).

## Användning

```
fpm-monitor [OPTIONS]

Alternativ:
  -c, --config <PATH>     php-fpm.conf-fil eller poolkatalog att analysera
  -v, --verbose           Visa detaljer för varje worker (pid, tillstånd, RSS)
      --color             Tvinga färg
      --no-color          Inaktivera färg
  -t, --tui               Interaktiv panel (flikar, `←`/`→` underflikar)
      --interval <SEC>    Uppdateringsintervall för TUI (standard 1)
      --watch             Webbpanel i stället för CLI-vyn
      --port <PORT>       Port för webbpanelen (standard 8471)
      --lang <LANG>       Gränssnittsspråk: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (standard: en)
      --mock              Demodata (lokalt test)
  -h, --help              Visa denna hjälp
```

### Gränssnittsspråk

`--lang` accepterar `en` (US, standard), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
(formen `--lang=fr` accepteras också). Det gäller både TUI-panelen och
CLI-utdata (rubriker, sammanfattning, teckenförklaring) samt `--help`.

### TUI-panel

TUI:n fyller hela terminalen (dynamisk storleksändring) och förblir responsiv:
data uppdateras i bakgrunden med det inställda intervallet (`--interval`,
standard 1 s); även om en läsning av `/proc` är långsam fortsätter renderingen.

| Tangent       | Åtgärd                             |
| ------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Byt flik                        |
| `←` / `→`     | Navigera i underflikar (pooler/konfig) |
| `↑` / `↓`     | Scrolla i övervakningsvyn          |
| `q`, `Ctrl-C` | Avsluta                            |

### Webbpanel

`--watch` serverar samma data som en ensidig webbapp i stället för att skriva ut
CLI-tabellen:

```sh
fpm-monitor --watch                 # http://127.0.0.1:8471
fpm-monitor --watch --port 9000     # egen port
fpm-monitor --watch --mock          # demodata utan PHP-FPM
```

Servern lyssnar bara på loopback, utan tredjepartsberoenden, och sidan frågar
`/api/state` efter pooler, workerdetaljer, loggar, konfigurationsdirektiv och de
senaste 60 proverna. Data samlas i bakgrunden vid `--interval` (standard 1 s), så
en långsam skanning blockerar aldrig en begäran. De fyra flikarna är
**Övervakning**, **Diagram**, **Loggar** och **Konfigurationer**; `1`–`4` och
`←`/`→` väljer flikar och underflikar.

`--port` accepterar `1`–`65535` (`--port=9000` fungerar också); andra värden
avslutas med kod `2`. Servern skickar aldrig någon utgående begäran.

## Projektstruktur

| Fil               | Roll                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, alternativ, orkestrering                       |
| `src/config.rs`   | Upptäckt och tolkning av konfiguration              |
| `src/proc.rs`     | Läsning av `/proc` (workers, RSS, tillstånd, TCP-backlog) |
| `src/data.rs`     | Sammanfogning av data till tabellrader              |
| `src/logs.rs`     | Läsning av PHP-logg och långsamma frågor            |
| `src/render.rs`   | Textutdata (CLI-tabell)                             |
| `src/tui.rs`      | Interaktiv panel (4 flikar, grafer)                 |
| `src/web.rs`       | Webbserver, bakgrundsinsamling, JSON-ögonblicksbild |
| `src/web/index.html` | Inbäddad ensidig webbpanel                       |
| `src/i18n.rs`     | Lokalisering (22 språk, `--lang`)                    |
| `src/term.rs`     | Färgdetektering / stilar                            |
| `tests/cli.rs`    | End-to-end-tester för CLI (`--help`, `--mock`, i18n) |
| `.github/workflows` | CI (tester, täckning, 18 byggen, rökprovningar i QEMU) |

## Utveckling

```sh
cargo build          # bygge (debug)
cargo test           # enhets- + integrationstester
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # formatering
```

CI kör testsviten på Linux, macOS och Windows, på MSRV (Rust 1.78), publicerar
täckningsmärket, korskompilerar de 18 utgåvemålen och kör binärerna för
främmande arkitekturer under QEMU.

## Licens

[MIT](LICENSE)
