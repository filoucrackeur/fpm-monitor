<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | **简体中文** | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

用 Rust 编写的 PHP-FPM 进程池监控工具（`fpm-monitor.c` 的移植版本），并带有一个受
[Ember](https://github.com/alexandre-daubois/ember) 启发的交互式终端仪表盘。
它以单个无依赖的二进制文件分发，可在 **Linux、macOS 和 Windows** 上运行，覆盖
**18 种目标架构**。界面通过 `--lang` 提供 8 种语言（默认：英语），本文档也已翻译为
22 种语言。

<p align="center">
  <img src="docs/screenshot.zh.animated.svg" alt="fpm-monitor TUI — 交互式仪表盘（简体中文）" width="700">
</p>

## 功能

- **CLI 输出**：进程池表格（pool、type、workers、running、idle、backlog、
  max_children、max_requests、内存），并提供每个 worker 的详细信息（`-v`：pid、状态、RSS）。
- **TUI 仪表盘**（`-t`）：包含 4 个标签页的交互式界面。
  - **监控**：进程池及其 worker 的实时状态（每个进程池的内存占用）。
  - **图表**：每个进程池一个大型图表（Ember 风格），带有 `←`/`→` 子标签页：
    `running`、`workers`、`idle`、`backlog`（始终显示，即使为空），以及
    虚线阈值线：`max_children`（灰色）、`max_requests`（红色），并在 `idle`
    面板上显示 `min_spare`（粉色）和 `max_spare`（紫色）。
  - **日志**：按进程池分别显示 PHP 日志（`error_log`）、慢查询（`slowlog`）和
    **访问日志**（`access.log`，HTTP 状态码着色）的最新行，以子标签页呈现。
  - **配置**：以子标签页显示从文件中读取的指令（`global` + 进程池）。
- 自动发现配置（常见位置 + `include` 指令）。
- 无任何网络调用：所有数据都直接从 `/proc` 读取（进程、RSS、状态、TCP 套接字接受队列）。
  在 macOS 和 BSD 上，相同数据从 `ps` 和 `netstat` 读取；无需任何 FPM 状态配置
  （`pm.status_path`）。
- **天生便携**：为 10 种 Linux 架构提供静态 musl 二进制文件，其余架构提供 glibc
  构建，另含 macOS 和 Windows 版本。无运行时依赖，无需匹配 libc 版本
  （文档中注明的 glibc 构建除外）。

## 环境要求

- **Rust 1.78+**，用于从源码编译（MSRV 已在 CI 中测试）。
- **Linux**：使用 `/proc` 获取真实数据。对于 TCP 套接字，接受队列从
  `/proc/net/tcp` 读取；对于 unix 套接字，仅将 `listen.backlog` 显示为上限。
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**：没有 `/proc`；进程和套接字
  通过 `ps` 和 `netstat` 读取。
- **Windows**：CLI 可以编译并运行（`--help`、`--mock`），但 Windows 上不提供
  PHP-FPM，因此实时进程发现会返回明确的错误。

## 安装

### 预编译二进制文件

每个 [release](https://github.com/filoucrackeur/fpm-monitor/releases) 均包含所有
受支持目标的原始二进制文件、每个目标一个归档包（Unix 上为 `.tar.gz`，Windows 上为
`.zip`）、`.deb`/`.rpm` 软件包以及 Homebrew formula。

这些归档包是自包含的：每个都包含 `fpm-monitor` 可执行文件、`LICENSE` 和 `README.md`：

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

或者直接使用原始二进制文件：

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### 支持的平台与架构

| 平台 | 架构 | Rust 目标 | 分发形式 |
| ---- | ---- | --------- | -------- |
| Linux（静态 musl） | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | 二进制文件、deb/rpm |
| Linux（glibc ≥ 2.17） | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | 二进制文件、deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | 二进制文件、Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI 镜像 |

### 从源码构建

```sh
cargo build --release
# 二进制文件位于 target/release/fpm-monitor
```

### 交叉编译

CI 使用 [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)，在单个
x86_64 runner 上生成全部 Linux 二进制文件：

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# 二进制文件位于 target/aarch64-unknown-linux-musl/release/fpm-monitor
```

将目标替换为上表中的任意 triple。若要在 `php:*-fpm-alpine` 容器内部署：

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux 软件包（Red Hat / Debian）

每次发布都会附带适用于 amd64、arm64、armhf、armel、i386、riscv64、ppc64el、ppc64、
s390x、loong64 和 sparc64 的 `.deb` 与 `.rpm` 软件包。musl 版本没有任何外部依赖，
可在任何较新的 RHEL、Fedora、CentOS、Debian 或 Ubuntu 版本上运行。

- **Red Hat、Fedora、CentOS**：
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian、Ubuntu**：
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- 在其他架构上，请将 `amd64`/`x86_64` 替换为对应的软件包名称（`arm64`、`armhf`、
  `armel`、`i386`、`riscv64`、`ppc64el`、`ppc64`、`s390x`、`loong64`、`sparc64`）。

如需基于现有的 `target/` 目录在本地重新构建归档包和软件包：

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # 需要在 PATH 中提供 nfpm
```

### macOS（Homebrew）

每次发布都会生成并附带一个可直接使用的 formula：

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

若要将其作为 tap 托管，请把文件移动到
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`，然后：

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

多架构镜像已发布至 GitHub Container Registry，支持 `linux/amd64`、`linux/arm64`、
`linux/ppc64le` 和 `linux/riscv64`：

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` 使监控工具能够从宿主机的 `/proc` 读取 PHP-FPM 进程；通过 `-v` 挂载
你的 FPM 进程池配置，以便发现进程池（常规的 `/etc` 路径也会被扫描）。

## 用法

```
fpm-monitor [OPTIONS]

选项：
  -c, --config <PATH>     要分析的 php-fpm.conf 文件或进程池目录
  -v, --verbose           显示每个 worker 的详细信息（pid、状态、RSS）
      --color             强制启用颜色
      --no-color          禁用颜色
  -t, --tui               交互式仪表盘（标签页、`←`/`→` 子标签页）
      --interval <SEC>    TUI 刷新间隔（默认 1）
      --lang <LANG>       界面语言：en, fr, zh, ar, es, it, ja, de
                          （默认：en）
      --mock              演示数据（本地测试）
  -h, --help              显示此帮助
```

### 界面语言

`--lang` 接受 `en`（美式，默认）、`fr`、`zh`、`ar`、`es`、`it`、`ja`、`de`
（也接受 `--lang=fr` 形式）。它既适用于 TUI 仪表盘，也适用于 CLI 输出
（表头、摘要、图例）和 `--help`。

### TUI 仪表盘

TUI 会占满整个终端（支持动态调整大小）并保持响应：数据按设定的间隔
（`--interval`，默认 1 秒）在后台刷新；即使某次 `/proc` 读取较慢，渲染也不会中断。

| 按键 | 操作 |
| ---- | ---- |
| `1` – `4`、`Tab` | 切换标签页 |
| `←` / `→` | 在子标签页间导航（进程池/配置） |
| `↑` / `↓` | 滚动监控视图 |
| `q`、`Ctrl-C` | 退出 |

## 项目结构

| 文件 | 作用 |
| ---- | ---- |
| `src/main.rs` | CLI、选项、流程编排 |
| `src/config.rs` | 配置发现与解析 |
| `src/proc.rs` | 读取 `/proc`（worker、RSS、状态、TCP backlog） |
| `src/data.rs` | 将数据合并为表格行 |
| `src/logs.rs` | 读取 PHP 日志与慢查询 |
| `src/render.rs` | 文本输出（CLI 表格） |
| `src/tui.rs` | 交互式仪表盘（4 个标签页、图表） |
| `src/i18n.rs` | 本地化（8 种语言、`--lang`） |
| `src/term.rs` | 颜色检测 / 样式 |
| `tests/cli.rs` | CLI 端到端测试（`--help`、`--mock`、i18n） |
| `.github/workflows` | CI（测试、覆盖率、18 个构建、QEMU 冒烟测试） |

## 开发

```sh
cargo build          # 构建（debug）
cargo test           # 单元测试 + 集成测试
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # 格式化
```

CI 在 Linux、macOS 和 Windows 上、于 MSRV（Rust 1.78）下运行测试套件，
发布覆盖率徽章，交叉编译 18 个发布目标，并在 QEMU 下执行外部架构的二进制文件。

## 许可证

[MIT](LICENSE)
