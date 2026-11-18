<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | **Русский** | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Монитор пулов PHP-FPM, написанный на Rust (порт `fpm-monitor.c`), с интерактивной
панелью управления в терминале, вдохновлённой
[Ember](https://github.com/alexandre-daubois/ember). Он поставляется в виде
одного бинарного файла без зависимостей и работает под **Linux, macOS и
Windows** на **18 целевых архитектурах**. Интерфейс доступен на 8 языках (по
умолчанию — английский) с помощью `--lang`, а эта документация переведена на
22 языка.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — интерактивная панель управления (английский)" width="700">
</p>

## Возможности

- **Вывод CLI**: таблица пулов (pool, type, workers, running, idle, backlog,
  max_children, max_requests, память) с подробностями по каждому воркеру
  (`-v`: pid, состояние, RSS).
- **Панель TUI** (`-t`): интерактивный интерфейс с 4 вкладками.
  - **Мониторинг**: состояние пулов и их воркеров в реальном времени (память по пулу).
  - **Графики**: один большой график на пул (в стиле Ember) с подвкладками
    `←`/`→`: `running`, `workers`, `idle`, `backlog` (показываются всегда, даже
    если пусты), а также пунктирные линии порогов: `max_children` (серый),
    `max_requests` (красный), `min_spare` (розовый) и `max_spare` (фиолетовый)
    на панели `idle`.
  - **Логи**: для каждого пула — последние строки журнала PHP (`error_log`),
    медленных запросов (`slowlog`) и **журнала доступа** (`access.log`, HTTP-коды
    с подсветкой), в подвкладках.
  - **Конфигурации**: директивы, прочитанные из файлов (`global` + пулы), в подвкладках.
- Автоматическое обнаружение конфигурации (обычные расположения + директива `include`).
- Никаких сетевых вызовов: все данные читаются напрямую из `/proc` (процессы,
  RSS, состояние, очередь принятия TCP-сокетов). В macOS и BSD те же данные
  читаются из `ps` и `netstat`; настройка статуса FPM (`pm.status_path`) не требуется.
- **Переносимость по замыслу**: статические бинарные файлы musl для 10 архитектур
  Linux, сборки glibc для остальных, а также macOS и Windows. Никаких зависимостей
  времени выполнения, никакой необходимости подбирать версию libc (кроме описанных
  сборок glibc).

## Требования

- **Rust 1.78+** для сборки из исходного кода (MSRV проверяется в CI).
- **Linux**: для реальных данных используется `/proc`. Для TCP-сокетов очередь
  принятия читается из `/proc/net/tcp`; для unix-сокетов в качестве максимума
  показывается только `listen.backlog`.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: `/proc` отсутствует;
  процессы и сокеты читаются с помощью `ps` и `netstat`.
- **Windows**: CLI собирается и работает (`--help`, `--mock`), но PHP-FPM под
  Windows недоступен, поэтому обнаружение процессов в реальном времени возвращает
  явную ошибку.

## Установка

### Готовые бинарные файлы

Каждый [релиз](https://github.com/filoucrackeur/fpm-monitor/releases) содержит
бинарные файлы для всех поддерживаемых целей, архив для каждой цели (`.tar.gz`
в Unix, `.zip` в Windows), пакеты `.deb`/`.rpm` и формулу Homebrew.

Архивы самодостаточны: в каждом находятся исполняемый файл `fpm-monitor`,
`LICENSE` и `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Или используйте бинарный файл напрямую:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Поддерживаемые платформы и архитектуры

| Платформа | Архитектуры | Цели Rust | Дистрибуция |
| --------- | ----------- | --------- | ----------- |
| Linux (статический musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | бинарный файл, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | бинарный файл, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | бинарный файл, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI-образ |

### Из исходного кода

```sh
cargo build --release
# бинарный файл находится в target/release/fpm-monitor
```

### Кросс-компиляция

CI использует [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild), чтобы
собрать все бинарные файлы Linux на одном раннере x86_64:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# бинарный файл находится в target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Замените цель на любой триплет из таблицы выше. Для развёртывания в контейнере
`php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Пакеты Linux (Red Hat / Debian)

Пакеты `.deb` и `.rpm` прилагаются к каждому релизу для amd64, arm64, armhf, armel,
i386, riscv64, ppc64el, ppc64, s390x, loong64 и sparc64. Пакеты musl не имеют
внешних зависимостей и работают на любой современной версии RHEL, Fedora, CentOS,
Debian или Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- На других архитектурах замените `amd64`/`x86_64` на соответствующее имя пакета
  (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`, `s390x`,
  `loong64`, `sparc64`).

Чтобы пересобрать архивы и пакеты локально из существующего каталога `target/`:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # требуется nfpm в PATH
```

### macOS (Homebrew)

Готовая формула создаётся и прилагается к каждому релизу:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

Чтобы разместить её вместо этого как tap, переместите файл в
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`, затем выполните:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Мультиархитектурный образ публикуется в GitHub Container Registry для
`linux/amd64`, `linux/arm64`, `linux/ppc64le` и `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` позволяет монитору читать процессы PHP-FPM из `/proc` хоста;
смонтируйте конфигурацию пулов с помощью `-v`, чтобы пулы были обнаружены
(обычные пути `/etc` также проверяются).

## Использование

```
fpm-monitor [OPTIONS]

Опции:
  -c, --config <PATH>     Файл php-fpm.conf или каталог пулов для анализа
  -v, --verbose           Подробности по каждому воркеру (pid, состояние, RSS)
      --color             Принудительно включить цвет
      --no-color          Отключить цвет
  -t, --tui               Интерактивная панель (вкладки, подвкладки `←`/`→`)
      --interval <SEC>    Интервал обновления TUI (по умолчанию 1)
      --lang <LANG>       Язык интерфейса: en, fr, zh, ar, es, it, ja, de
                          (по умолчанию: en)
      --mock              Демонстрационные данные (локальный тест)
  -h, --help              Показать эту справку
```

### Язык интерфейса

`--lang` принимает `en` (США, по умолчанию), `fr`, `zh`, `ar`, `es`, `it`, `ja`,
`de` (форма `--lang=fr` также поддерживается). Параметр применяется как к панели
TUI, так и к выводу CLI (заголовки, сводка, легенда) и к `--help`.

### Панель TUI

Панель TUI занимает весь терминал (динамическое изменение размера) и остаётся
отзывчивой: данные обновляются в фоне с заданным интервалом (`--interval`, по
умолчанию 1 с); даже если чтение `/proc` замедляется, отрисовка продолжается.

| Клавиша          | Действие                                      |
| ---------------- | --------------------------------------------- |
| `1` – `4`, `Tab` | Переключение вкладки                          |
| `←` / `→`        | Навигация по подвкладкам (пулы/конфигурация)  |
| `↑` / `↓`        | Прокрутка представления «Мониторинг»          |
| `q`, `Ctrl-C`    | Выход                                          |

## Структура проекта

| Файл              | Роль                                                    |
| ----------------- | ------------------------------------------------------- |
| `src/main.rs`     | CLI, параметры, оркестрация                             |
| `src/config.rs`   | Обнаружение и разбор конфигурации                       |
| `src/proc.rs`     | Чтение `/proc` (воркеры, RSS, состояние, backlog TCP)   |
| `src/data.rs`     | Объединение данных в строки таблицы                     |
| `src/logs.rs`     | Чтение журнала PHP и медленных запросов                 |
| `src/render.rs`   | Текстовый вывод (таблица CLI)                           |
| `src/tui.rs`      | Интерактивная панель (4 вкладки, графики)               |
| `src/i18n.rs`     | Локализация (8 языков, `--lang`)                        |
| `src/term.rs`     | Определение цвета / стили                               |
| `tests/cli.rs`    | Сквозные тесты CLI (`--help`, `--mock`, i18n)           |
| `.github/workflows` | CI (тесты, покрытие, 18 сборок, smoke-тесты в QEMU)   |

## Разработка

```sh
cargo build          # сборка (debug)
cargo test           # модульные + интеграционные тесты
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # форматирование
```

CI запускает набор тестов в Linux, macOS и Windows на MSRV (Rust 1.78),
публикует значок покрытия, выполняет кросс-компиляцию 18 целей релиза и запускает
бинарные файлы для чужих архитектур в QEMU.

## Лицензия

[MIT](LICENSE)
