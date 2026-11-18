<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | **Italiano** | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Monitor di pool PHP-FPM scritto in Rust (un port di `fpm-monitor.c`), con un
dashboard interattivo per il terminale ispirato a [Ember](https://github.com/alexandre-daubois/ember).
Viene distribuito come un unico binario senza dipendenze e funziona su **Linux,
macOS e Windows**, su **18 architetture target**. L'interfaccia è disponibile in 8
lingue (predefinita: inglese) tramite `--lang`, e questa documentazione è tradotta
in 22 lingue.

<p align="center">
  <img src="docs/screenshot.it.animated.svg" alt="fpm-monitor TUI — dashboard interattivo (italiano)" width="700">
</p>

## Funzionalità

- **Output CLI**: tabella dei pool (pool, type, workers, running, idle, backlog,
  max_children, max_requests, memoria) con il dettaglio per worker (`-v`: pid, stato, RSS).
- **Dashboard TUI** (`-t`): interfaccia interattiva con 4 schede.
  - **Monitoraggio**: stato in tempo reale dei pool e dei loro worker (memoria per pool).
  - **Grafici**: un grande grafico per pool (stile Ember) con le sotto-schede
    `←`/`→`: `running`, `workers`, `idle`, `backlog` (sempre mostrati, anche se
    vuoti), più linee tratteggiate di soglia: `max_children` (grigio),
    `max_requests` (rosso), `min_spare` (rosa) e `max_spare` (viola) sul pannello `idle`.
  - **Log**: per pool, le ultime righe del log PHP (`error_log`), delle query
    lente (`slowlog`) e dell'**access log** (`access.log`, codici HTTP colorati),
    in sotto-schede.
  - **Configurazioni**: direttive lette dai file (`global` + pool) in sotto-schede.
- Individuazione automatica della configurazione (posizioni abituali + direttiva `include`).
- Nessuna chiamata di rete: tutto viene letto direttamente da `/proc` (processi,
  RSS, stato, coda di accept dei socket TCP). Su macOS e BSD, gli stessi dati
  vengono letti da `ps` e `netstat`; non serve alcuna configurazione dello stato
  FPM (`pm.status_path`).
- **Portabile per design**: binari statici musl per 10 architetture Linux, build
  glibc per le altre, più macOS e Windows. Nessuna dipendenza a runtime, nessuna
  versione di libc da rispettare (eccetto le build glibc documentate).

## Requisiti

- **Rust 1.78+** per compilare dai sorgenti (MSRV testata in CI).
- **Linux**: `/proc` viene usato per i dati reali. Per i socket TCP la coda di
  accept viene letta da `/proc/net/tcp`; per i socket unix viene mostrato solo
  `listen.backlog` come massimo.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: nessun `/proc`; processi e
  socket vengono letti con `ps` e `netstat`.
- **Windows**: la CLI compila e funziona (`--help`, `--mock`), ma PHP-FPM non è
  disponibile su Windows, quindi l'individuazione delle istanze live restituisce
  un errore esplicito.

## Installazione

### Binari precompilati

Ogni [release](https://github.com/filoucrackeur/fpm-monitor/releases) contiene
binari grezzi per tutti i target supportati, un archivio per target (`.tar.gz` su
Unix, `.zip` su Windows), pacchetti `.deb`/`.rpm` e una formula Homebrew.

Gli archivi sono autonomi: ognuno contiene l'eseguibile `fpm-monitor`, `LICENSE`
e `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Oppure usa direttamente il binario grezzo:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Piattaforme e architetture supportate

| Piattaforma | Architetture | Target Rust | Distribuzione |
| -------- | ------------- | ------------ | ------------ |
| Linux (musl statico) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | binario, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | binario, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | binario, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | immagine OCI |

### Dai sorgenti

```sh
cargo build --release
# il binario si trova in target/release/fpm-monitor
```

### Compilazione incrociata

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) è usato dalla CI
per produrre tutti i binari Linux da un unico runner x86_64:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# il binario si trova in target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Sostituisci il target con una qualsiasi triple della tabella qui sopra. Per
distribuire all'interno di un container `php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Pacchetti Linux (Red Hat / Debian)

I pacchetti `.deb` e `.rpm` sono allegati a ogni release per amd64, arm64,
armhf, armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 e sparc64. Quelli
musl non hanno dipendenze esterne e funzionano su qualsiasi versione recente di
RHEL, Fedora, CentOS, Debian o Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Sulle altre architetture, sostituisci `amd64`/`x86_64` con il nome del
  pacchetto corrispondente (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`,
  `ppc64`, `s390x`, `loong64`, `sparc64`).

Per ricostruire localmente gli archivi e i pacchetti da una directory `target/`
esistente:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # richiede nfpm nel PATH
```

### macOS (Homebrew)

Una formula pronta all'uso viene generata e allegata a ogni release:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

Per ospitarla invece come tap, sposta il file in
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`, poi:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Un'immagine multi-arch è pubblicata su GitHub Container Registry per
`linux/amd64`, `linux/arm64`, `linux/ppc64le` e `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` consente al monitor di leggere i processi PHP-FPM dal `/proc`
dell'host; monta la configurazione dei tuoi pool FPM con `-v` affinché i pool
vengano individuati (anche i consueti percorsi `/etc` vengono analizzati).

## Utilizzo

```
fpm-monitor [OPTIONS]

Opzioni:
  -c, --config <PATH>     file php-fpm.conf o directory dei pool da analizzare
  -v, --verbose           Mostra il dettaglio di ogni worker (pid, stato, RSS)
      --color             Forza il colore
      --no-color          Disabilita il colore
  -t, --tui               Dashboard interattiva (schede, sotto-schede `←`/`→`)
      --interval <SEC>    Intervallo di aggiornamento della TUI (predefinito 1)
      --lang <LANG>       Lingua dell'interfaccia: en, fr, zh, ar, es, it, ja, de
                          (predefinita: en)
      --mock              Dati dimostrativi (test locale)
  -h, --help              Mostra questa guida
```

### Lingua dell'interfaccia

`--lang` accetta `en` (US, predefinita), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`
(è accettata anche la forma `--lang=fr`). Si applica sia alla dashboard TUI sia
all'output CLI (intestazioni, riepilogo, legenda) e a `--help`.

### Dashboard TUI

La TUI occupa l'intero terminale (ridimensionamento dinamico) e resta reattiva:
i dati vengono aggiornati in background all'intervallo configurato (`--interval`,
predefinito 1 s); anche se una lettura di `/proc` è lenta, il rendering continua.

| Tasto            | Azione                             |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Cambia scheda                    |
| `←` / `→`      | Naviga le sotto-schede (pool/config)   |
| `↑` / `↓`      | Scorri la vista Monitoraggio       |
| `q`, `Ctrl-C`  | Esci                               |

## Struttura del progetto

| File              | Ruolo                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, opzioni, orchestrazione                         |
| `src/config.rs`   | Individuazione e parsing della configurazione        |
| `src/proc.rs`     | Lettura di `/proc` (worker, RSS, stato, backlog TCP) |
| `src/data.rs`     | Unione dei dati in righe di tabella                  |
| `src/logs.rs`     | Lettura del log PHP e delle query lente              |
| `src/render.rs`   | Output testuale (tabella CLI)                        |
| `src/tui.rs`      | Dashboard interattiva (4 schede, grafici)            |
| `src/i18n.rs`     | Localizzazione (8 lingue, `--lang`)                  |
| `src/term.rs`     | Rilevamento colore / stili                           |
| `tests/cli.rs`    | Test CLI end-to-end (`--help`, `--mock`, i18n)       |
| `.github/workflows` | CI (test, copertura, 18 build, smoke test QEMU)    |

## Sviluppo

```sh
cargo build          # compilazione (debug)
cargo test           # test unitari + di integrazione
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # formattazione
```

La CI esegue la suite di test su Linux, macOS e Windows, sulla MSRV (Rust 1.78),
pubblica il badge di copertura, compila in modo incrociato i 18 target di release
ed esegue i binari per architetture esterne sotto QEMU.

## Licenza

[MIT](LICENSE)
