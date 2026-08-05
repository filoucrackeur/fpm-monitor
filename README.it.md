<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | **Italiano** | [日本語](README.ja.md) | [Deutsch](README.de.md)

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Release](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/release.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/release.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases) [![codecov](https://img.shields.io/codecov/c/github/filoucrackeur/fpm-monitor)](https://codecov.io/github/filoucrackeur/fpm-monitor) [![Maintainability](https://img.shields.io/codeclimate/maintainability/filoucrackeur/fpm-monitor)](https://codeclimate.com/github/filoucrackeur/fpm-monitor) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) ![Static](https://img.shields.io/badge/static-musl-blue) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

Monitor dei pool PHP-FPM in Rust (un port di `fpm-monitor.c`), con un dashboard
interattivo nel terminale ispirato a [Ember](https://github.com/alexandre-daubois/ember).
L'interfaccia è disponibile in 8 lingue (predefinita: inglese) tramite `--lang`.

<p align="center">
  <img src="docs/screenshot.it.animated.svg" alt="Dashboard interattivo fpm-monitor TUI (italiano)" width="700">
</p>

## Funzionalità

- **Output CLI**: tabella dei pool (pool, type, workers, running, idle, backlog,
  max_children, max_requests, memoria) con dettaglio per worker (`-v`: pid, stato, RSS).
- **Dashboard TUI** (`-t`): interfaccia interattiva con 4 schede.
  - **Monitoraggio**: stato in tempo reale dei pool e dei loro worker (memoria per pool).
  - **Grafici**: un grande grafico per pool (stile Ember) con sottoschede ← →:
    `running`, `workers`, `idle`, `backlog` (sempre visibile, anche se vuoto), e
    linee di soglia tratteggiate: `max_children` (grigio), `max_requests` (rosso),
    `min_spare` (rosa) e `max_spare` (viola) sul pannello `idle`.
  - **Log**: per pool, le ultime righe del log PHP (`error_log`), delle query
    lente (`slowlog`) e dell'**access log** (`access.log`, codici HTTP colorati),
    in sottoschede.
  - **Configurazioni**: direttive lette dai file (`global` + pool) in sottoschede.
- Rilevamento automatico della configurazione (posizioni usuali + direttiva `include`).
- Nessuna chiamata di rete: tutto viene letto direttamente da `/proc` (processi,
  RSS, stato, coda di accettazione dei socket TCP).

## Requisiti

- Rust (stable) per compilare.
- **Linux con `/proc`** per i dati reali. Non è necessaria alcuna configurazione di
  stato FPM (`pm.status_path`): i dati provengono da `/proc` e dalla configurazione.
  Per i socket TCP, il backlog viene letto da `/proc/net/tcp` (coda di
  accettazione); per i socket unix viene mostrato solo `listen.backlog` come massimo.

## Installazione

```sh
cargo build --release
# il binario si trova in target/release/fpm-monitor
```

Per il deploy in un container `php:*-fpm-alpine` (musl, aarch64):

```sh
RUSTC=$HOME/.cargo/bin/rustc ~/.cargo/bin/cargo build --release --target aarch64-unknown-linux-musl
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Pacchetti Linux (Red Hat / Debian)

Ogni [release](https://github.com/filoucrackeur/fpm-monitor/releases) include pacchetti `.deb` e
`.rpm` statici (musl). Senza dipendenze esterne, funzionano su qualsiasi versione
recente di RHEL, Fedora, CentOS, Debian o Ubuntu (x86_64 e ARM64).

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor_<version>_amd64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Su ARM64, usa i pacchetti `arm64` corrispondenti.

### macOS (Homebrew)

Ogni release genera e allega una formula pronta all'uso:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

Per ospitarla come tap, sposta il file in
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`, poi:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

## Utilizzo

```
fpm-monitor [OPZIONI]

Opzioni:
  -c, --config <PERCORSO>  File php-fpm.conf o cartella dei pool da analizzare
  -v, --verbose            Mostra il dettaglio di ogni worker (pid, stato, RSS)
      --color              Forza il colore
      --no-color           Disabilita il colore
  -t, --tui                Dashboard interattivo (schede, sottoschede ← →)
      --interval <SEC>     Intervallo di aggiornamento del TUI (default 1)
      --lang <LINGUA>      Lingua dell'interfaccia: en, fr, zh, ar, es, it, ja, de
                           (default: en)
      --mock               Dati dimostrativi (test locale)
  -h, --help               Mostra questo aiuto
```

### Lingua dell'interfaccia

`--lang` accetta `en` (US, predefinita), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`
(viene accettata anche la forma `--lang=fr`). Si applica al dashboard TUI e anche
all'output CLI (intestazioni, riepilogo, legenda) e a `--help`.

### Dashboard TUI

Il TUI occupa tutto il terminale (ridimensionamento dinamico) e resta reattivo: i
dati vengono aggiornati in background all'intervallo configurato (`--interval`,
default 1 s); anche se una lettura `/proc` è lenta, il rendering continua.

| Tasto          | Azione                                |
| -------------- | ------------------------------------- |
| `1` – `4`, `Tab` | Cambia scheda                      |
| `←` / `→`      | Naviga le sottoschede (pool/config)   |
| `↑` / `↓`      | Scorri la vista Monitoraggio          |
| `q`, `Ctrl-C`  | Esci                                  |

## Struttura del progetto

| File                | Ruolo                                     |
| ------------------- | ----------------------------------------- |
| `src/main.rs`       | CLI, opzioni, orchestrazione              |
| `src/config.rs`     | Rilevamento e parsing della configurazione|
| `src/proc.rs`       | Lettura di `/proc` (worker, RSS, stato, backlog TCP) |
| `src/data.rs`       | Fusione dei dati in righe di tabella      |
| `src/logs.rs`       | Lettura del log PHP e delle query lente   |
| `src/render.rs`     | Output di testo (tabella CLI)             |
| `src/tui.rs`        | Dashboard interattivo (4 schede, grafici) |
| `src/i18n.rs`       | Localizzazione (8 lingue, `--lang`)       |
| `src/term.rs`       | Rilevamento colore / stili                |

## Sviluppo

```sh
cargo build          # compilazione (debug)
cargo test           # esecuzione dei test unitari
cargo clippy         # lint
cargo fmt --check    # formattazione
```

## Licenza

[MIT](LICENSE)
