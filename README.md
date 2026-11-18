<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
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
Windows**, across **18 target architectures**. The interface is available in 8
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
- Automatic configuration discovery (usual locations + `include` directive).
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

A ready-to-use formula is generated and attached to every release:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

To host it as a tap instead, move the file to
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`, then:

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

## License

[MIT](LICENSE)
