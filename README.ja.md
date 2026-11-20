<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | **日本語** | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Rust で書かれた PHP-FPM プールモニター（`fpm-monitor.c` の移植版）で、
[Ember](https://github.com/alexandre-daubois/ember) に着想を得た対話型ターミナル
ダッシュボードを備えています。単一の依存関係のないバイナリとして提供され、
**Linux、macOS、Windows** の **18 のターゲットアーキテクチャ**で動作します。
インターフェースは `--lang` により 22 言語（デフォルト: 英語）で利用でき、
このドキュメントは 22 言語に翻訳されています。

<p align="center">
  <img src="docs/screenshot.ja.animated.svg" alt="fpm-monitor TUI — 対話型ダッシュボード（日本語）" width="700">
</p>

## 機能

- **CLI 出力**: プールの表（プール、種別、ワーカー、稼働中、アイドル、バックログ、
  max_children、max_requests、メモリ）と、ワーカーごとの詳細（`-v`: pid、状態、RSS）。
- **TUI ダッシュボード**（`-t`）: 4 つのタブを備えた対話型インターフェース。
  - **モニタリング**: プールとそのワーカーのリアルタイム状態（プールごとのメモリ）。
  - **グラフ**: プールごとに 1 つの大きなチャート（Ember スタイル）。`←`/`→` で
    `running`、`workers`、`idle`、`backlog` のサブタブを切り替えられます（空でも常に
    表示）。さらに破線のしきい値ライン: `max_children`（グレー）、`max_requests`（赤）、
    `min_spare`（ピンク）、`max_spare`（紫）を `idle` パネルに表示します。
  - **ログ**: プールごとに、PHP ログ（`error_log`）、スロークエリ（`slowlog`）、および
    **アクセスログ**（`access.log`、HTTP ステータスコードを色分け）の最新行を
    サブタブで表示します。
  - **設定**: ファイルから読み取ったディレクティブ（`global` + プール）を
    サブタブで表示します。
- 設定の自動検出（一般的な場所 + `include` ディレクティブ）。
- ネットワーク通信なし: すべて `/proc` から直接読み取ります（プロセス、RSS、状態、
  TCP ソケットの accept キュー）。macOS と BSD では同じデータを `ps` と `netstat`
  から読み取るため、FPM のステータス設定（`pm.status_path`）は不要です。
- **ポータブル設計**: 10 の Linux アーキテクチャ向けの静的 musl バイナリ、残りは
  glibc ビルドに加え、macOS と Windows。実行時の依存関係も、一致させる libc
  バージョンもありません（記載の glibc ビルドを除く）。

## 要件

- ソースからコンパイルするには **Rust 1.78 以上**（MSRV は CI で検証）。
- **Linux**: 実データには `/proc` を使用します。TCP ソケットでは accept キューを
  `/proc/net/tcp` から読み取り、unix ソケットでは `listen.backlog` を最大値として
  表示するのみです。
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: `/proc` はなく、プロセスと
  ソケットは `ps` と `netstat` で読み取ります。
- **Windows**: CLI はビルド・実行できますが（`--help`、`--mock`）、Windows では
  PHP-FPM が利用できないため、ライブのプロセス検出は明示的なエラーを返します。

## インストール

### ビルド済みバイナリ

各[リリース](https://github.com/filoucrackeur/fpm-monitor/releases)には、サポート
されるすべてのターゲット向けの生バイナリ、ターゲットごとのアーカイブ（Unix では
`.tar.gz`、Windows では `.zip`）、`.deb`/`.rpm` パッケージ、Homebrew の formula が
含まれています。

アーカイブは自己完結型で、それぞれに `fpm-monitor` 実行ファイル、`LICENSE`、
`README.md` が含まれます:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

または、生バイナリを直接使用します:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### 対応プラットフォームとアーキテクチャ

| プラットフォーム | アーキテクチャ | Rust ターゲット | 配布形態 |
| -------- | ------------- | ------------ | ------------ |
| Linux（静的 musl） | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | バイナリ、deb/rpm |
| Linux（glibc 2.17 以上） | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | バイナリ、deb/rpm |
| macOS 11 以上 | Apple Silicon、Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | バイナリ、Homebrew |
| Windows 10 以上 | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI イメージ |

### ソースから

```sh
cargo build --release
# バイナリは target/release/fpm-monitor にあります
```

### クロスコンパイル

CI では [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) を使用して、
単一の x86_64 ランナーからすべての Linux バイナリを生成します:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# バイナリは target/aarch64-unknown-linux-musl/release/fpm-monitor にあります
```

ターゲットは上の表の任意のトリプルに置き換えてください。
`php:*-fpm-alpine` コンテナ内にデプロイするには:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux パッケージ（Red Hat / Debian）

`.deb` と `.rpm` パッケージは各リリースに添付され、amd64、arm64、armhf、armel、
i386、riscv64、ppc64el、ppc64、s390x、loong64、sparc64 に対応しています。musl 版は
外部依存がなく、最近の RHEL、Fedora、CentOS、Debian、Ubuntu のいずれでも動作します。

- **Red Hat、Fedora、CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian、Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- その他のアーキテクチャでは、`amd64`/`x86_64` を対応するパッケージ名
  （`arm64`、`armhf`、`armel`、`i386`、`riscv64`、`ppc64el`、`ppc64`、`s390x`、
  `loong64`、`sparc64`）に置き換えてください。

既存の `target/` ディレクトリからアーカイブとパッケージをローカルで再ビルドするには:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # PATH に nfpm が必要
```

### macOS（Homebrew）

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

マルチアーキテクチャイメージが GitHub Container Registry で公開されています
（`linux/amd64`、`linux/arm64`、`linux/ppc64le`、`linux/riscv64`）:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` により、モニターはホストの `/proc` から PHP-FPM プロセスを読み取れます。
`-v` で FPM プール設定をマウントするとプールが検出されます（通常の `/etc` パスも
スキャンされます）。

## 使い方

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     解析する php-fpm.conf ファイルまたはプールディレクトリ
  -v, --verbose           ワーカーごとの詳細を表示（pid、状態、RSS）
      --color             色を強制
      --no-color          色を無効化
  -t, --tui               対話型ダッシュボード（タブ、`←`/`→` サブタブ）
      --interval <SEC>    TUI の更新間隔（デフォルト 1）
      --lang <LANG>       インターフェース言語: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          （デフォルト: en）
      --mock              デモデータ（ローカルテスト）
  -h, --help              このヘルプを表示
```

### インターフェース言語

`--lang` は `en`（US、デフォルト）、`fr`、`zh`、`ar`、`es`、`it`、`ja`、`de`、`ru`、`hi`、`id`、`ko`、`no`、`sv`、`fi`、`da`、`el`、`tr`、`pl`、`uk`、`th`、`vi` を
受け付けます（`--lang=fr` の形式も使用できます）。TUI ダッシュボードのほか、CLI 出力
（ヘッダー、サマリー、凡例）と `--help` にも適用されます。

### TUI ダッシュボード

TUI はターミナル全体を占有し（動的なリサイズ）、応答性を保ちます: データは設定した
間隔（`--interval`、デフォルト 1 秒）でバックグラウンド更新され、`/proc` の読み取りが
遅くても描画は続行されます。

| キー            | 操作                               |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | タブを切り替え                   |
| `←` / `→`      | サブタブを移動（プール/設定）      |
| `↑` / `↓`      | モニタリングビューをスクロール     |
| `q`, `Ctrl-C`  | 終了                               |

## プロジェクト構成

| ファイル          | 役割                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI、オプション、オーケストレーション               |
| `src/config.rs`   | 設定の検出と解析                                    |
| `src/proc.rs`     | `/proc` の読み取り（ワーカー、RSS、状態、TCP バックログ） |
| `src/data.rs`     | データを表の行にマージ                              |
| `src/logs.rs`     | PHP ログとスロークエリの読み取り                    |
| `src/render.rs`   | テキスト出力（CLI の表）                            |
| `src/tui.rs`      | 対話型ダッシュボード（4 タブ、グラフ）              |
| `src/i18n.rs`     | ローカライズ（22 言語、`--lang`）                    |
| `src/term.rs`     | 色の検出 / スタイル                                 |
| `tests/cli.rs`    | エンドツーエンド CLI テスト（`--help`、`--mock`、i18n） |
| `.github/workflows` | CI（テスト、カバレッジ、18 ビルド、QEMU スモークテスト） |

## 開発

```sh
cargo build          # ビルド（デバッグ）
cargo test           # ユニット + 統合テスト
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # フォーマット
```

CI は Linux、macOS、Windows でテストスイートを実行し、MSRV（Rust 1.78）を検証し、
カバレッジバッジを公開し、18 のリリースターゲットをクロスコンパイルし、異なる
アーキテクチャのバイナリを QEMU 上で実行します。

## ライセンス

[MIT](LICENSE)
