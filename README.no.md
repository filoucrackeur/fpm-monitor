<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/version.json)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/downloads.json)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/issues.json)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/last-commit.json)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | **Norsk** | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

PHP-FPM-poolovervåker skrevet i Rust (en port av `fpm-monitor.c`), med et
interaktivt terminaldashbord inspirert av [Ember](https://github.com/alexandre-daubois/ember).
Den leveres som én enkelt binærfil uten avhengigheter og kjører på **Linux, macOS
og Windows**, på tvers av **18 målargitekturer**. Grensesnittet er tilgjengelig på
22 språk (standard: engelsk) via `--lang`, og denne dokumentasjonen er oversatt
til 22 språk.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — interaktivt dashbord (engelsk)" width="700">
</p>

## Funksjoner

- **CLI-utdata**: pooltabell (pool, type, workers, running, idle, backlog,
  max_children, max_requests, minne) med detaljer per worker (`-v`: pid, tilstand,
  RSS).
- **TUI-dashbord** (`-t`): interaktivt grensesnitt med 4 faner.
  - **Overvåking**: sanntidsstatus for poolene og deres workers (minne per pool).
  - **Grafer**: ett stort diagram per pool (Ember-stil) med `←`/`→`-underfaner:
    `running`, `workers`, `idle`, `backlog` (vises alltid, selv når den er tom),
    pluss stiplede terskellinjer: `max_children` (grå), `max_requests` (rød),
    `min_spare` (rosa) og `max_spare` (lilla) i `idle`-panelet.
  - **Logger**: per pool de nyeste linjene fra PHP-loggen (`error_log`), trege
    spørringer (`slowlog`) og **tilgangsloggen** (`access.log`, fargelagte
    HTTP-koder), i underfaner.
  - **Konfigurasjoner**: direktiver lest fra filene (`global` + pooler) i
    underfaner.
- **Nettdashbord** (`--watch`): de samme fire visningene i nettleseren, servert på
  `http://127.0.0.1:8471` av en HTTP-server uten avhengigheter. Porten velges med
  `--port`; data samles i bakgrunnen ved `--interval`, og siden beholder
  60 historikkpunkter til diagrammene.
- Automatisk konfigurasjonsoppdagelse (vanlige plasseringer + `include`-direktivet).
- Ingen nettverkskall: alt leses direkte fra `/proc` (prosesser, RSS, tilstand,
  TCP-socketens accept-kø). På macOS og BSD leses de samme dataene fra `ps` og
  `netstat`; ingen FPM-statuskonfigurasjon (`pm.status_path`) er nødvendig.
- **Bærbart av design**: statiske musl-binærfiler for 10 Linux-arkitekturer,
  glibc-bygg for de øvrige, i tillegg til macOS og Windows. Ingen
  kjøretidsavhengighet, ingen libc-versjon å matche (unntatt de dokumenterte
  glibc-byggene).

## Krav

- **Rust 1.78+** for å kompilere fra kildekode (MSRV testes i CI).
- **Linux**: `/proc` brukes for reelle data. For TCP-sockets leses accept-køen fra
  `/proc/net/tcp`; for unix-sockets vises bare `listen.backlog` som maksimum.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: ingen `/proc`; prosesser og
  sockets leses med `ps` og `netstat`.
- **Windows**: CLI-en bygges og kjører (`--help`, `--mock`), men PHP-FPM er ikke
  tilgjengelig på Windows, så aktiv prosessoppdagelse returnerer en eksplisitt
  feil.

## Installasjon

### Forhåndsbygde binærfiler

Hver [utgivelse](https://github.com/filoucrackeur/fpm-monitor/releases) inneholder
rå binærfiler for alle støttede mål, et arkiv per mål (`.tar.gz` på Unix, `.zip` på
Windows), `.deb`/`.rpm`-pakker og en Homebrew-formel.

Arkivene er selvstendige: hvert arkiv inneholder kjørbar fil `fpm-monitor`,
`LICENSE` og `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Eller bruk den rå binærfilen direkte:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Støttede plattformer og arkitekturer

| Plattform | Arkitekturer | Rust-mål | Distribusjon |
| -------- | ------------- | ------------ | ------------ |
| Linux (statisk musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | binær, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | binær, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | binær, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI-image |

### Fra kildekode

```sh
cargo build --release
# binærfilen ligger i target/release/fpm-monitor
```

### Krysskompilering

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) brukes av CI for å
produsere alle Linux-binærfiler fra én enkelt x86_64-runner:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# binærfilen ligger i target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Erstatt målet med hvilken som helst triple fra tabellen ovenfor. For å
distribuere inne i en `php:*-fpm-alpine`-container:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux-pakker (Red Hat / Debian)

`.deb`- og `.rpm`-pakker legges ved hver utgivelse for amd64, arm64, armhf,
armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 og sparc64. musl-pakkene
har ingen eksterne avhengigheter og fungerer på alle nyere versjoner av RHEL,
Fedora, CentOS, Debian eller Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- På andre arkitekturer erstatter du `amd64`/`x86_64` med det tilsvarende
  pakkenavnet (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`,
  `s390x`, `loong64`, `sparc64`).

For å bygge arkivene og pakkene på nytt lokalt fra en eksisterende
`target/`-katalog:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # trenger nfpm i PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Et multiarkitektur-image publiseres på GitHub Container Registry for
`linux/amd64`, `linux/arm64`, `linux/ppc64le` og `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` lar overvåkeren lese PHP-FPM-prosessene fra vertens `/proc`; monter
FPM-poolkonfigurasjonen din med `-v` slik at poolene oppdages (de vanlige
`/etc`-banene skannes også).

## Bruk

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     php-fpm.conf-fil eller poolkatalog som skal analyseres
  -v, --verbose           Vis detaljer for hver worker (pid, tilstand, RSS)
      --color             Tving farger
      --no-color          Deaktiver farger
  -t, --tui               Interaktivt dashbord (faner, `←`/`→`-underfaner)
      --interval <SEC>    Oppdateringsintervall for TUI (standard 1)
      --watch             Nettdashbord i stedet for CLI-visningen
      --port <PORT>       Port for nettdashbordet (standard 8471)
      --lang <LANG>       Grensesnittspråk: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (standard: en)
      --mock              Demodata (lokal test)
  -h, --help              Vis denne hjelpen
```

### Grensesnittspråk

`--lang` godtar `en` (US, standard), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
(også formen `--lang=fr` godtas). Det gjelder både TUI-dashbordet og CLI-utdataene
(overskrifter, sammendrag, tegnforklaring) og `--help`.

### TUI-dashbord

TUI-en fyller hele terminalen (dynamisk størrelsesendring) og forblir responsiv:
dataene oppdateres i bakgrunnen med det konfigurerte intervallet (`--interval`,
standard 1 s); selv om en `/proc`-lesing er treg, fortsetter gjengivelsen.

| Tast           | Handling                           |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Bytt fane                        |
| `←` / `→`      | Naviger i underfaner (pooler/konfigurasjon) |
| `↑` / `↓`      | Rull i overvåkingsvisningen        |
| `q`, `Ctrl-C`  | Avslutt                            |

### Nettdashbord

`--watch` serverer de samme dataene som en én-sides webapp i stedet for å skrive
ut CLI-tabellen:

```sh
fpm-monitor --watch                 # http://127.0.0.1:8471
fpm-monitor --watch --port 9000     # egendefinert port
fpm-monitor --watch --mock          # demodata uten PHP-FPM
```

Serveren lytter bare på loopback, uten tredjepartsavhengigheter, og siden spør
`/api/state` om pooler, workerdetaljer, logger, konfigurasjonsdirektiver og de
siste 60 prøvene. Data samles i bakgrunnen ved `--interval` (standard 1 s), så en
treg skanning blokkerer aldri en forespørsel. De fire fanene er **Overvåking**,
**Diagrammer**, **Logger** og **Konfigurasjoner**; `1`–`4` og `←`/`→` velger
faner og underfaner.

`--port` godtar `1`–`65535` (`--port=9000` virker også); andre verdier avsluttes
med kode `2`. Serveren sender aldri en utgående forespørsel.

## Prosjektstruktur

| Fil               | Rolle                                               |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, valg, orkestrering                             |
| `src/config.rs`   | Oppdagelse og parsing av konfigurasjon              |
| `src/proc.rs`     | Lesing av `/proc` (workers, RSS, tilstand, TCP-backlog) |
| `src/data.rs`     | Sammenfletting av data til tabellrader              |
| `src/logs.rs`     | Lesing av PHP-logg og trege spørringer              |
| `src/render.rs`   | Tekstutdata (CLI-tabell)                            |
| `src/tui.rs`      | Interaktivt dashbord (4 faner, grafer)              |
| `src/web.rs`       | Webserver, bakgrunnsinnsamling, JSON-øyeblikksbilde |
| `src/web/index.html` | Innebygd nettdashbord på én side                   |
| `src/i18n.rs`     | Lokalisering (22 språk, `--lang`)                    |
| `src/term.rs`     | Fargedeteksjon / stiler                             |
| `tests/cli.rs`    | Ende-til-ende-CLI-tester (`--help`, `--mock`, i18n) |
| `.github/workflows` | CI (tester, dekning, 18 bygg, QEMU-røyktester)    |

## Utvikling

```sh
cargo build          # bygg (debug)
cargo test           # enhets- og integrasjonstester
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # formatering
```

CI kjører testpakken på Linux, macOS og Windows, på MSRV (Rust 1.78), publiserer
dekningsmerket, krysskompilerer de 18 utgivelsesmålene og kjører binærfiler for
fremmede arkitekturer under QEMU.

## Bidra

Bidrag er velkomne! Les [CONTRIBUTING.md](CONTRIBUTING.md) før du åpner en issue
eller en pull request, følg [retningslinjene for oppførsel](CODE_OF_CONDUCT.md),
og se [SUPPORT.md](SUPPORT.md) for spørsmål om bruk. Rapporter sikkerhetsproblemer
privat som beskrevet i [SECURITY.md](SECURITY.md).

## Lisens

[MIT](LICENSE)
