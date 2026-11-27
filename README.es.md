<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/version.json)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/downloads.json)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/issues.json)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/last-commit.json)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | **Español** | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Monitor de pools PHP-FPM escrito en Rust (un port de `fpm-monitor.c`), con un
panel interactivo de terminal inspirado en [Ember](https://github.com/alexandre-daubois/ember).
Se distribuye como un único binario sin dependencias y funciona en **Linux, macOS
y Windows**, en **18 arquitecturas de destino**. La interfaz está disponible en
22 idiomas (predeterminado: inglés) mediante `--lang`, y esta documentación está
traducida a 22 idiomas.

<p align="center">
  <img src="docs/screenshot.es.animated.svg" alt="fpm-monitor TUI — panel interactivo (español)" width="700">
</p>

## Características

- **Salida CLI**: tabla de pools (pool, tipo, workers, running, idle, backlog,
  max_children, max_requests, memoria) con el detalle de cada worker (`-v`: pid,
  estado, RSS).
- **Panel TUI** (`-t`): interfaz interactiva con 4 pestañas.
  - **Monitoreo**: estado en tiempo real de los pools y de sus workers
    (memoria por pool).
  - **Gráficas**: un gráfico grande por pool (estilo Ember) con subpestañas
    `←`/`→`: `running`, `workers`, `idle`, `backlog` (siempre visibles, aunque
    estén vacías), además de líneas de umbral discontinuas: `max_children` (gris),
    `max_requests` (rojo), `min_spare` (rosa) y `max_spare` (morado) en el panel
    `idle`.
  - **Registros**: por pool, las últimas líneas del log de PHP (`error_log`), de
    las consultas lentas (`slowlog`) y del **log de acceso** (`access.log`, con
    los códigos HTTP coloreados), en subpestañas.
  - **Configuración**: directivas leídas de los archivos (`global` + pools) en
    subpestañas.
- **Panel web** (`--watch`): las mismas cuatro vistas en el navegador, servidas
  en `http://127.0.0.1:8471` por un servidor HTTP sin dependencias. El puerto se
  define con `--port`; los datos se muestrean en segundo plano con `--interval` y
  la página conserva 60 puntos de historial para sus gráficos.
- Descubrimiento automático de la configuración (ubicaciones habituales +
  directiva `include`).
- Ninguna llamada de red: todo se lee directamente de `/proc` (procesos, RSS,
  estado, cola de aceptación de los sockets TCP). En macOS y BSD, los mismos
  datos se leen de `ps` y `netstat`; no hace falta ninguna configuración de
  estado de FPM (`pm.status_path`).
- **Portátil por diseño**: binarios estáticos musl para 10 arquitecturas de
  Linux, compilaciones glibc para las demás, además de macOS y Windows. Sin
  dependencias en tiempo de ejecución ni versión de libc que hacer coincidir
  (salvo las compilaciones glibc documentadas).

## Requisitos

- **Rust 1.78+** para compilar desde el código fuente (la MSRV se prueba en CI).
- **Linux**: se usa `/proc` para obtener datos reales. Para los sockets TCP, la
  cola de aceptación se lee de `/proc/net/tcp`; para los sockets unix solo se
  muestra `listen.backlog` como máximo.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: no hay `/proc`; los
  procesos y los sockets se leen con `ps` y `netstat`.
- **Windows**: la CLI compila y se ejecuta (`--help`, `--mock`), pero PHP-FPM no
  existe en Windows, así que el descubrimiento de procesos en vivo devuelve un
  error explícito.

## Instalación

### Binarios precompilados

Cada [versión publicada](https://github.com/filoucrackeur/fpm-monitor/releases)
contiene los binarios sin empaquetar de todas las plataformas admitidas, un
archivo por plataforma (`.tar.gz` en Unix, `.zip` en Windows), paquetes
`.deb`/`.rpm` y una fórmula de Homebrew.

Los archivos comprimidos son autónomos: cada uno incluye el ejecutable
`fpm-monitor`, `LICENSE` y `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

O utilice directamente el binario sin empaquetar:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Plataformas y arquitecturas admitidas

| Plataforma | Arquitecturas | Objetivos de Rust | Distribución |
| ---------- | ------------- | ----------------- | ------------ |
| Linux (musl estático) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | binario, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | binario, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | binario, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | imagen OCI |

### Desde el código fuente

```sh
cargo build --release
# el binario está en target/release/fpm-monitor
```

### Compilación cruzada

La CI usa [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) para
generar todos los binarios de Linux desde un único runner x86_64:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# el binario está en target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Sustituya el objetivo por cualquier triple de la tabla anterior. Para desplegar
dentro de un contenedor `php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Paquetes de Linux (Red Hat / Debian)

Los paquetes `.deb` y `.rpm` se adjuntan a cada versión publicada para amd64,
arm64, armhf, armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 y sparc64.
Los paquetes musl no tienen ninguna dependencia externa y funcionan en cualquier
versión reciente de RHEL, Fedora, CentOS, Debian o Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- En las demás arquitecturas, sustituya `amd64`/`x86_64` por el nombre del
  paquete correspondiente (`arm64`, `armhf`, `armel`, `i386`, `riscv64`,
  `ppc64el`, `ppc64`, `s390x`, `loong64`, `sparc64`).

Para reconstruir localmente los archivos comprimidos y los paquetes a partir de
un directorio `target/` existente:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # requiere nfpm en PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Se publica una imagen multiarquitectura en GitHub Container Registry para
`linux/amd64`, `linux/arm64`, `linux/ppc64le` y `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` permite al monitor leer los procesos PHP-FPM del `/proc` del
anfitrión; monte su configuración de pools con `-v` para que se descubran los
pools (también se examinan las rutas habituales de `/etc`).

## Uso

```
fpm-monitor [OPTIONS]

Opciones:
  -c, --config <PATH>     Archivo php-fpm.conf o directorio de pools a analizar
  -v, --verbose           Muestra el detalle de cada worker (pid, estado, RSS)
      --color             Fuerza el color
      --no-color          Desactiva el color
  -t, --tui               Panel interactivo (pestañas, subpestañas `←`/`→`)
      --interval <SEC>    Intervalo de actualización del TUI (predeterminado 1)
      --watch             Panel web en lugar de la vista CLI
      --port <PORT>       Puerto del panel web (8471 por defecto)
      --lang <LANG>       Idioma de la interfaz: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (predeterminado: en)
      --mock              Datos de demostración (prueba local)
  -h, --help              Muestra esta ayuda
```

### Idioma de la interfaz

`--lang` acepta `en` (US, predeterminado), `fr`, `zh`, `ar`, `es`, `it`, `ja`,
`de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi` (también se acepta la forma `--lang=fr`). Se aplica al panel TUI, a la
salida de la CLI (encabezados, resumen, leyenda) y a `--help`.

### Panel TUI

El TUI ocupa toda la terminal (redimensionado dinámico) y se mantiene fluido:
los datos se actualizan en segundo plano con el intervalo configurado
(`--interval`, 1 s por defecto); aunque una lectura de `/proc` sea lenta, el
dibujado continúa.

| Tecla          | Acción                             |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Cambiar de pestaña               |
| `←` / `→`      | Navegar por las subpestañas (pools/config) |
| `↑` / `↓`      | Desplazar la vista de Monitoreo |
| `q`, `Ctrl-C`  | Salir                              |

### Panel web

`--watch` sirve los mismos datos como una aplicación web de una sola página en
lugar de imprimir la tabla CLI:

```sh
fpm-monitor --watch                 # http://127.0.0.1:8471
fpm-monitor --watch --port 9000     # puerto personalizado
fpm-monitor --watch --mock          # datos de demostración sin PHP-FPM
```

El servidor escucha solo en la interfaz local, sin ninguna dependencia externa, y
la página consulta `/api/state` para los pools, el detalle de los workers, los
registros, las directivas de configuración y las últimas 60 muestras. Los datos
se recopilan en segundo plano con `--interval` (1 s por defecto), así que un
escaneo lento nunca bloquea una petición. Las cuatro pestañas son
**Monitorización**, **Gráficos**, **Registros** y **Configuraciones**; `1`–`4` y
`←`/`→` seleccionan pestañas y subpestañas.

`--port` acepta `1`–`65535` (`--port=9000` también funciona); cualquier otro
valor sale con el código `2`. El servidor nunca realiza peticiones salientes.

## Estructura del proyecto

| Archivo           | Función                                             |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, opciones, orquestación                         |
| `src/config.rs`   | Descubrimiento y análisis de la configuración       |
| `src/proc.rs`     | Lectura de `/proc` (workers, RSS, estado, backlog TCP) |
| `src/data.rs`     | Fusión de los datos en filas de tabla               |
| `src/logs.rs`     | Lectura del log de PHP y de las consultas lentas    |
| `src/render.rs`   | Salida de texto (tabla CLI)                         |
| `src/tui.rs`      | Panel interactivo (4 pestañas, gráficas)            |
| `src/web.rs`       | Servidor web, muestreo en segundo plano, instantánea JSON |
| `src/web/index.html` | Panel web de una sola página integrado            |
| `src/i18n.rs`     | Localización (22 idiomas, `--lang`)                  |
| `src/term.rs`     | Detección de color / estilos                        |
| `tests/cli.rs`    | Pruebas de extremo a extremo de la CLI (`--help`, `--mock`, i18n) |
| `.github/workflows` | CI (pruebas, cobertura, 18 compilaciones, pruebas de humo en QEMU) |

## Desarrollo

```sh
cargo build          # compilación (debug)
cargo test           # pruebas unitarias + de integración
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # formato
```

La CI ejecuta la batería de pruebas en Linux, macOS y Windows, sobre la MSRV
(Rust 1.78), publica la insignia de cobertura, compila de forma cruzada los 18
objetivos de publicación y ejecuta los binarios de arquitecturas extranjeras
bajo QEMU.

## Contribuir

¡Las contribuciones son bienvenidas! Lee [CONTRIBUTING.md](CONTRIBUTING.md) antes
de abrir una issue o un pull request, respeta nuestro
[Código de conducta](CODE_OF_CONDUCT.md) y consulta [SUPPORT.md](SUPPORT.md) para
dudas de uso. Informa de los problemas de seguridad de forma privada como se
describe en [SECURITY.md](SECURITY.md).

## Licencia

[MIT](LICENSE)
