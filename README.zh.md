# fpm-monitor

[English](README.md) | [Français](README.fr.md) | **简体中文** | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md)

[![CI](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml) [![Release](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/php-fpm-monitor?sort=semver)](https://github.com/filoucrackeur/php-fpm-monitor/releases) [![codecov](https://img.shields.io/codecov/c/github/filoucrackeur/php-fpm-monitor)](https://codecov.io/github/filoucrackeur/php-fpm-monitor) [![Maintainability](https://img.shields.io/codeclimate/maintainability/filoucrackeur/php-fpm-monitor)](https://codeclimate.com/github/filoucrackeur/php-fpm-monitor) [![License](https://img.shields.io/github/license/filoucrackeur/php-fpm-monitor)](LICENSE) ![Static](https://img.shields.io/badge/static-musl-blue) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/php-fpm-monitor)](https://github.com/filoucrackeur/php-fpm-monitor/commits/main)

用 Rust 编写的 PHP-FPM 进程池监视器（`fpm-monitor.c` 的移植版），带有灵感来自
[Ember](https://github.com/alexandre-daubois/ember) 的交互式终端仪表盘。界面支持 8 种语言
（默认：英语），通过 `--lang` 切换。

## 功能

- **CLI 输出**：进程池表格（pool、type、workers、running、idle、backlog、max_children、
  max_requests、内存），并显示每个 worker 的详细信息（`-v`：pid、状态、RSS）。
- **TUI 仪表盘**（`-t`）：包含 4 个标签页的交互界面。
  - **监控**：进程池及其 worker 的实时状态（按进程池显示内存）。
  - **图表**：每个进程池一个大型图表（Ember 风格），支持 ← → 子标签：
    `running`、`workers`、`idle`、`backlog`（即使为空也始终显示），以及虚线阈值线：
    `max_children`（灰色）、`max_requests`（红色）、`min_spare`（粉色）和 `max_spare`（紫色），
    显示在 `idle` 面板上。
  - **日志**：每个进程池的 PHP 日志（`error_log`）、慢查询（`slowlog`）和**访问日志**
    （`access.log`，HTTP 状态码着色）的最新行，位于子标签中。
  - **配置**：从文件中读取的指令（`global` + 进程池），位于子标签中。
- 自动检测配置（常见位置 + `include` 指令）。
- 无网络调用：一切直接从 `/proc` 读取（进程、RSS、状态、TCP 套接字接受队列）。

## 要求

- 需要 Rust（stable）进行编译。
- **使用真实数据需要 Linux 和 `/proc`**。无需 FPM 状态配置（`pm.status_path`）：数据来自
  `/proc` 和配置文件。对于 TCP 套接字，从 `/proc/net/tcp`（接受队列）读取 backlog；对于
  unix 套接字，仅显示 `listen.backlog` 作为最大值。

## 安装

```sh
cargo build --release
# 二进制文件位于 target/release/fpm-monitor
```

部署到 `php:*-fpm-alpine` 容器（musl、aarch64）中：

```sh
RUSTC=$HOME/.cargo/bin/rustc ~/.cargo/bin/cargo build --release --target aarch64-unknown-linux-musl
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux 软件包（Red Hat / Debian）

每个 [release](https://github.com/filoucrackeur/php-fpm-monitor/releases) 都会附带静态（musl）`.deb` 和
`.rpm` 软件包。它们没有外部依赖，可在任何较新版本的 RHEL、Fedora、CentOS、Debian 或
Ubuntu 上运行（x86_64 和 ARM64）。

- **Red Hat、Fedora、CentOS**：
  ```sh
  sudo dnf install fpm-monitor_<version>_amd64.rpm
  ```
- **Debian、Ubuntu**：
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- 在 ARM64 上，请改用 `arm64` 软件包。

### macOS（Homebrew）

每次发布都会生成一个可直接使用的公式并附带：

```sh
brew install https://github.com/filoucrackeur/php-fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

若要将其托管为 tap，请把文件放到 `<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`，然后：

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

## 用法

```
fpm-monitor [OPTIONS]

选项:
  -c, --config <路径>   要分析的 php-fpm.conf 文件或进程池目录
  -v, --verbose         显示每个 worker 的详细信息（pid、状态、RSS）
      --color           强制启用颜色
      --no-color        禁用颜色
  -t, --tui             交互式仪表盘（标签页、← → 子标签）
      --interval <秒>   TUI 刷新间隔（默认 1）
      --lang <语言>      界面语言：en、fr、zh、ar、es、it、ja、de
                         （默认：en）
      --mock            演示数据（本地测试）
  -h, --help            显示此帮助
```

### 界面语言

`--lang` 接受 `en`（美式，默认）、`fr`、`zh`、`ar`、`es`、`it`、`ja`、`de`
（也接受 `--lang=fr` 形式）。它适用于 TUI 仪表盘以及 CLI 输出（表头、摘要、图例）和 `--help`。

### TUI 仪表盘

TUI 占满整个终端（动态调整大小）并保持响应：数据按设定的间隔（`--interval`，默认 1 秒）
在后台刷新；即使 `/proc` 读取缓慢，渲染也会继续。

| 按键             | 操作                          |
| ---------------- | ----------------------------- |
| `1` – `4`、`Tab` | 切换标签页                    |
| `←` / `→`        | 浏览子标签（进程池/配置）     |
| `↑` / `↓`        | 滚动监控视图                  |
| `q`、`Ctrl-C`    | 退出                          |

## 项目结构

| 文件              | 作用                                     |
| ----------------- | ---------------------------------------- |
| `src/main.rs`     | CLI、选项、编排                          |
| `src/config.rs`   | 配置发现与解析                           |
| `src/proc.rs`     | 读取 `/proc`（worker、RSS、状态、TCP backlog） |
| `src/data.rs`     | 将数据合并为表格行                       |
| `src/logs.rs`     | 读取 PHP 日志和慢查询                    |
| `src/render.rs`   | 文本输出（CLI 表格）                     |
| `src/tui.rs`      | 交互式仪表盘（4 个标签页、图表）         |
| `src/i18n.rs`     | 本地化（8 种语言，`--lang`）             |
| `src/term.rs`     | 颜色检测 / 样式                          |

## 开发

```sh
cargo build          # 编译（debug）
cargo test           # 运行单元测试
cargo clippy         # 代码检查
cargo fmt --check    # 格式化
```

## 许可证

[MIT](LICENSE)
