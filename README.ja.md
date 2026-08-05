<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | **日本語** | [Deutsch](README.de.md)

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Release](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/release.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/release.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases) [![codecov](https://img.shields.io/codecov/c/github/filoucrackeur/fpm-monitor)](https://codecov.io/github/filoucrackeur/fpm-monitor) [![Maintainability](https://img.shields.io/codeclimate/maintainability/filoucrackeur/fpm-monitor)](https://codeclimate.com/github/filoucrackeur/fpm-monitor) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) ![Static](https://img.shields.io/badge/static-musl-blue) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

Rust 製の PHP-FPM プールモニター（`fpm-monitor.c` の移植版）。[Ember](https://github.com/alexandre-daubois/ember)
に着想を得た対話型ターミナルダッシュボードを備えています。インターフェースは `--lang` で
8 言語（デフォルト: 英語）から選択できます。

<p align="center">
  <img src="docs/screenshot.ja.animated.svg" alt="fpm-monitor TUI インタラクティブダッシュボード（日本語）" width="700">
</p>

## 機能

- **CLI 出力**: プールの表（pool、type、workers、running、idle、backlog、max_children、
  max_requests、メモリ）に加え、ワーカーごとの詳細（`-v`: pid、状態、RSS）。
- **TUI ダッシュボード**（`-t`）: 4 つのタブを持つ対話型インターフェース。
  - **モニタリング**: プールとそのワーカーのリアルタイム状態（プールごとのメモリ）。
  - **グラフ**: プールごとに 1 つの大きなチャート（Ember スタイル）。← → でサブタブを
    切り替え: `running`、`workers`、`idle`、`backlog`（空でも常に表示）。さらに破線の
    しきい値ライン: `max_children`（グレー）、`max_requests`（赤）、`min_spare`（ピンク）、
    `max_spare`（紫）を `idle` パネルに表示。
  - **ログ**: プールごとに、PHP ログ（`error_log`）、スロークエリ（`slowlog`）、および
    **アクセスログ**（`access.log`、HTTP ステータスコードに色付け）の最新行をサブタブで表示。
  - **設定**: ファイルから読み取ったディレクティブ（`global` + プール）をサブタブで表示。
- 設定の自動検出（一般的な場所 + `include` ディレクティブ）。
- ネットワーク呼び出しなし: すべて `/proc` から直接読み取ります（プロセス、RSS、状態、
  TCP ソケットの受付キュー）。

## 要件

- コンパイルには Rust（stable）。
- 実データには **Linux と `/proc`** が必要。FPM のステータス設定（`pm.status_path`）は
  不要です。データは `/proc` と設定ファイルから取得します。TCP ソケットの backlog は
  `/proc/net/tcp`（受付キュー）から読み取り、unix ソケットでは `listen.backlog` のみを
  最大値として表示します。

## インストール

```sh
cargo build --release
# バイナリは target/release/fpm-monitor に生成されます
```

`php:*-fpm-alpine` コンテナ（musl、aarch64）にデプロイする場合:

```sh
RUSTC=$HOME/.cargo/bin/rustc ~/.cargo/bin/cargo build --release --target aarch64-unknown-linux-musl
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux パッケージ（Red Hat / Debian）

各 [リリース](https://github.com/filoucrackeur/fpm-monitor/releases) には静的（musl）な `.deb` と
`.rpm` パッケージが添付されます。外部依存はなく、最新の RHEL、Fedora、CentOS、Debian、
Ubuntu（x86_64 / ARM64）で動作します。

- **Red Hat、Fedora、CentOS**:
  ```sh
  sudo dnf install fpm-monitor_<version>_amd64.rpm
  ```
- **Debian、Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- ARM64 では `arm64` パッケージを使ってください。

### macOS（Homebrew）

各リリースで、すぐ使えるフォーミュラが生成され添付されます:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

tap として公開するには、ファイルを `<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`
に置いてから:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

`linux/amd64` / `linux/arm64` 向けのマルチアーキテクチャイメージが
GitHub Container Registry に公開されています：

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` によりホストの `/proc` から PHP-FPM プロセスを読み取ります。
`-v` でプール設定をマウントするとプールが検出されます（通常の `/etc`
パスもスキャンされます）。

## 使い方

```
fpm-monitor [オプション]

オプション:
  -c, --config <パス>   解析する php-fpm.conf ファイルまたはプールディレクトリ
  -v, --verbose         各ワーカーの詳細を表示（pid、状態、RSS）
      --color           色を強制
      --no-color        色を無効化
  -t, --tui             対話型ダッシュボード（タブ、← → サブタブ）
      --interval <秒>   TUI 更新間隔（デフォルト 1）
      --lang <言語>      インターフェース言語: en、fr、zh、ar、es、it、ja、de
                         （デフォルト: en）
      --mock            デモデータ（ローカルテスト）
  -h, --help            このヘルプを表示
```

### インターフェース言語

`--lang` は `en`（米国、デフォルト）、`fr`、`zh`、`ar`、`es`、`it`、`ja`、`de` を
受け付けます（`--lang=fr` の形式も可）。TUI ダッシュボードだけでなく CLI 出力
（ヘッダー、サマリー、凡例）と `--help` にも適用されます。

### TUI ダッシュボード

TUI はターミナル全体を占め（動的なリサイズ）、応答性を保ちます。データは設定した間隔
（`--interval`、デフォルト 1 秒）でバックグラウンド更新され、`/proc` の読み取りが遅くても
描画は継続します。

| キー            | 操作                          |
| --------------- | ----------------------------- |
| `1` – `4`、`Tab` | タブを切り替え               |
| `←` / `→`       | サブタブを移動（プール/設定） |
| `↑` / `↓`       | モニタリング画面をスクロール  |
| `q`、`Ctrl-C`   | 終了                          |

## プロジェクト構造

| ファイル           | 役割                                     |
| ------------------ | ---------------------------------------- |
| `src/main.rs`      | CLI、オプション、オーケストレーション    |
| `src/config.rs`    | 設定の検出と解析                         |
| `src/proc.rs`      | `/proc` の読み取り（ワーカー、RSS、状態、TCP backlog） |
| `src/data.rs`      | データをテーブル行にマージ               |
| `src/logs.rs`      | PHP ログとスロークエリの読み取り         |
| `src/render.rs`    | テキスト出力（CLI テーブル）             |
| `src/tui.rs`       | 対話型ダッシュボード（4 タブ、グラフ）   |
| `src/i18n.rs`      | ローカライズ（8 言語、`--lang`）         |
| `src/term.rs`      | 色検出 / スタイル                        |

## 開発

```sh
cargo build          # コンパイル（debug）
cargo test           # ユニットテストを実行
cargo clippy         # lint
cargo fmt --check    # フォーマット
```

## ライセンス

[MIT](LICENSE)
