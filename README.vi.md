<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | **Tiếng Việt**

Trình giám sát pool PHP-FPM viết bằng Rust (bản chuyển từ `fpm-monitor.c`), với
bảng điều khiển tương tác trên terminal lấy cảm hứng từ
[Ember](https://github.com/alexandre-daubois/ember). Ứng dụng được phát hành dưới
dạng một tệp nhị phân duy nhất, không phụ thuộc, chạy trên **Linux, macOS và
Windows**, trên **18 kiến trúc mục tiêu**. Giao diện có sẵn 22 ngôn ngữ (mặc định:
tiếng Anh) qua `--lang`, và tài liệu này được dịch sang 22 ngôn ngữ.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — bảng điều khiển tương tác (tiếng Việt)" width="700">
</p>

## Tính năng

- **Đầu ra CLI**: bảng pool (pool, type, workers, running, idle, backlog,
  max_children, max_requests, memory) kèm chi tiết từng worker (`-v`: pid, state, RSS).
- **Bảng điều khiển TUI** (`-t`): giao diện tương tác với 4 tab.
  - **Giám sát**: trạng thái thời gian thực của các pool và worker của chúng (bộ nhớ theo từng pool).
  - **Biểu đồ**: một biểu đồ lớn cho mỗi pool (kiểu Ember) với các tab con `←`/`→`:
    `running`, `workers`, `idle`, `backlog` (luôn hiển thị, kể cả khi trống), cùng
    các đường ngưỡng nét đứt: `max_children` (xám), `max_requests` (đỏ),
    `min_spare` (hồng) và `max_spare` (tím) trên bảng `idle`.
  - **Nhật ký**: theo từng pool, các dòng mới nhất của log PHP (`error_log`), truy vấn
    chậm (`slowlog`) và **access log** (`access.log`, mã HTTP được tô màu), trong các tab con.
  - **Cấu hình**: các chỉ thị đọc từ tệp (`global` + các pool) trong các tab con.
- Tự động phát hiện cấu hình (các vị trí thông thường + chỉ thị `include`).
- Không gọi qua mạng: mọi thứ được đọc trực tiếp từ `/proc` (tiến trình, RSS,
  trạng thái, hàng đợi chấp nhận kết nối của socket TCP). Trên macOS và BSD, dữ liệu
  tương tự được đọc từ `ps` và `netstat`; không cần cấu hình trạng thái FPM (`pm.status_path`).
- **Di động theo thiết kế**: tệp nhị phân musl tĩnh cho 10 kiến trúc Linux,
  bản dựng glibc cho các kiến trúc còn lại, cùng với macOS và Windows. Không phụ
  thuộc lúc chạy, không cần khớp phiên bản libc (trừ các bản dựng glibc được ghi chú).

## Yêu cầu

- **Rust 1.78+** để biên dịch từ mã nguồn (MSRV được kiểm tra trong CI).
- **Linux**: `/proc` được dùng cho dữ liệu thực. Với socket TCP, hàng đợi chấp nhận
  được đọc từ `/proc/net/tcp`; với socket unix, chỉ hiển thị `listen.backlog` làm giá trị tối đa.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: không có `/proc`; tiến trình và
  socket được đọc bằng `ps` và `netstat`.
- **Windows**: CLI biên dịch và chạy được (`--help`, `--mock`), nhưng PHP-FPM không
  có trên Windows, nên việc phát hiện tiến trình trực tiếp trả về lỗi rõ ràng.

## Cài đặt

### Tệp nhị phân dựng sẵn

Mỗi [bản phát hành](https://github.com/filoucrackeur/fpm-monitor/releases) chứa tệp
nhị phân thô cho tất cả mục tiêu được hỗ trợ, một tệp lưu trữ cho mỗi mục tiêu
(`.tar.gz` trên Unix, `.zip` trên Windows), gói `.deb`/`.rpm` và công thức Homebrew.

Các tệp lưu trữ khép kín: mỗi tệp chứa tệp thực thi `fpm-monitor`, `LICENSE`
và `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Hoặc dùng trực tiếp tệp nhị phân thô:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Nền tảng và kiến trúc được hỗ trợ

| Nền tảng | Kiến trúc | Mục tiêu Rust | Phân phối |
| -------- | ------------- | ------------ | ------------ |
| Linux (musl tĩnh) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | nhị phân, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | nhị phân, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | nhị phân, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | ảnh OCI |

### Từ mã nguồn

```sh
cargo build --release
# tệp nhị phân nằm tại target/release/fpm-monitor
```

### Biên dịch chéo

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) được CI sử dụng để
tạo toàn bộ tệp nhị phân Linux từ một runner x86_64 duy nhất:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# tệp nhị phân nằm tại target/aarch64-unknown-linux-musl/release/fpm-monitor
```

Thay mục tiêu bằng bất kỳ bộ ba nào trong bảng trên. Để triển khai bên trong
container `php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Gói Linux (Red Hat / Debian)

Các gói `.deb` và `.rpm` được đính kèm vào mỗi bản phát hành cho amd64, arm64,
armhf, armel, i386, riscv64, ppc64el, ppc64, s390x, loong64 và sparc64. Các gói
musl không có phụ thuộc bên ngoài và hoạt động trên mọi phiên bản RHEL, Fedora,
CentOS, Debian hoặc Ubuntu gần đây.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Trên các kiến trúc khác, thay `amd64`/`x86_64` bằng tên gói tương ứng
  (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`,
  `s390x`, `loong64`, `sparc64`).

Để dựng lại các tệp lưu trữ và gói cục bộ từ thư mục `target/` có sẵn:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # cần nfpm trong PATH
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

Một ảnh đa kiến trúc được phát hành trên GitHub Container Registry cho
`linux/amd64`, `linux/arm64`, `linux/ppc64le` và `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` cho phép trình giám sát đọc các tiến trình PHP-FPM từ `/proc` của máy
chủ; gắn kết cấu hình pool FPM bằng `-v` để các pool được phát hiện (các đường dẫn
`/etc` thông thường cũng được quét).

## Cách sử dụng

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     tệp php-fpm.conf hoặc thư mục pool cần phân tích
  -v, --verbose           hiển thị chi tiết từng worker (pid, state, RSS)
      --color             buộc bật màu
      --no-color          tắt màu
  -t, --tui               bảng điều khiển tương tác (tab, tab con `←`/`→`)
      --interval <SEC>    khoảng thời gian làm mới TUI (mặc định 1)
      --lang <LANG>       ngôn ngữ giao diện: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (mặc định: en)
      --mock              dữ liệu demo (kiểm thử cục bộ)
  -h, --help              hiển thị trợ giúp này
```

### Ngôn ngữ giao diện

`--lang` chấp nhận `en` (Mỹ, mặc định), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
(cũng chấp nhận dạng `--lang=fr`). Tùy chọn này áp dụng cho cả bảng điều khiển TUI
lẫn đầu ra CLI (tiêu đề, tóm tắt, chú giải) và `--help`.

### Bảng điều khiển TUI

TUI chiếm toàn bộ terminal (tự co giãn) và luôn phản hồi: dữ liệu được làm mới ở
chế độ nền theo khoảng thời gian đã đặt (`--interval`, mặc định 1 giây); ngay cả khi
việc đọc `/proc` chậm, quá trình hiển thị vẫn tiếp tục.

| Phím            | Hành động                             |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Chuyển tab                       |
| `←` / `→`      | Điều hướng tab con (pool/cấu hình)   |
| `↑` / `↓`      | Cuộn chế độ xem Giám sát         |
| `q`, `Ctrl-C`  | Thoát                               |

## Cấu trúc dự án

| Tệp              | Vai trò                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, tùy chọn, điều phối                         |
| `src/config.rs`   | Phát hiện và phân tích cấu hình                 |
| `src/proc.rs`     | Đọc `/proc` (worker, RSS, trạng thái, TCP backlog)  |
| `src/data.rs`     | Hợp nhất dữ liệu thành các hàng của bảng                        |
| `src/logs.rs`     | Đọc log PHP và truy vấn chậm                    |
| `src/render.rs`   | Đầu ra văn bản (bảng CLI)                             |
| `src/tui.rs`      | Bảng điều khiển tương tác (4 tab, biểu đồ)              |
| `src/i18n.rs`     | Bản địa hóa (22 ngôn ngữ, `--lang`)                |
| `src/term.rs`     | Phát hiện màu / kiểu                            |
| `tests/cli.rs`    | Kiểm thử CLI đầu-cuối (`--help`, `--mock`, i18n)      |
| `.github/workflows` | CI (kiểm thử, độ phủ, 18 bản dựng, kiểm thử QEMU) |

## Phát triển

```sh
cargo build          # dựng (debug)
cargo test           # kiểm thử đơn vị + tích hợp
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # định dạng
```

CI chạy bộ kiểm thử trên Linux, macOS và Windows, trên MSRV (Rust 1.78), công bố
huy hiệu độ phủ, biên dịch chéo 18 mục tiêu phát hành và thực thi các tệp nhị phân
cho kiến trúc khác dưới QEMU.

## Đóng góp

Rất hoan nghênh các đóng góp! Hãy đọc [CONTRIBUTING.md](CONTRIBUTING.md) trước khi
mở issue hoặc pull request, tuân thủ [Quy tắc ứng xử](CODE_OF_CONDUCT.md) của
chúng tôi và xem [SUPPORT.md](SUPPORT.md) cho các câu hỏi về cách sử dụng. Vui
lòng báo cáo các vấn đề bảo mật một cách riêng tư theo hướng dẫn trong
[SECURITY.md](SECURITY.md).

## Giấy phép

[MIT](LICENSE)
