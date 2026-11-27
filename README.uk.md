<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/version.json)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/downloads.json)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/issues.json)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/last-commit.json)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | **Українська** | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Монітор пулів PHP-FPM, написаний на Rust (порт `fpm-monitor.c`), з
інтерактивною панеллю в терміналі, натхненною [Ember](https://github.com/alexandre-daubois/ember).
Постачається як єдиний бінарний файл без залежностей і працює на **Linux, macOS
та Windows** для **18 цільових архітектур**. Інтерфейс доступний 22 мовами
(за замовчуванням — англійська) через `--lang`, а ця документація перекладена
22 мовами.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — інтерактивна панель (українська)" width="700">
</p>

## Функції

- **Вивід CLI**: таблиця пулів (pool, type, workers, running, idle, backlog,
  max_children, max_requests, memory) з деталями по кожному воркеру (`-v`: pid, state, RSS).
- **Панель TUI** (`-t`): інтерактивний інтерфейс із 4 вкладками.
  - **Моніторинг**: стан пулів та їхніх воркерів у реальному часі (пам'ять на пул).
  - **Графіки**: один великий графік на пул (у стилі Ember) з підвкладками `←`/`→`:
    `running`, `workers`, `idle`, `backlog` (показуються завжди, навіть порожні), а також
    пунктирні лінії порогів: `max_children` (сіра), `max_requests` (червона),
    `min_spare` (рожева) і `max_spare` (фіолетова) на панелі `idle`.
  - **Журнали**: для кожного пулу останні рядки журналу PHP (`error_log`), повільних
    запитів (`slowlog`) і **журналу доступу** (`access.log`, коди HTTP з підсвічуванням),
    у підвкладках.
  - **Конфігурації**: директиви, прочитані з файлів (`global` + пули), у підвкладках.
- **Веб-панель** (`--watch`): ті самі чотири подання у браузері, які віддає
  HTTP-сервер без залежностей на `http://127.0.0.1:8471`. Порт задається через
  `--port`; дані збираються у фоні з інтервалом `--interval`, а сторінка зберігає
  60 точок історії для графіків.
- Автоматичне виявлення конфігурації (звичайні розташування + директива `include`).
- Жодних мережевих викликів: усе читається безпосередньо з `/proc` (процеси, RSS,
  стан, черга прийняття TCP-сокетів). У macOS і BSD ті самі дані читаються з
  `ps` та `netstat`; конфігурація статусу FPM (`pm.status_path`) не потрібна.
- **Портативність за задумом**: статичні musl-збірки для 10 архітектур Linux,
  glibc-збірки для решти, а також macOS і Windows. Жодних залежностей під час
  виконання, жодної версії libc, яку потрібно узгоджувати (окрім задокументованих
  glibc-збірок).

## Вимоги

- **Rust 1.78+** для збирання з вихідного коду (MSRV перевіряється в CI).
- **Linux**: для реальних даних використовується `/proc`. Для TCP-сокетів черга
  прийняття читається з `/proc/net/tcp`; для unix-сокетів як максимум показується
  лише `listen.backlog`.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: `/proc` відсутній; процеси
  та сокети читаються за допомогою `ps` і `netstat`.
- **Windows**: CLI збирається й працює (`--help`, `--mock`), але PHP-FPM на Windows
  недоступний, тому виявлення живих процесів повертає явну помилку.

## Встановлення

### Готові бінарні файли

Кожен [випуск](https://github.com/filoucrackeur/fpm-monitor/releases) містить
сирі бінарні файли для всіх підтримуваних цілей, архів для кожної цілі
(`.tar.gz` на Unix, `.zip` на Windows), пакунки `.deb`/`.rpm` і формулу Homebrew.

Архіви самодостатні: кожен містить виконуваний файл `fpm-monitor`, `LICENSE`
і `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Або скористайтеся сирим бінарним файлом напряму:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Підтримувані платформи й архітектури

| Платформа | Архітектури | Цілі Rust | Розповсюдження |
| -------- | ------------- | ------------ | ------------ |
| Linux (статичний musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | бінарний файл, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | бінарний файл, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | бінарний файл, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI-образ |

### З вихідного коду

```sh
cargo build --release
# бінарний файл розташований у target/release/fpm-monitor
```

### Кроскомпіляція

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) використовується
в CI, щоб створити всі бінарні файли для Linux на одному раннері x86_64:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# бінарний файл розташований у target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Замініть ціль на будь-який триплет із таблиці вище. Щоб розгорнути всередині
контейнера `php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Пакунки Linux (Red Hat / Debian)

Пакунки `.deb` і `.rpm` додаються до кожного випуску для amd64, arm64, armhf,
armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 і sparc64. Версії musl не
мають зовнішніх залежностей і працюють на будь-якій сучасній версії RHEL,
Fedora, CentOS, Debian чи Ubuntu.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Для інших архітектур замініть `amd64`/`x86_64` на відповідну назву пакунка
  (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`,
  `s390x`, `loong64`, `sparc64`).

Щоб перезібрати архіви й пакунки локально з наявного каталогу `target/`:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # потрібен nfpm у PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Багатоархітектурний образ публікується в GitHub Container Registry для
`linux/amd64`, `linux/arm64`, `linux/ppc64le` та `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` дозволяє монітору читати процеси PHP-FPM із `/proc` хоста;
змонтуйте конфігурацію пулів FPM за допомогою `-v`, щоб пули було виявлено
(звичайні шляхи `/etc` також скануються).

## Використання

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     файл php-fpm.conf або каталог пулів для аналізу
  -v, --verbose           показувати деталі кожного воркера (pid, state, RSS)
      --color             примусово вмикати колір
      --no-color          вимкнути колір
  -t, --tui               інтерактивна панель (вкладки, підвкладки `←`/`→`)
      --interval <SEC>    інтервал оновлення TUI (за замовчуванням 1)
      --watch             Веб-панель замість подання CLI
      --port <PORT>       Порт веб-панелі (типово 8471)
      --lang <LANG>       мова інтерфейсу: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (за замовчуванням: en)
      --mock              демонстраційні дані (локальний тест)
  -h, --help              показати цю довідку
```

### Мова інтерфейсу

`--lang` приймає `en` (США, за замовчуванням), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
(форма `--lang=fr` також приймається). Це застосовується як до панелі TUI, так
і до виводу CLI (заголовки, підсумок, легенда) та `--help`.

### Панель TUI

Панель TUI займає весь термінал (динамічна зміна розміру) і працює плавно:
дані оновлюються у фоні з налаштованим інтервалом (`--interval`, за замовчуванням 1 с);
навіть якщо читання `/proc` повільне, відображення не зупиняється.

| Клавіша            | Дія                             |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Перемкнути вкладку                       |
| `←` / `→`      | Навігація підвкладками (пули/конфігурація)   |
| `↑` / `↓`      | Прокрутка екрана «Моніторинг»         |
| `q`, `Ctrl-C`  | Вихід                               |

### Веб-панель

`--watch` віддає ті самі дані як односторінковий вебзастосунок замість виводу
таблиці CLI:

```sh
fpm-monitor --watch                 # http://127.0.0.1:8471
fpm-monitor --watch --port 9000     # власний порт
fpm-monitor --watch --mock          # демодані без PHP-FPM
```

Сервер слухає лише локальний інтерфейс, без сторонніх залежностей, а сторінка
запитує `/api/state` для пулів, відомостей про worker'ів, журналів, директив
конфігурації та останніх 60 вибірок. Дані збираються у фоні з інтервалом
`--interval` (типово 1 с), тому повільне сканування ніколи не блокує запит.
Чотири вкладки — **Моніторинг**, **Графіки**, **Журнали** та
**Конфігурації**; `1`–`4` і `←`/`→` вибирають вкладки та підвкладки.

`--port` приймає `1`–`65535` (також працює `--port=9000`); будь-яке інше
значення завершується з кодом `2`. Сервер ніколи не робить вихідних запитів.

## Структура проєкту

| Файл              | Роль                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, параметри, оркестрація                         |
| `src/config.rs`   | Виявлення й розбір конфігурації                 |
| `src/proc.rs`     | Читання `/proc` (воркери, RSS, стан, TCP backlog)  |
| `src/data.rs`     | Об'єднання даних у рядки таблиці                        |
| `src/logs.rs`     | Читання журналу PHP і повільних запитів                    |
| `src/render.rs`   | Текстовий вивід (таблиця CLI)                             |
| `src/tui.rs`      | Інтерактивна панель (4 вкладки, графіки)              |
| `src/web.rs`       | Веб-сервер, фонова вибірка, JSON-знімок               |
| `src/web/index.html` | Вбудована односторінкова веб-панель                |
| `src/i18n.rs`     | Локалізація (22 мов, `--lang`)                |
| `src/term.rs`     | Визначення кольорів / стилі                            |
| `tests/cli.rs`    | Наскрізні тести CLI (`--help`, `--mock`, i18n)      |
| `.github/workflows` | CI (тести, покриття, 18 збірок, дим-тести QEMU) |

## Розробка

```sh
cargo build          # збирання (debug)
cargo test           # модульні + інтеграційні тести
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # форматування
```

CI запускає набір тестів на Linux, macOS і Windows, на MSRV (Rust 1.78),
публікує бейдж покриття, кроскомпілює 18 цільових збірок і виконує
бінарні файли для інших архітектур під QEMU.

## Участь у проєкті

Ми вітаємо внесок у проєкт! Прочитайте [CONTRIBUTING.md](CONTRIBUTING.md), перш
ніж відкривати issue або pull request, дотримуйтеся нашого
[Кодексу поведінки](CODE_OF_CONDUCT.md), а щодо запитань про використання див.
[SUPPORT.md](SUPPORT.md). Про проблеми безпеки повідомляйте конфіденційно, як
описано в [SECURITY.md](SECURITY.md).

## Ліцензія

[MIT](LICENSE)
