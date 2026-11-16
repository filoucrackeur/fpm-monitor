<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[English](README.md) | **Français** | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md)

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

Moniteur de pools PHP-FPM en Rust (port de `fpm-monitor.c`), avec un dashboard
interactif en terminal inspiré d'[Ember](https://github.com/alexandre-daubois/ember).
Il est distribué sous forme d'un binaire unique sans dépendance et fonctionne
sur **Linux, macOS et Windows**, sur **18 architectures cibles**. L'interface
est disponible en 8 langues (défaut : anglais) via `--lang`.

<p align="center">
  <img src="docs/screenshot.fr.animated.svg" alt="Tableau de bord interactif fpm-monitor (français)" width="700">
</p>

## Fonctionnalités

- **Sortie CLI** : tableau des pools (pool, type, workers, running, idle, backlog,
  max_children, max_requests, mémoire) avec détail par worker (`-v` : pid, état, RSS).
- **Dashboard TUI** (`-t`) : interface interactive avec 4 onglets.
  - **Monitoring** : état en temps réel des pools et de leurs workers (mémoire par pool).
  - **Graphiques** : un grand graphique par pool (style Ember) avec sous-onglets ← → :
    `running`, `workers`, `idle`, `backlog` (toujours visible, même vide), et des
    lignes de seuils en pointillés : `max_children` (gris), `max_requests` (rouge),
    `min_spare` (rose) et `max_spare` (violet) sur le panneau `idle`.
  - **Logs** : par pool, les dernières lignes du log PHP (`error_log`), des requêtes
    lentes (`slowlog`) et de l'**access log** (`access.log`, codes HTTP colorés),
    en sous-onglets.
  - **Configurations** : directives lues des fichiers (`global` + pools) en sous-onglets.
- Détection automatique de la configuration (emplacements usuels + directive `include`).
- Aucun appel réseau : tout est lu directement dans `/proc` (processus, RSS, état,
  file d'attente d'acceptation des sockets TCP). Sous macOS et BSD, les mêmes
  données viennent de `ps` et `netstat` ; aucune configuration de statut FPM
  (`pm.status_path`) n'est nécessaire.
- **Portable par conception** : binaires statiques musl pour 10 architectures
  Linux, builds glibc pour les autres, plus macOS et Windows. Aucune dépendance
  à l'exécution.

## Exigences

- **Rust 1.78+** pour compiler depuis les sources (MSRV testée en CI).
- **Linux** : `/proc` est utilisé pour les données réelles. Pour les sockets TCP,
  le backlog est lu dans `/proc/net/tcp` (file d'attente d'acceptation) ; pour
  les sockets unix, seul `listen.backlog` est affiché comme maximum.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly** : pas de `/proc` ; les
  processus et sockets sont lus avec `ps` et `netstat`.
- **Windows** : la CLI compile et fonctionne (`--help`, `--mock`), mais PHP-FPM
  n'existe pas sous Windows : la découverte des processus renvoie une erreur
  explicite.

## Installation

### Binaires prêts à l'emploi

Chaque [release](https://github.com/filoucrackeur/fpm-monitor/releases) contient
les binaires de toutes les cibles, les paquets `.deb`/`.rpm` et une formule
Homebrew. Téléchargez le fichier correspondant à votre architecture, rendez-le
exécutable et lancez-le :

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Plateformes et architectures prises en charge

| Plateforme | Architectures | Cibles Rust | Distribution |
| ---------- | ------------- | ----------- | ------------ |
| Linux (musl statique) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | binaire, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | binaire, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | binaire, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | image OCI |

### Depuis les sources

```sh
cargo build --release
# le binaire est dans target/release/fpm-monitor
```

### Compilation croisée

La CI utilise [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
pour produire tous les binaires Linux depuis un runner x86_64 :

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# le binaire est dans target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Remplacez la cible par n'importe quel triple du tableau ci-dessus. Pour un
déploiement dans un container `php:*-fpm-alpine` :

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Paquets Linux (Red Hat / Debian)

Des paquets `.deb` et `.rpm` sont joints à chaque release pour amd64, arm64,
armhf, i386, riscv64, ppc64el, ppc64, s390x, loong64 et sparc64. Les paquets
musl n'ont aucune dépendance externe et fonctionnent sur toute version récente
de RHEL, Fedora, CentOS, Debian ou Ubuntu.

- **Red Hat, Fedora, CentOS** :
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu** :
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Sur les autres architectures, remplacez `amd64`/`x86_64` par le nom du paquet
  correspondant (`arm64`, `armhf`, `i386`, `riscv64`, `ppc64el`, `ppc64`,
  `s390x`, `loong64`, `sparc64`).

### macOS (Homebrew)

Une formule prête à l'emploi est générée et jointe à chaque release :

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

Pour l'héberger comme tap, placez le fichier dans
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`, puis :

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Une image multi-arch est publiée sur le GitHub Container Registry pour
`linux/amd64`, `linux/arm64`, `linux/ppc64le` et `linux/riscv64` :

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` permet au moniteur de lire les processus PHP-FPM depuis le
`/proc` hôte ; montez votre configuration de pools avec `-v` pour que les
pools soient découverts (les chemins `/etc` habituels sont aussi analysés).

## Utilisation

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     Fichier php-fpm.conf ou dossier de pools à analyser
  -v, --verbose           Affiche le détail de chaque worker (pid, état, RSS)
      --color             Force la couleur
      --no-color          Désactive la couleur
  -t, --tui               Dashboard interactif (onglets, sous-onglets ← →)
      --interval <SEC>    Intervalle de rafraîchissement du TUI (défaut 1)
      --lang <LANG>       Langue de l'interface : en, fr, zh, ar, es, it, ja, de
                          (défaut : en)
      --mock              Données de démonstration (test local)
  -h, --help              Affiche l'aide
```

### Langue de l'interface

`--lang` accepte `en` (US, défaut), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`
(la forme `--lang=fr` est aussi acceptée). Elle s'applique au dashboard TUI comme
à la sortie CLI (en-têtes, résumé, légende) et à `--help`.

### Dashboard TUI

Le TUI occupe toute la taille du terminal (redimensionnement dynamique) et reste
réactif : les données sont rafraîchies en arrière-plan à l'intervalle défini
(`--interval`, défaut 1 s) ; même si une lecture `/proc` est lente, le rendu
continue de tourner.

| Touche        | Action                                |
| ------------- | ------------------------------------- |
| `1` – `4`, `Tab` | Changer d'onglet                  |
| `←` / `→`     | Naviguer les sous-onglets (pools/config) |
| `↑` / `↓`     | Défiler la vue Monitoring             |
| `q`, `Ctrl-C` | Quitter                               |

## Structure du projet

| Fichier            | Rôle                                        |
| ------------------ | ------------------------------------------- |
| `src/main.rs`      | CLI, options, orchestration                 |
| `src/config.rs`    | Découverte et parsing de la configuration    |
| `src/proc.rs`      | Lecture de `/proc` (workers, RSS, état, backlog TCP) |
| `src/data.rs`      | Fusion des données en lignes de table        |
| `src/logs.rs`      | Lecture des logs PHP et des requêtes lentes  |
| `src/render.rs`    | Sortie texte (tableau CLI)                   |
| `src/tui.rs`       | Dashboard interactif (4 onglets, graphiques) |
| `src/i18n.rs`      | Localisation (8 langues, `--lang`)           |
| `src/term.rs`      | Détection couleur / styles                   |

## Développement

```sh
cargo build          # compilation (debug)
cargo test           # tests unitaires + d'intégration
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # formatage
```

La CI exécute les tests sous Linux, macOS et Windows, sur la MSRV (Rust 1.78),
publie le badge de couverture, compile les 18 cibles de release et exécute
les binaires d'architectures étrangères sous QEMU.

## Licence

[MIT](LICENSE)
