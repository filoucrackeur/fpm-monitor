<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | **Polski** | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Napisany w Rust monitor pul PHP-FPM (port `fpm-monitor.c`) z interaktywnym panelem
terminalowym inspirowanym [Ember](https://github.com/alexandre-daubois/ember).
Jest dostarczany jako pojedynczy plik binarny bez zależności i działa na
**Linuksie, macOS i Windows** na **18 architekturach docelowych**. Interfejs jest
dostępny w 22 językach (domyślnie: angielski) przez `--lang`, a ta dokumentacja
została przetłumaczona na 22 języki.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — interaktywny panel (angielski)" width="700">
</p>

## Funkcje

- **Dane wyjściowe CLI**: tabela pul (pula, typ, workery, uruchomione, bezczynne,
  kolejka, max_children, max_requests, pamięć) wraz ze szczegółami każdego workera
  (`-v`: pid, stan, RSS).
- **Panel TUI** (`-t`): interaktywny interfejs z 4 zakładkami.
  - **Monitorowanie**: stan pul i ich workerów w czasie rzeczywistym (pamięć na pulę).
  - **Wykresy**: jeden duży wykres na pulę (w stylu Ember) z podzakładkami
    przełączanymi przez `←`/`→`: `running`, `workers`, `idle`, `backlog` (zawsze
    widoczne, nawet gdy są puste), a także przerywane linie progowe: `max_children`
    (szara), `max_requests` (czerwona), `min_spare` (różowa) i `max_spare`
    (fioletowa) na panelu `idle`.
  - **Dzienniki**: dla każdej puli najnowsze wiersze dziennika PHP (`error_log`),
    wolnych zapytań (`slowlog`) oraz **dziennika dostępu** (`access.log`, kody HTTP
    z kolorowaniem), w podzakładkach.
  - **Konfiguracje**: dyrektywy odczytane z plików (`global` + pule) w podzakładkach.
- Automatyczne wykrywanie konfiguracji (typowe lokalizacje + dyrektywa `include`).
- Brak połączeń sieciowych: wszystko jest odczytywane bezpośrednio z `/proc`
  (procesy, RSS, stan, kolejka akceptacji gniazd TCP). W systemach macOS i BSD te
  same dane są odczytywane z `ps` i `netstat`; konfiguracja statusu FPM
  (`pm.status_path`) nie jest potrzebna.
- **Przenośny z założenia**: statyczne binaria musl dla 10 architektur Linuksa,
  kompilacje glibc dla pozostałych, a także macOS i Windows. Brak zależności w
  czasie działania i wersji libc do dopasowania (z wyjątkiem opisanych kompilacji
  glibc).

## Wymagania

- **Rust 1.78+** do kompilacji ze źródeł (MSRV testowany w CI).
- **Linux**: do rzeczywistych danych używany jest `/proc`. W przypadku gniazd TCP
  kolejka akceptacji jest odczytywana z `/proc/net/tcp`; dla gniazd unixowych
  wyświetlany jest tylko `listen.backlog` jako wartość maksymalna.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: brak `/proc`; procesy i
  gniazda są odczytywane za pomocą `ps` i `netstat`.
- **Windows**: CLI kompiluje się i działa (`--help`, `--mock`), ale ponieważ
  PHP-FPM nie jest dostępny w systemie Windows, wykrywanie aktywnych procesów
  zwraca jawny błąd.

## Instalacja

### Gotowe binaria

Każde [wydanie](https://github.com/filoucrackeur/fpm-monitor/releases) zawiera
surowe binaria dla wszystkich obsługiwanych celów, archiwum dla każdego celu
(`.tar.gz` w systemach Unix, `.zip` w systemie Windows), pakiety `.deb`/`.rpm`
oraz formułę Homebrew.

Archiwa są samowystarczalne: każde zawiera plik wykonywalny `fpm-monitor`,
`LICENSE` i `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Możesz też użyć surowego binarnego bezpośrednio:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Obsługiwane platformy i architektury

| Platforma | Architektury | Cele Rust | Dystrybucja |
| -------- | ------------- | ------------ | ------------ |
| Linux (statyczne musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | binaria, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | binaria, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | binaria, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | obraz OCI |

### Ze źródeł

```sh
cargo build --release
# plik binarny znajduje się w target/release/fpm-monitor
```

### Kompilacja krzyżowa

CI używa [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) do
tworzenia wszystkich binariów Linuksa z jednego runnera x86_64:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# plik binarny znajduje się w target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Zastąp cel dowolnym tripletem z powyższej tabeli. Aby wdrożyć w kontenerze
`php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Pakiety Linuksa (Red Hat / Debian)

Pakiety `.deb` i `.rpm` są dołączane do każdego wydania dla amd64, arm64, armhf,
armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 i sparc64. Pakiety musl nie
mają zależności zewnętrznych i działają w każdej nowszej wersji RHEL, Fedora,
CentOS, Debian lub Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- W innych architekturach zastąp `amd64`/`x86_64` odpowiednią nazwą pakietu
  (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`, `s390x`,
  `loong64`, `sparc64`).

Aby odtworzyć archiwa i pakiety lokalnie z istniejącego katalogu `target/`:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # wymaga nfpm w PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Wieloarchitekturowy obraz jest publikowany w GitHub Container Registry dla
`linux/amd64`, `linux/arm64`, `linux/ppc64le` i `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` pozwala monitorowi odczytywać procesy PHP-FPM z hostowego `/proc`;
zamontuj konfigurację pul FPM przez `-v`, aby pule zostały wykryte (typowe ścieżki
`/etc` są również skanowane).

## Użycie

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     Plik php-fpm.conf lub katalog pul do analizy
  -v, --verbose           Pokazuj szczegóły każdego workera (pid, stan, RSS)
      --color             Wymuś kolor
      --no-color          Wyłącz kolor
  -t, --tui               Interaktywny panel (zakładki, podzakładki `←`/`→`)
      --interval <SEC>    Interwał odświeżania TUI (domyślnie 1)
      --lang <LANG>       Język interfejsu: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (domyślnie: en)
      --mock              Dane demonstracyjne (test lokalny)
  -h, --help              Pokaż tę pomoc
```

### Język interfejsu

`--lang` akceptuje `en` (US, domyślnie), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
(forma `--lang=fr` jest również akceptowana). Ma zastosowanie zarówno do panelu TUI,
jak i do danych wyjściowych CLI (nagłówki, podsumowanie, legenda) oraz `--help`.

### Panel TUI

TUI zajmuje cały terminal (dynamiczna zmiana rozmiaru) i pozostaje responsywny:
dane są odświeżane w tle w skonfigurowanym interwale (`--interval`, domyślnie 1 s);
nawet jeśli odczyt `/proc` jest wolny, renderowanie jest kontynuowane.

| Klawisz        | Działanie                          |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Przełącz zakładkę                |
| `←` / `→`      | Nawiguj po podzakładkach (pule/konfiguracja) |
| `↑` / `↓`      | Przewiń widok Monitorowanie         |
| `q`, `Ctrl-C`  | Wyjście                            |

## Struktura projektu

| Plik              | Rola                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, opcje, orkiestracja                            |
| `src/config.rs`   | Wykrywanie i parsowanie konfiguracji                |
| `src/proc.rs`     | Odczyt `/proc` (workery, RSS, stan, kolejka TCP)    |
| `src/data.rs`     | Scalanie danych w wiersze tabeli                    |
| `src/logs.rs`     | Odczyt dziennika PHP i wolnych zapytań              |
| `src/render.rs`   | Wyjście tekstowe (tabela CLI)                       |
| `src/tui.rs`      | Interaktywny panel (4 zakładki, wykresy)            |
| `src/i18n.rs`     | Lokalizacja (22 języków, `--lang`)                   |
| `src/term.rs`     | Wykrywanie kolorów / style                          |
| `tests/cli.rs`    | Testy CLI end-to-end (`--help`, `--mock`, i18n)     |
| `.github/workflows` | CI (testy, pokrycie, 18 kompilacji, testy QEMU)   |

## Rozwój

```sh
cargo build          # kompilacja (debug)
cargo test           # testy jednostkowe + integracyjne
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # formatowanie
```

CI uruchamia zestaw testów na Linuksie, macOS i Windows, na MSRV (Rust 1.78),
publikuje odznakę pokrycia, kompiluje krzyżowo 18 celów wydania i uruchamia
binaria obcych architektur pod QEMU.

## Współtworzenie

Wkład jest mile widziany! Przeczytaj [CONTRIBUTING.md](CONTRIBUTING.md) przed
otwarciem issue lub pull requesta, przestrzegaj naszego
[Kodeksu postępowania](CODE_OF_CONDUCT.md), a w razie pytań o użytkowanie zajrzyj
do [SUPPORT.md](SUPPORT.md). Problemy z bezpieczeństwem zgłaszaj prywatnie,
zgodnie z opisem w [SECURITY.md](SECURITY.md).

## Licencja

[MIT](LICENSE)
