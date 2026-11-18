<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | **Bahasa Indonesia** | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Monitor pool PHP-FPM yang ditulis dalam Rust (port dari `fpm-monitor.c`), dengan
dasbor terminal interaktif yang terinspirasi oleh
[Ember](https://github.com/alexandre-daubois/ember). Dirilis sebagai satu biner
tanpa dependensi dan berjalan di **Linux, macOS, dan Windows**, pada **18
arsitektur target**. Antarmuka tersedia dalam 8 bahasa (default: Inggris)
melalui `--lang`, dan dokumentasi ini diterjemahkan ke dalam 22 bahasa.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="Dasbor interaktif fpm-monitor TUI (Inggris)" width="700">
</p>

## Fitur

- **Keluaran CLI**: tabel pool (pool, type, workers, running, idle, backlog,
  max_children, max_requests, memori) dengan detail per worker (`-v`: pid,
  status, RSS).
- **Dasbor TUI** (`-t`): antarmuka interaktif dengan 4 tab.
  - **Pemantauan**: status real-time pool dan worker-nya (memori per pool).
  - **Grafik**: satu grafik besar per pool (gaya Ember) dengan sub-tab `←`/`→`:
    `running`, `workers`, `idle`, `backlog` (selalu ditampilkan, bahkan saat
    kosong), plus garis ambang putus-putus: `max_children` (abu-abu),
    `max_requests` (merah), `min_spare` (merah muda) dan `max_spare` (ungu) pada
    panel `idle`.
  - **Log**: per pool, baris terbaru log PHP (`error_log`), kueri lambat
    (`slowlog`) dan **log akses** (`access.log`, kode HTTP berwarna), dalam
    sub-tab.
  - **Konfigurasi**: direktif yang dibaca dari berkas (`global` + pool) dalam
    sub-tab.
- Penemuan konfigurasi otomatis (lokasi umum + direktif `include`).
- Tanpa panggilan jaringan: semuanya dibaca langsung dari `/proc` (proses, RSS,
  status, antrean penerimaan soket TCP). Di macOS dan BSD, data yang sama dibaca
  dari `ps` dan `netstat`; tidak diperlukan konfigurasi status FPM
  (`pm.status_path`).
- **Portabel secara desain**: biner musl statis untuk 10 arsitektur Linux, build
  glibc untuk sisanya, plus macOS dan Windows. Tanpa dependensi runtime, tanpa
  versi libc yang harus dicocokkan (kecuali build glibc yang didokumentasikan).

## Persyaratan

- **Rust 1.78+** untuk mengompilasi dari sumber (MSRV diuji di CI).
- **Linux**: `/proc` digunakan untuk data nyata. Untuk soket TCP, antrean
  penerimaan dibaca dari `/proc/net/tcp`; untuk soket unix hanya `listen.backlog`
  yang ditampilkan sebagai nilai maksimum.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: tanpa `/proc`; proses dan
  soket dibaca dengan `ps` dan `netstat`.
- **Windows**: CLI dapat dibangun dan dijalankan (`--help`, `--mock`), tetapi
  PHP-FPM tidak tersedia di Windows, sehingga penemuan proses langsung
  mengembalikan galat yang eksplisit.

## Instalasi

### Biner siap pakai

Setiap [release](https://github.com/filoucrackeur/fpm-monitor/releases) berisi
biner mentah untuk semua target yang didukung, satu arsip per target (`.tar.gz`
di Unix, `.zip` di Windows), paket `.deb`/`.rpm`, dan formula Homebrew.

Arsip bersifat mandiri: masing-masing berisi eksekutabel `fpm-monitor`,
`LICENSE`, dan `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Atau gunakan biner mentah secara langsung:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Platform dan arsitektur yang didukung

| Platform | Arsitektur | Target Rust | Distribusi |
| -------- | ---------- | ----------- | ---------- |
| Linux (musl statis) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | biner, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | biner, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | biner, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | image OCI |

### Dari sumber

```sh
cargo build --release
# biner berada di target/release/fpm-monitor
```

### Kompilasi silang

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) digunakan oleh CI
untuk menghasilkan semua biner Linux dari satu runner x86_64:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# biner berada di target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Ganti target dengan triple mana pun dari tabel di atas. Untuk men-deploy di dalam
kontainer `php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Paket Linux (Red Hat / Debian)

Paket `.deb` dan `.rpm` dilampirkan pada setiap rilis untuk amd64, arm64, armhf,
armel, i386, riscv64, ppc64el, ppc64, s390x, loong64, dan sparc64. Paket musl
tidak memiliki dependensi eksternal dan berfungsi pada versi RHEL, Fedora,
CentOS, Debian, atau Ubuntu terbaru mana pun.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Pada arsitektur lain, ganti `amd64`/`x86_64` dengan nama paket yang sesuai
  (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`, `s390x`,
  `loong64`, `sparc64`).

Untuk membangun ulang arsip dan paket secara lokal dari direktori `target/`
yang sudah ada:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # memerlukan nfpm di PATH
```

### macOS (Homebrew)

Formula siap pakai dibuat dan dilampirkan pada setiap rilis:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

Untuk menghosting-nya sebagai tap, pindahkan berkas ke
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`, lalu:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Image multi-arsitektur dipublikasikan di GitHub Container Registry untuk
`linux/amd64`, `linux/arm64`, `linux/ppc64le`, dan `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` memungkinkan monitor membaca proses PHP-FPM dari `/proc` host;
pasang konfigurasi pool FPM Anda dengan `-v` agar pool terdeteksi (jalur `/etc`
yang umum juga dipindai).

## Penggunaan

```
fpm-monitor [OPTIONS]

Pilihan:
  -c, --config <PATH>     berkas php-fpm.conf atau direktori pool yang akan dianalisis
  -v, --verbose           Tampilkan detail setiap worker (pid, status, RSS)
      --color             Paksa warna
      --no-color          Nonaktifkan warna
  -t, --tui               Dasbor interaktif (tab, sub-tab `←`/`→`)
      --interval <SEC>    Interval penyegaran TUI (default 1)
      --lang <LANG>       Bahasa antarmuka: en, fr, zh, ar, es, it, ja, de
                          (default: en)
      --mock              Data demo (uji lokal)
  -h, --help              Tampilkan bantuan ini
```

### Bahasa antarmuka

`--lang` menerima `en` (US, default), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`
(bentuk `--lang=fr` juga diterima). Ini berlaku untuk dasbor TUI serta keluaran
CLI (header, ringkasan, legenda) dan `--help`.

### Dasbor TUI

TUI memenuhi seluruh terminal (resize dinamis) dan tetap responsif: data
disegarkan di latar belakang pada interval yang ditentukan (`--interval`, default
1 dtk); bahkan jika pembacaan `/proc` lambat, rendering tetap berjalan.

| Tombol | Aksi |
| ------ | ---- |
| `1` – `4`, `Tab` | Beralih tab |
| `←` / `→` | Menavigasi sub-tab (pool/konfig) |
| `↑` / `↓` | Menggulir tampilan Pemantauan |
| `q`, `Ctrl-C` | Keluar |

## Struktur proyek

| Berkas | Peran |
| ------ | ----- |
| `src/main.rs` | CLI, opsi, orkestrasi |
| `src/config.rs` | Penemuan dan penguraian konfigurasi |
| `src/proc.rs` | Pembacaan `/proc` (worker, RSS, status, backlog TCP) |
| `src/data.rs` | Menggabungkan data menjadi baris tabel |
| `src/logs.rs` | Pembacaan log PHP dan kueri lambat |
| `src/render.rs` | Keluaran teks (tabel CLI) |
| `src/tui.rs` | Dasbor interaktif (4 tab, grafik) |
| `src/i18n.rs` | Lokalisasi (8 bahasa, `--lang`) |
| `src/term.rs` | Deteksi warna / gaya |
| `tests/cli.rs` | Uji CLI end-to-end (`--help`, `--mock`, i18n) |
| `.github/workflows` | CI (uji, cakupan, 18 build, uji asap QEMU) |

## Pengembangan

```sh
cargo build          # build (debug)
cargo test           # uji unit + integrasi
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # format
```

CI menjalankan rangkaian uji di Linux, macOS, dan Windows, pada MSRV (Rust 1.78),
menerbitkan lencana cakupan, mengompilasi silang 18 target rilis, dan menjalankan
biner arsitektur asing di bawah QEMU.

## Lisensi

[MIT](LICENSE)
