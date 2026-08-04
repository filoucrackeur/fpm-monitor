# fpm-monitor

[English](README.md) | **Français** | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md)

[![CI](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml) [![Release](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/php-fpm-monitor?sort=semver)](https://github.com/filoucrackeur/php-fpm-monitor/releases) [![codecov](https://img.shields.io/codecov/c/github/filoucrackeur/php-fpm-monitor)](https://codecov.io/github/filoucrackeur/php-fpm-monitor) [![Maintainability](https://img.shields.io/codeclimate/maintainability/filoucrackeur/php-fpm-monitor)](https://codeclimate.com/github/filoucrackeur/php-fpm-monitor) [![License](https://img.shields.io/github/license/filoucrackeur/php-fpm-monitor)](LICENSE) ![Static](https://img.shields.io/badge/static-musl-blue) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/php-fpm-monitor)](https://github.com/filoucrackeur/php-fpm-monitor/commits/main)

Moniteur de pools PHP-FPM en Rust (port de `fpm-monitor.c`), avec un dashboard
interactif en terminal inspiré d'[Ember](https://github.com/alexandre-daubois/ember).
L'interface est disponible en 8 langues (défaut : anglais) via `--lang`.

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
  file d'attente d'acceptation des sockets TCP).

## Exigences

- Rust (stable) pour compiler.
- **Linux avec `/proc`** pour les données réelles. Aucune configuration de statut
  FPM (`pm.status_path`) n'est nécessaire : les données viennent de `/proc` et de
  la configuration. Pour les sockets TCP, le backlog est lu dans `/proc/net/tcp`
  (file d'attente d'acceptation) ; pour les sockets unix, seul `listen.backlog`
  est affiché comme maximum.

## Installation

```sh
cargo build --release
# le binaire est dans target/release/fpm-monitor
```

Pour un déploiement dans un container `php:*-fpm-alpine` (musl, aarch64) :

```sh
RUSTC=$HOME/.cargo/bin/rustc ~/.cargo/bin/cargo build --release --target aarch64-unknown-linux-musl
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Paquets Linux (Red Hat / Debian)

Des paquets `.deb` et `.rpm` statiques (musl) sont joints à chaque
[release](https://github.com/filoucrackeur/php-fpm-monitor/releases). Sans dépendance externe, ils
fonctionnent sur toute version récente de RHEL, Fedora, CentOS, Debian ou Ubuntu
(x86_64 et ARM64).

- **Red Hat, Fedora, CentOS** :
  ```sh
  sudo dnf install fpm-monitor_<version>_amd64.rpm
  ```
- **Debian, Ubuntu** :
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Sur ARM64, utilisez les paquets `arm64` à la place.

### macOS (Homebrew)

Une formule prête à l'emploi est générée et jointe à chaque release :

```sh
brew install https://github.com/filoucrackeur/php-fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

Pour l'héberger comme tap, placez le fichier dans
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`, puis :

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

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
cargo test           # exécution des tests unitaires
cargo clippy         # lint
cargo fmt --check    # formatage
```

## Licence

[MIT](LICENSE)
