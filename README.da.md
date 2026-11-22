<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | **Dansk** | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

PHP-FPM-poolovervågning skrevet i Rust (en port af `fpm-monitor.c`), med et
interaktivt terminal-dashboard inspireret af [Ember](https://github.com/alexandre-daubois/ember).
Det leveres som én enkelt binærfil uden afhængigheder og kører på **Linux, macOS
og Windows** på tværs af **18 target-arkitekturer**. Grænsefladen findes i 22
sprog (standard: engelsk) via `--lang`, og denne dokumentation er oversat til
22 sprog.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — interaktivt dashboard (engelsk)" width="700">
</p>

## Funktioner

- **CLI-output**: tabel over pools (pool, type, workers, running, idle, backlog,
  max_children, max_requests, hukommelse) med detaljer pr. worker (`-v`: pid, tilstand, RSS).
- **TUI-dashboard** (`-t`): interaktiv grænseflade med 4 faner.
  - **Overvågning**: realtidsstatus for poolene og deres workers (hukommelse pr. pool).
  - **Grafer**: ét stort diagram pr. pool (Ember-stil) med underfanerne
    `←`/`→`: `running`, `workers`, `idle`, `backlog` (vises altid, også når de er
    tomme), samt stiplede tærskellinjer: `max_children` (grå), `max_requests`
    (rød), `min_spare` (pink) og `max_spare` (lilla) på panelet `idle`.
  - **Logfiler**: pr. pool de seneste linjer fra PHP-loggen (`error_log`),
    langsomme forespørgsler (`slowlog`) og **access-loggen** (`access.log`,
    farvelagte HTTP-koder), i underfaner.
  - **Konfigurationer**: direktiver læst fra filerne (`global` + pools) i underfaner.
- Automatisk registrering af konfiguration (sædvanlige placeringer + `include`-direktivet).
- Ingen netværkskald: alt læses direkte fra `/proc` (processer, RSS, tilstand,
  accept-kø for TCP-sockets). På macOS og BSD læses de samme data fra `ps` og
  `netstat`; der kræves ingen FPM-statuskonfiguration (`pm.status_path`).
- **Bærbar af design**: statiske musl-binærfiler til 10 Linux-arkitekturer,
  glibc-builds til de resterende, plus macOS og Windows. Ingen
  køretidsafhængighed, ingen libc-version der skal matche (undtagen de
  dokumenterede glibc-builds).

## Krav

- **Rust 1.78+** for at kompilere fra kildekoden (MSRV testet i CI).
- **Linux**: `/proc` bruges til rigtige data. For TCP-sockets læses accept-køen
  fra `/proc/net/tcp`; for unix-sockets vises kun `listen.backlog` som maksimum.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: ingen `/proc`; processer og
  sockets læses med `ps` og `netstat`.
- **Windows**: CLI'en bygger og kører (`--help`, `--mock`), men PHP-FPM er ikke
  tilgængeligt på Windows, så registrering af kørende processer returnerer en
  eksplicit fejl.

## Installation

### Forbyggede binærfiler

Hver [release](https://github.com/filoucrackeur/fpm-monitor/releases) indeholder
rå binærfiler til alle understøttede targets, et arkiv pr. target (`.tar.gz` på
Unix, `.zip` på Windows), `.deb`/`.rpm`-pakker og en Homebrew-formel.

Arkiverne er selvbærende: hvert arkiv indeholder eksekverbaren `fpm-monitor`,
`LICENSE` og `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Eller brug den rå binærfil direkte:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Understøttede platforme og arkitekturer

| Platform | Arkitekturer | Rust-targets | Distribution |
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

### Krydskompilering

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) bruges af CI til
at producere alle Linux-binærfilerne fra én enkelt x86_64-runner:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# binærfilen ligger i target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Erstat target med en vilkårlig triple fra tabellen ovenfor. For at deploye i en
`php:*-fpm-alpine`-container:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux-pakker (Red Hat / Debian)

`.deb`- og `.rpm`-pakker vedhæftes hver release til amd64, arm64, armhf, armel,
i386, riscv64, ppc64el, ppc64, s390x, loong64 og sparc64. Musl-pakkerne har ingen
eksterne afhængigheder og fungerer på enhver nyere version af RHEL, Fedora,
CentOS, Debian eller Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- På andre arkitekturer skal du erstatte `amd64`/`x86_64` med det tilsvarende
  pakkenavn (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`,
  `s390x`, `loong64`, `sparc64`).

For at genopbygge arkiverne og pakkerne lokalt fra en eksisterende
`target/`-mappe:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # kræver nfpm i PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Et multi-arch-image udgives på GitHub Container Registry til `linux/amd64`,
`linux/arm64`, `linux/ppc64le` og `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` lader overvågningen læse PHP-FPM-processerne fra værtens `/proc`;
montér din FPM-poolkonfiguration med `-v`, så poolene registreres (de sædvanlige
`/etc`-stier scannes også).

## Brug

```
fpm-monitor [OPTIONS]

Indstillinger:
  -c, --config <PATH>     php-fpm.conf-fil eller pool-mappe, der skal analyseres
  -v, --verbose           Vis detaljer for hver worker (pid, tilstand, RSS)
      --color             Gennemtving farve
      --no-color          Deaktiver farve
  -t, --tui               Interaktivt dashboard (faner, `←`/`→`-underfaner)
      --interval <SEC>    Opdateringsinterval for TUI (standard 1)
      --lang <LANG>       Grænsefladesprog: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (standard: en)
      --mock              Demodata (lokal test)
  -h, --help              Vis denne hjælp
```

### Grænsefladesprog

`--lang` accepterer `en` (US, standard), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
(formen `--lang=fr` accepteres også). Det gælder både TUI-dashboardet og
CLI-outputtet (overskrifter, oversigt, forklaring) samt `--help`.

### TUI-dashboard

TUI'en fylder hele terminalen (dynamisk størrelsesændring) og forbliver
responsiv: data opdateres i baggrunden med det konfigurerede interval
(`--interval`, standard 1 s); selv hvis en `/proc`-læsning er langsom, fortsætter
renderingen.

| Tast           | Handling                           |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Skift fane                       |
| `←` / `→`      | Naviger i underfaner (pools/config) |
| `↑` / `↓`      | Rul overvågningsvisningen          |
| `q`, `Ctrl-C`  | Afslut                             |

## Projektstruktur

| Fil               | Rolle                                               |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, indstillinger, orkestrering                    |
| `src/config.rs`   | Registrering og parsing af konfiguration            |
| `src/proc.rs`     | Læsning af `/proc` (workers, RSS, tilstand, TCP-backlog) |
| `src/data.rs`     | Sammenfletning af data til tabelrækker              |
| `src/logs.rs`     | Læsning af PHP-log og langsomme forespørgsler       |
| `src/render.rs`   | Tekstoutput (CLI-tabel)                             |
| `src/tui.rs`      | Interaktivt dashboard (4 faner, grafer)             |
| `src/i18n.rs`     | Lokalisering (22 sprog, `--lang`)                    |
| `src/term.rs`     | Farvedetektering / stilarter                        |
| `tests/cli.rs`    | End-to-end CLI-tests (`--help`, `--mock`, i18n)     |
| `.github/workflows` | CI (tests, dækning, 18 builds, QEMU-smoketests)   |

## Udvikling

```sh
cargo build          # byg (debug)
cargo test           # enheds- + integrationstests
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # formatering
```

CI kører testsuiten på Linux, macOS og Windows, på MSRV (Rust 1.78), publicerer
dækningsbadgen, krydskompilerer de 18 release-targets og kører binærfilerne til
fremmede arkitekturer under QEMU.

## Bidrag

Bidrag er velkomne! Læs [CONTRIBUTING.md](CONTRIBUTING.md), før du åbner en issue
eller en pull request, følg vores [adfærdskodeks](CODE_OF_CONDUCT.md), og se
[SUPPORT.md](SUPPORT.md) for spørgsmål om brug. Rapportér sikkerhedsproblemer
privat som beskrevet i [SECURITY.md](SECURITY.md).

## Licens

[MIT](LICENSE)
