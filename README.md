<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

**English** | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

PHP-FPM pool monitor written in Rust (a port of `fpm-monitor.c`), with an
interactive terminal dashboard inspired by [Ember](https://github.com/alexandre-daubois/ember).
It ships as a single, dependency-free binary and runs on **Linux, macOS and
Windows**, across **18 target architectures**. The interface is available in 22
languages (default: English) via `--lang`, and this documentation is translated
into 22 languages.

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
- **Web dashboard** (`--watch`): the same four views in the browser, served on
  `http://127.0.0.1:8471` by a dependency-free HTTP server. The port is set with
  `--port`; data is sampled in the background at `--interval` and the page keeps
  60 points of history for its charts. A language menu offers the 22 locales
  (default: the browser/OS language, falling back to English), the choice is
  remembered locally, and a light/dark theme follows the system preference.
- Automatic configuration discovery (usual locations + `include` directive).
- **Per-pool isolation**: when launched by a non-root user, only the pools whose
  `user` directive matches that account are shown. Root sees every pool.
  See [SECURITY.md](SECURITY.md).
- No network calls: everything is read directly from `/proc` (processes, RSS,
  state, TCP socket accept queue). On macOS and BSD, the same data is read from
  `ps` and `netstat`; no FPM status configuration (`pm.status_path`) is needed.
- **Portable by design**: static musl binaries for 10 Linux architectures,
  glibc builds for the remaining ones, plus macOS and Windows. No runtime
  dependency, no libc version to match (except the documented glibc builds).

## Requirements

- **Rust 1.78+** to compile from source (MSRV tested in CI).
- **Linux**: `/proc` is used for real data. For TCP sockets the accept queue is
  read from `/proc/net/tcp`; for unix sockets only `listen.backlog` is shown as
  the maximum.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: no `/proc`; processes and
  sockets are read with `ps` and `netstat`.
- **Windows**: the CLI builds and runs (`--help`, `--mock`), but PHP-FPM is not
  available on Windows, so live process discovery returns an explicit error.

## PHP-FPM compatibility

`fpm-monitor` does not link against PHP and does not require `pm.status_path`.
It reads the pool configuration, local logs and process tables, so it supports
the PHP-FPM branches currently supported upstream: **8.2, 8.3, 8.4, 8.5 and
8.6** in 2026. Older, EOL branches may work but are not covered.

Configuration values using `${NAME}` or `${NAME:-default}` are expanded with
the environment of the `fpm-monitor` process, like PHP-FPM does at startup.
If PHP-FPM was started with a different environment, pass the same variables to
the monitor so that `user`, socket paths and log paths resolve identically.
The phpinfo tab looks for `php-cgi` and versioned binaries from `php-cgi8.2`
through `php-cgi8.6`.

## Installation

### Prebuilt binaries

Every [release](https://github.com/filoucrackeur/fpm-monitor/releases) contains
raw binaries for all supported targets, an archive per target (`.tar.gz` on
Unix, `.zip` on Windows), `.deb`/`.rpm` packages and a Homebrew formula.

The archives are self-contained: each one holds the `fpm-monitor` executable,
`LICENSE` and `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Or use the raw binary directly:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Supported platforms and architectures

| Platform | Architectures | Rust targets | Distribution |
| -------- | ------------- | ------------ | ------------ |
| Linux (static musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | binary, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | binary, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | binary, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI image |

### From source

```sh
cargo build --release
# the binary is at target/release/fpm-monitor
```

### Cross-compilation

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) is used by CI to
produce all the Linux binaries from a single x86_64 runner:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# the binary is at target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Replace the target with any triple from the table above. To deploy inside a
`php:*-fpm-alpine` container:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux packages (Red Hat / Debian)

`.deb` and `.rpm` packages are attached to each release for amd64, arm64,
armhf, armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 and sparc64. The
musl ones have no external dependency and work on any recent RHEL, Fedora,
CentOS, Debian or Ubuntu version.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- On other architectures, replace `amd64`/`x86_64` with the matching package
  name (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`,
  `s390x`, `loong64`, `sparc64`).

To rebuild the archives and packages locally from an existing `target/`
directory:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # needs nfpm on PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

A multi-arch image is published on GitHub Container Registry for
`linux/amd64`, `linux/arm64`, `linux/ppc64le` and `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` lets the monitor read the PHP-FPM processes from the host
`/proc`; mount your FPM pool configuration with `-v` so the pools are
discovered (the usual `/etc` paths are scanned too).

## Usage

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     php-fpm.conf file or pools directory to analyze
  -v, --verbose           Show each worker detail (pid, state, RSS)
      --color             Force color
      --no-color          Disable color
  -t, --tui               Interactive dashboard (tabs, `←`/`→` sub-tabs)
      --interval <SEC>    Refresh interval (default 1)
      --watch             Web dashboard instead of the CLI view
      --port <PORT>       Web dashboard port (default 8471)
      --lang <LANG>       Interface language: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (default: en)
      --mock              Demo data (local test)
  -h, --help              Show this help
```

### Interface language

`--lang` accepts `en` (US, default), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
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

### Web dashboard

`--watch` serves the same data as a single-page web application instead of
printing the CLI table:

```sh
fpm-monitor --watch                 # http://127.0.0.1:8471
fpm-monitor --watch --port 9000     # custom port
fpm-monitor --watch --mock          # demo data, no PHP-FPM required
```

The server listens on loopback only and uses no third-party crate: it is a small
`std::net::TcpListener` serving the page, the API (`/api/state`, `/api/i18n`),
the favicon and the bundled Fira fonts (`/fonts/*`). `/api/state` returns the
pools, worker details, logs, configuration directives and the last 60 samples as
JSON; `/api/i18n` serves the labels for the 22 locales. Data is collected in the
background at `--interval` (default 1 s), so a slow scan never blocks a request.

The page has four tabs — **Monitoring**, **Graphs**, **Logs** and
**Configurations** — with `1`–`4` and `←`/`→` as shortcuts, per-pool charts
with the same thresholds as the TUI, and the latest log lines. It refreshes
itself automatically and shows a banner if collection fails. The language menu
follows the browser/OS preference on first visit (English fallback), remembers
the user choice in `localStorage`, and switches the document to right-to-left
for Arabic. The php.net Fira Sans and Fira Mono webfonts are embedded in the
binary and served locally, so the typography matches php.net without any
outgoing request.

`--port` accepts `1`–`65535` (`--port=9000` also works); anything else exits
with code `2`. The server never makes an outgoing request.

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
| `src/web.rs`      | Web server, background sampler, JSON snapshot        |
| `src/web/index.html` | Embedded single-page web dashboard               |
| `src/i18n.rs`     | Localization (22 languages, `--lang`)                |
| `src/term.rs`     | Color detection / styles                            |
| `tests/cli.rs`    | End-to-end CLI tests (`--help`, `--mock`, i18n)      |
| `.github/workflows` | CI (tests, coverage, 18 builds, QEMU smoke tests) |

## Development

```sh
cargo build          # build (debug)
cargo test           # unit + integration tests
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # formatting
```

CI runs the test suite on Linux, macOS and Windows, on the MSRV (Rust 1.78),
publishes the coverage badge, cross-compiles the 18 release targets and executes
the foreign-architecture binaries under QEMU.

## Contributing

Contributions are welcome! Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening
an issue or a pull request, follow our [Code of Conduct](CODE_OF_CONDUCT.md), and
use [SUPPORT.md](SUPPORT.md) for usage questions. Please report security issues
privately as described in [SECURITY.md](SECURITY.md).

## License

[MIT](LICENSE)
