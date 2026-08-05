<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml) [![Release](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/php-fpm-monitor?sort=semver)](https://github.com/filoucrackeur/php-fpm-monitor/releases) [![codecov](https://img.shields.io/codecov/c/github/filoucrackeur/php-fpm-monitor)](https://codecov.io/github/filoucrackeur/php-fpm-monitor) [![Maintainability](https://img.shields.io/codeclimate/maintainability/filoucrackeur/php-fpm-monitor)](https://codeclimate.com/github/filoucrackeur/php-fpm-monitor) [![License](https://img.shields.io/github/license/filoucrackeur/php-fpm-monitor)](LICENSE) ![Static](https://img.shields.io/badge/static-musl-blue) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/php-fpm-monitor)](https://github.com/filoucrackeur/php-fpm-monitor/commits/main)

**English** | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md)

PHP-FPM pool monitor in Rust (a port of `fpm-monitor.c`), with an interactive
terminal dashboard inspired by [Ember](https://github.com/alexandre-daubois/ember).
The interface is available in 8 languages (default: English) via `--lang`.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — interactive dashboard (English)" width="700">
</p>

## Features

- **CLI output**: pools table (pool, type, workers, running, idle, backlog,
  max_children, max_requests, memory) with per-worker detail (`-v`: pid, state, RSS).
- **TUI dashboard** (`-t`): interactive interface with 4 tabs.
  - **Monitoring**: real-time status of the pools and their workers (memory per pool).
  - **Graphs**: one large chart per pool (Ember style) with `←`/`→` sub-tabs:
    `running`, `workers`, `idle`, `backlog` (always shown, even when empty), plus
    dashed threshold lines: `max_children` (grey), `max_requests` (red),
    `min_spare` (pink) and `max_spare` (purple) on the `idle` panel.
  - **Logs**: per pool, the latest lines of the PHP log (`error_log`), slow
    queries (`slowlog`) and the **access log** (`access.log`, colorized HTTP
    codes), in sub-tabs.
  - **Configurations**: directives read from the files (`global` + pools) in sub-tabs.
- Automatic configuration discovery (usual locations + `include` directive).
- No network calls: everything is read directly from `/proc` (processes, RSS,
  state, TCP socket accept queue).

## Requirements

- Rust (stable) to compile.
- **Linux with `/proc`** for real data. No FPM status configuration
  (`pm.status_path`) is required: data comes from `/proc` and the configuration.
  For TCP sockets, the backlog is read from `/proc/net/tcp` (accept queue); for
  unix sockets, only `listen.backlog` is shown as the maximum.

## Installation

```sh
cargo build --release
# the binary is at target/release/fpm-monitor
```

To deploy inside a `php:*-fpm-alpine` container (musl, aarch64):

```sh
RUSTC=$HOME/.cargo/bin/rustc ~/.cargo/bin/cargo build --release --target aarch64-unknown-linux-musl
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux packages (Red Hat / Debian)

Static (musl) `.deb` and `.rpm` packages are attached to each
[release](https://github.com/filoucrackeur/php-fpm-monitor/releases). They have no external
dependency and work on any recent RHEL, Fedora, CentOS, Debian or Ubuntu
version (x86_64 and ARM64).

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor_<version>_amd64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- On ARM64, use the `arm64` packages instead.

### macOS (Homebrew)

A ready-to-use formula is generated and attached to every release:

```sh
brew install https://github.com/filoucrackeur/php-fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

To host it as a tap instead, move the file to
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`, then:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

## Usage

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     php-fpm.conf file or pools directory to analyze
  -v, --verbose           Show each worker detail (pid, state, RSS)
      --color             Force color
      --no-color          Disable color
  -t, --tui               Interactive dashboard (tabs, `←`/`→` sub-tabs)
      --interval <SEC>    TUI refresh interval (default 1)
      --lang <LANG>       Interface language: en, fr, zh, ar, es, it, ja, de
                          (default: en)
      --mock              Demo data (local test)
  -h, --help              Show this help
```

### Interface language

`--lang` accepts `en` (US, default), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`
(the `--lang=fr` form is also accepted). It applies to the TUI dashboard as well
as the CLI output (headers, summary, legend) and `--help`.

### TUI dashboard

The TUI fills the whole terminal (dynamic resize) and stays responsive: data is
refreshed in the background at the configured interval (`--interval`, default 1 s);
even if a `/proc` read is slow, rendering keeps going.

| Key            | Action                             |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Switch tab                       |
| `←` / `→`      | Navigate sub-tabs (pools/config)   |
| `↑` / `↓`      | Scroll the Monitoring view         |
| `q`, `Ctrl-C`  | Quit                               |

## Project structure

| File              | Role                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, options, orchestration                         |
| `src/config.rs`   | Configuration discovery and parsing                 |
| `src/proc.rs`     | `/proc` reading (workers, RSS, state, TCP backlog)  |
| `src/data.rs`     | Merging data into table rows                        |
| `src/logs.rs`     | PHP log and slow queries reading                    |
| `src/render.rs`   | Text output (CLI table)                             |
| `src/tui.rs`      | Interactive dashboard (4 tabs, graphs)              |
| `src/i18n.rs`     | Localization (8 languages, `--lang`)                |
| `src/term.rs`     | Color detection / styles                            |

## Development

```sh
cargo build          # build (debug)
cargo test           # run the unit tests
cargo clippy         # lint
cargo fmt --check    # formatting
```

## License

[MIT](LICENSE)
