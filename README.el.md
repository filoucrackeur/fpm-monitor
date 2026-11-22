<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | **Ελληνικά** | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Παρακολούθηση pools PHP-FPM γραμμένη σε Rust (μεταφορά του `fpm-monitor.c`), με
ένα διαδραστικό dashboard τερματικού εμπνευσμένο από το
[Ember](https://github.com/alexandre-daubois/ember). Διανέμεται ως ένα μόνο
εκτελέσιμο χωρίς εξαρτήσεις και λειτουργεί σε **Linux, macOS και Windows**, σε
**18 αρχιτεκτονικές target**. Η διεπαφή είναι διαθέσιμη σε 22 γλώσσες
(προεπιλογή: αγγλικά) μέσω `--lang`, και αυτή η τεκμηρίωση είναι μεταφρασμένη σε
22 γλώσσες.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — διαδραστικό dashboard (αγγλικά)" width="700">
</p>

## Χαρακτηριστικά

- **Έξοδος CLI**: πίνακας pools (pool, type, workers, running, idle, backlog,
  max_children, max_requests, μνήμη) με λεπτομέρειες ανά worker (`-v`: pid, κατάσταση, RSS).
- **Dashboard TUI** (`-t`): διαδραστική διεπαφή με 4 καρτέλες.
  - **Παρακολούθηση**: κατάσταση σε πραγματικό χρόνο των pools και των workers
    τους (μνήμη ανά pool).
  - **Γραφήματα**: ένα μεγάλο γράφημα ανά pool (στυλ Ember) με υποκαρτέλες
    `←`/`→`: `running`, `workers`, `idle`, `backlog` (εμφανίζονται πάντα, ακόμη
    και όταν είναι κενά), καθώς και διακεκομμένες γραμμές ορίων: `max_children`
    (γκρι), `max_requests` (κόκκινο), `min_spare` (ροζ) και `max_spare` (μωβ) στο
    πάνελ `idle`.
  - **Αρχεία καταγραφής**: ανά pool, οι τελευταίες γραμμές του αρχείου
    καταγραφής PHP (`error_log`), των αργών ερωτημάτων (`slowlog`) και του
    **access log** (`access.log`, HTTP codes με χρώματα), σε υποκαρτέλες.
  - **Διαμορφώσεις**: οδηγίες που διαβάζονται από τα αρχεία (`global` + pools) σε
    υποκαρτέλες.
- Αυτόματη ανακάλυψη διαμόρφωσης (συνήθεις τοποθεσίες + οδηγία `include`).
- Χωρίς κλήσεις δικτύου: όλα διαβάζονται απευθείας από το `/proc` (διεργασίες,
  RSS, κατάσταση, ουρά αποδοχής TCP sockets). Σε macOS και BSD, τα ίδια δεδομένα
  διαβάζονται από `ps` και `netstat`· δεν απαιτείται διαμόρφωση κατάστασης FPM
  (`pm.status_path`).
- **Φορητότητα εκ σχεδιασμού**: στατικά εκτελέσιμα musl για 10 αρχιτεκτονικές
  Linux, builds glibc για τις υπόλοιπες, συν macOS και Windows. Καμία εξάρτηση
  χρόνου εκτέλεσης, καμία έκδοση libc προς αντιστοίχιση (εκτός από τα
  τεκμηριωμένα builds glibc).

## Απαιτήσεις

- **Rust 1.78+** για μεταγλώττιση από τον πηγαίο κώδικα (MSRV δοκιμασμένη στο CI).
- **Linux**: το `/proc` χρησιμοποιείται για τα πραγματικά δεδομένα. Για τα TCP
  sockets η ουρά αποδοχής διαβάζεται από το `/proc/net/tcp`· για τα unix sockets
  εμφανίζεται μόνο το `listen.backlog` ως μέγιστο.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: δεν υπάρχει `/proc`· οι
  διεργασίες και τα sockets διαβάζονται με `ps` και `netstat`.
- **Windows**: το CLI μεταγλωττίζεται και εκτελείται (`--help`, `--mock`), αλλά
  το PHP-FPM δεν είναι διαθέσιμο στα Windows, οπότε η ζωντανή ανακάλυψη
  διεργασιών επιστρέφει σαφές σφάλμα.

## Εγκατάσταση

### Έτοιμα εκτελέσιμα

Κάθε [release](https://github.com/filoucrackeur/fpm-monitor/releases) περιέχει
ακατέργαστα εκτελέσιμα για όλα τα υποστηριζόμενα targets, ένα αρχείο συμπίεσης
ανά target (`.tar.gz` σε Unix, `.zip` σε Windows), πακέτα `.deb`/`.rpm` και μια
φόρμουλα Homebrew.

Τα αρχεία συμπίεσης είναι αυτόνομα: το καθένα περιέχει το εκτελέσιμο
`fpm-monitor`, το `LICENSE` και το `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Ή χρησιμοποιήστε απευθείας το ακατέργαστο εκτελέσιμο:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Υποστηριζόμενες πλατφόρμες και αρχιτεκτονικές

| Πλατφόρμα | Αρχιτεκτονικές | Rust targets | Διανομή |
| -------- | ------------- | ------------ | ------------ |
| Linux (στατικό musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | εκτελέσιμο, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | εκτελέσιμο, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | εκτελέσιμο, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | image OCI |

### Από τον πηγαίο κώδικα

```sh
cargo build --release
# το εκτελέσιμο βρίσκεται στο target/release/fpm-monitor
```

### Διασταυρούμενη μεταγλώττιση

Το [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) χρησιμοποιείται
από το CI για την παραγωγή όλων των εκτελέσιμων Linux από έναν μόνο runner
x86_64:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# το εκτελέσιμο βρίσκεται στο target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Αντικαταστήστε το target με οποιοδήποτε triple από τον παραπάνω πίνακα. Για
ανάπτυξη μέσα σε container `php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Πακέτα Linux (Red Hat / Debian)

Πακέτα `.deb` και `.rpm` επισυνάπτονται σε κάθε release για amd64, arm64, armhf,
armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 και sparc64. Αυτά για musl
δεν έχουν εξωτερικές εξαρτήσεις και λειτουργούν σε οποιαδήποτε πρόσφατη έκδοση
RHEL, Fedora, CentOS, Debian ή Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Σε άλλες αρχιτεκτονικές, αντικαταστήστε το `amd64`/`x86_64` με το αντίστοιχο
  όνομα πακέτου (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`,
  `ppc64`, `s390x`, `loong64`, `sparc64`).

Για να ξαναχτίσετε τοπικά τα αρχεία συμπίεσης και τα πακέτα από έναν υπάρχοντα
κατάλογο `target/`:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # απαιτεί nfpm στο PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Ένα multi-arch image δημοσιεύεται στο GitHub Container Registry για
`linux/amd64`, `linux/arm64`, `linux/ppc64le` και `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

Το `--pid=host` επιτρέπει στην παρακολούθηση να διαβάζει τις διεργασίες PHP-FPM
από το `/proc` του host· προσαρτήστε τη διαμόρφωση των pools σας με `-v` ώστε να
ανακαλύπτονται τα pools (γίνεται επίσης σάρωση των συνήθων διαδρομών `/etc`).

## Χρήση

```
fpm-monitor [OPTIONS]

Επιλογές:
  -c, --config <PATH>     αρχείο php-fpm.conf ή κατάλογος pools προς ανάλυση
  -v, --verbose           Εμφάνιση λεπτομερειών κάθε worker (pid, κατάσταση, RSS)
      --color             Εξαναγκασμός χρώματος
      --no-color          Απενεργοποίηση χρώματος
  -t, --tui               Διαδραστικό dashboard (καρτέλες, υποκαρτέλες `←`/`→`)
      --interval <SEC>    Διάστημα ανανέωσης του TUI (προεπιλογή 1)
      --lang <LANG>       Γλώσσα διεπαφής: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (προεπιλογή: en)
      --mock              Δεδομένα επίδειξης (τοπική δοκιμή)
  -h, --help              Εμφάνιση αυτής της βοήθειας
```

### Γλώσσα διεπαφής

Το `--lang` δέχεται `en` (US, προεπιλογή), `fr`, `zh`, `ar`, `es`, `it`, `ja`,
`de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi` (η μορφή `--lang=fr` γίνεται επίσης δεκτή). Εφαρμόζεται τόσο στο dashboard
TUI όσο και στην έξοδο CLI (επικεφαλίδες, σύνοψη, υπόμνημα) και στο `--help`.

### Dashboard TUI

Το TUI καταλαμβάνει ολόκληρο το τερματικό (δυναμική αλλαγή μεγέθους) και
παραμένει αποκρίσιμο: τα δεδομένα ανανεώνονται στο παρασκήνιο στο διαμορφωμένο
διάστημα (`--interval`, προεπιλογή 1 s)· ακόμη και αν μια ανάγνωση του `/proc`
είναι αργή, η απόδοση συνεχίζεται.

| Πλήκτρο       | Ενέργεια                           |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Εναλλαγή καρτέλας                |
| `←` / `→`      | Πλοήγηση υποκαρτελών (pools/config) |
| `↑` / `↓`      | Κύλιση της προβολής Παρακολούθησης |
| `q`, `Ctrl-C`  | Έξοδος                             |

## Δομή του έργου

| Αρχείο            | Ρόλος                                               |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, επιλογές, ενορχήστρωση                         |
| `src/config.rs`   | Ανακάλυψη και ανάλυση της διαμόρφωσης               |
| `src/proc.rs`     | Ανάγνωση του `/proc` (workers, RSS, κατάσταση, TCP backlog) |
| `src/data.rs`     | Συγχώνευση δεδομένων σε γραμμές πίνακα              |
| `src/logs.rs`     | Ανάγνωση του log PHP και των αργών ερωτημάτων       |
| `src/render.rs`   | Έξοδος κειμένου (πίνακας CLI)                       |
| `src/tui.rs`      | Διαδραστικό dashboard (4 καρτέλες, γραφήματα)       |
| `src/i18n.rs`     | Τοπικοποίηση (22 γλώσσες, `--lang`)                  |
| `src/term.rs`     | Ανίχνευση χρώματος / στυλ                           |
| `tests/cli.rs`    | End-to-end δοκιμές CLI (`--help`, `--mock`, i18n)   |
| `.github/workflows` | CI (δοκιμές, κάλυψη, 18 builds, smoke tests QEMU)  |

## Ανάπτυξη

```sh
cargo build          # μεταγλώττιση (debug)
cargo test           # δοκιμές μονάδας + ενσωμάτωσης
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # μορφοποίηση
```

Το CI εκτελεί τη σουίτα δοκιμών σε Linux, macOS και Windows, στο MSRV
(Rust 1.78), δημοσιεύει το badge κάλυψης, κάνει διασταυρούμενη μεταγλώττιση των
18 targets έκδοσης και εκτελεί τα εκτελέσιμα άλλων αρχιτεκτονικών υπό QEMU.

## Συνεισφορά

Οι συνεισφορές είναι ευπρόσδεκτες! Διαβάστε το [CONTRIBUTING.md](CONTRIBUTING.md)
πριν ανοίξετε ένα issue ή ένα pull request, τηρήστε τον
[Κώδικα Δεοντολογίας](CODE_OF_CONDUCT.md) μας και δείτε το [SUPPORT.md](SUPPORT.md)
για ερωτήσεις χρήσης. Αναφέρετε τα ζητήματα ασφαλείας ιδιωτικά, όπως περιγράφεται
στο [SECURITY.md](SECURITY.md).

## Άδεια

[MIT](LICENSE)
