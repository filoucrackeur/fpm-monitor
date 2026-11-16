<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | **Español** | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md)

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

Monitor de pools PHP-FPM en Rust (un port de `fpm-monitor.c`), con un panel
interactivo en terminal inspirado en [Ember](https://github.com/alexandre-daubois/ember).
La interfaz está disponible en 8 idiomas (por defecto: inglés) mediante `--lang`.

<p align="center">
  <img src="docs/screenshot.es.animated.svg" alt="Panel interactivo fpm-monitor TUI (español)" width="700">
</p>

## Funcionalidades

- **Salida CLI**: tabla de pools (pool, type, workers, running, idle, backlog,
  max_children, max_requests, memoria) con el detalle por worker (`-v`: pid, estado, RSS).
- **Panel TUI** (`-t`): interfaz interactiva con 4 pestañas.
  - **Monitoreo**: estado en tiempo real de los pools y sus workers (memoria por pool).
  - **Gráficas**: una gráfica grande por pool (estilo Ember) con subpestañas ← →:
    `running`, `workers`, `idle`, `backlog` (siempre visible, incluso vacío), y
    líneas de umbral discontinuas: `max_children` (gris), `max_requests` (rojo),
    `min_spare` (rosa) y `max_spare` (violeta) en el panel `idle`.
  - **Registros**: por pool, las últimas líneas del log de PHP (`error_log`), las
    consultas lentas (`slowlog`) y el **log de acceso** (`access.log`, códigos HTTP
    coloreados), en subpestañas.
  - **Configuración**: directivas leídas de los archivos (`global` + pools) en subpestañas.
- Detección automática de la configuración (ubicaciones habituales + directiva `include`).
- Sin llamadas de red: todo se lee directamente de `/proc` (procesos, RSS, estado,
  cola de aceptación de los sockets TCP).

## Requisitos

- Rust 1.78+ para compilar (MSRV probada en CI).
- Además: macOS/FreeBSD/BSD (`ps`/`netstat`) y compilaciones para Windows; cada release incluye binarios para 18 arquitecturas.
- **Linux con `/proc`** para los datos reales. No se necesita configuración de
  estado FPM (`pm.status_path`): los datos provienen de `/proc` y de la
  configuración. Para los sockets TCP, el backlog se lee de `/proc/net/tcp` (cola
  de aceptación); para los sockets unix, solo se muestra `listen.backlog` como máximo.

## Instalación

```sh
cargo build --release
# el binario está en target/release/fpm-monitor
```

Para desplegar dentro de un contenedor `php:*-fpm-alpine` (musl, aarch64):

```sh
RUSTC=$HOME/.cargo/bin/rustc ~/.cargo/bin/cargo build --release --target aarch64-unknown-linux-musl
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Paquetes Linux (Red Hat / Debian)

Cada [release](https://github.com/filoucrackeur/fpm-monitor/releases) incluye paquetes `.deb` y
`.rpm` estáticos (musl). Sin dependencias externas, funcionan en cualquier versión
reciente de RHEL, Fedora, CentOS, Debian o Ubuntu (x86_64 y ARM64).

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor_<version>_amd64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- En ARM64, usa los paquetes `arm64` en su lugar.

### macOS (Homebrew)

Cada release genera y adjunta una fórmula lista para usar:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

Para alojarla como tap, coloca el archivo en
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb` y luego:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Una imagen multi-arquitectura (`linux/amd64`, `linux/arm64`) se publica en
GitHub Container Registry:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` permite al monitor leer los procesos PHP-FPM del `/proc` del
anfitrión; monta tu configuración de pools con `-v` para que se descubran
(también se escanean las rutas habituales de `/etc`).

## Uso

```
fpm-monitor [OPCIONES]

Opciones:
  -c, --config <RUTA>    Archivo php-fpm.conf o carpeta de pools a analizar
  -v, --verbose          Muestra el detalle de cada worker (pid, estado, RSS)
      --color            Forzar color
      --no-color         Desactivar color
  -t, --tui              Panel interactivo (pestañas, subpestañas ← →)
      --interval <SEG>   Intervalo de refresco del TUI (por defecto 1)
      --lang <LANG>      Idioma de la interfaz: en, fr, zh, ar, es, it, ja, de
                         (por defecto: en)
      --mock             Datos de demostración (prueba local)
  -h, --help             Muestra esta ayuda
```

### Idioma de la interfaz

`--lang` acepta `en` (US, por defecto), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`
(también se acepta la forma `--lang=fr`). Se aplica al panel TUI y también a la
salida CLI (encabezados, resumen, leyenda) y a `--help`.

### Panel TUI

El TUI ocupa todo el terminal (cambio de tamaño dinámico) y sigue siendo
responsivo: los datos se refrescan en segundo plano al intervalo configurado
(`--interval`, por defecto 1 s); incluso si una lectura de `/proc` es lenta, el
renderizado continúa.

| Tecla          | Acción                              |
| -------------- | ----------------------------------- |
| `1` – `4`, `Tab` | Cambiar de pestaña                |
| `←` / `→`      | Navegar subpestañas (pools/config)  |
| `↑` / `↓`      | Desplazar la vista de monitoreo     |
| `q`, `Ctrl-C`  | Salir                               |

## Estructura del proyecto

| Archivo            | Rol                                        |
| ------------------ | ------------------------------------------ |
| `src/main.rs`      | CLI, opciones, orquestación                |
| `src/config.rs`    | Descubrimiento y análisis de la configuración |
| `src/proc.rs`      | Lectura de `/proc` (workers, RSS, estado, backlog TCP) |
| `src/data.rs`      | Fusión de datos en filas de tabla          |
| `src/logs.rs`      | Lectura del log de PHP y consultas lentas  |
| `src/render.rs`    | Salida de texto (tabla CLI)                |
| `src/tui.rs`       | Panel interactivo (4 pestañas, gráficas)   |
| `src/i18n.rs`      | Localización (8 idiomas, `--lang`)         |
| `src/term.rs`      | Detección de color / estilos               |

## Desarrollo

```sh
cargo build          # compilar (debug)
cargo test           # ejecutar las pruebas unitarias
cargo clippy         # lint
cargo fmt --check    # formato
```

## Licencia

[MIT](LICENSE)
