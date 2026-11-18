<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | **ไทย** | [Tiếng Việt](README.vi.md)

มอนิเตอร์พูล PHP-FPM ที่เขียนด้วย Rust (พอร์ตจาก `fpm-monitor.c`) พร้อม
แดชบอร์ดเทอร์มินัลแบบอินเทอร์แอกทีฟที่ได้แรงบันดาลใจจาก
[Ember](https://github.com/alexandre-daubois/ember) เผยแพร่เป็นไบนารีเดียวที่ไม่มี
dependency ใด ๆ และทำงานบน **Linux, macOS และ Windows** ครอบคลุม **18 สถาปัตยกรรม
เป้าหมาย** อินเทอร์เฟซมีให้เลือก 8 ภาษา (ค่าเริ่มต้น: อังกฤษ) ผ่าน `--lang`
และเอกสารนี้แปลเป็น 22 ภาษา

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — แดชบอร์ดแบบอินเทอร์แอกทีฟ (ภาษาไทย)" width="700">
</p>

## คุณสมบัติ

- **เอาต์พุต CLI**: ตารางพูล (pool, type, workers, running, idle, backlog,
  max_children, max_requests, memory) พร้อมรายละเอียดของแต่ละเวิร์กเกอร์ (`-v`: pid, state, RSS)
- **แดชบอร์ด TUI** (`-t`): อินเทอร์เฟซแบบอินเทอร์แอกทีฟที่มี 4 แท็บ
  - **การตรวจสอบ**: สถานะแบบเรียลไทม์ของพูลและเวิร์กเกอร์ (หน่วยความจำต่อพูล)
  - **กราฟ**: กราฟขนาดใหญ่หนึ่งกราฟต่อพูล (สไตล์ Ember) พร้อมแท็บย่อย `←`/`→`:
    `running`, `workers`, `idle`, `backlog` (แสดงเสมอแม้จะว่างเปล่า) พร้อม
    เส้นเกณฑ์แบบเส้นประ: `max_children` (สีเทา), `max_requests` (สีแดง),
    `min_spare` (สีชมพู) และ `max_spare` (สีม่วง) บนแผง `idle`
  - **บันทึก**: สำหรับแต่ละพูล แสดงบรรทัดล่าสุดของบันทึก PHP (`error_log`),
    คำสั่งที่ทำงานช้า (`slowlog`) และ **บันทึกการเข้าถึง** (`access.log` พร้อมสีรหัส HTTP)
    ในแท็บย่อย
  - **การกำหนดค่า**: ไดเรกทีฟที่อ่านจากไฟล์ (`global` + พูล) ในแท็บย่อย
- การค้นหาการกำหนดค่าโดยอัตโนมัติ (ตำแหน่งตามปกติ + ไดเรกทีฟ `include`)
- ไม่มีการเรียกผ่านเครือข่าย: อ่านทุกอย่างโดยตรงจาก `/proc` (โปรเซส, RSS,
  สถานะ, คิวรับการเชื่อมต่อของซ็อกเก็ต TCP) บน macOS และ BSD จะอ่านข้อมูลเดียวกันจาก
  `ps` และ `netstat`; ไม่จำเป็นต้องกำหนดค่า `pm.status_path` ของ FPM
- **พกพาได้โดยการออกแบบ**: ไบนารี static musl สำหรับ 10 สถาปัตยกรรม Linux,
  บิลด์ glibc สำหรับสถาปัตยกรรมที่เหลือ พร้อมทั้ง macOS และ Windows ไม่มี
  dependency ขณะรัน ไม่ต้องจับคู่เวอร์ชัน libc (ยกเว้นบิลด์ glibc ที่ระบุไว้)

## ข้อกำหนด

- **Rust 1.78+** สำหรับคอมไพล์จากซอร์ส (ทดสอบ MSRV ใน CI)
- **Linux**: ใช้ `/proc` สำหรับข้อมูลจริง สำหรับซ็อกเก็ต TCP จะอ่านคิวรับการเชื่อมต่อ
  จาก `/proc/net/tcp`; สำหรับซ็อกเก็ต unix จะแสดงเฉพาะ `listen.backlog` เป็นค่าสูงสุด
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: ไม่มี `/proc`; อ่านโปรเซสและ
  ซ็อกเก็ตด้วย `ps` และ `netstat`
- **Windows**: CLI คอมไพล์และทำงานได้ (`--help`, `--mock`) แต่ไม่มี PHP-FPM บน
  Windows ดังนั้นการค้นหาโปรเซสจริงจะคืนค่าข้อผิดพลาดอย่างชัดเจน

## การติดตั้ง

### ไบนารีที่สร้างไว้ล่วงหน้า

ทุก [release](https://github.com/filoucrackeur/fpm-monitor/releases) มีไบนารีแบบดิบ
สำหรับทุกเป้าหมายที่รองรับ ไฟล์อาร์ไคว์หนึ่งไฟล์ต่อเป้าหมาย (`.tar.gz` บน Unix, `.zip`
บน Windows) แพ็กเกจ `.deb`/`.rpm` และสูตร Homebrew

ไฟล์อาร์ไคว์เป็นแบบครบในตัวเอง: แต่ละไฟล์มีโปรแกรม `fpm-monitor`, `LICENSE`
และ `README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

หรือใช้ไบนารีแบบดิบโดยตรง:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### แพลตฟอร์มและสถาปัตยกรรมที่รองรับ

| แพลตฟอร์ม | สถาปัตยกรรม | เป้าหมาย Rust | การเผยแพร่ |
| -------- | ------------- | ------------ | ------------ |
| Linux (static musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | ไบนารี, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | ไบนารี, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | ไบนารี, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | อิมเมจ OCI |

### จากซอร์ส

```sh
cargo build --release
# ไบนารีอยู่ที่ target/release/fpm-monitor
```

### การคอมไพล์ข้ามแพลตฟอร์ม

[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) ถูกใช้โดย CI
เพื่อสร้างไบนารี Linux ทั้งหมดจากรันเนอร์ x86_64 เครื่องเดียว:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# ไบนารีอยู่ที่ target/aarch64-unknown-linux-musl/release/fpm-monitor
```

แทนที่เป้าหมายด้วยทริปเปิลใด ๆ จากตารางด้านบน หากต้องการติดตั้งภายใน
คอนเทนเนอร์ `php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### แพ็กเกจ Linux (Red Hat / Debian)

แพ็กเกจ `.deb` และ `.rpm` แนบมากับทุกรีลีสสำหรับ amd64, arm64, armhf, armel,
i386, riscv64, ppc64el, ppc64, s390x, loong64 และ sparc64 เวอร์ชัน musl ไม่มี
dependency ภายนอกและทำงานได้บน RHEL, Fedora, CentOS, Debian หรือ Ubuntu
เวอร์ชันใหม่ใด ๆ

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- บนสถาปัตยกรรมอื่น ให้แทนที่ `amd64`/`x86_64` ด้วยชื่อแพ็กเกจที่ตรงกัน
  (`arm64`, `armhf`, `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`,
  `s390x`, `loong64`, `sparc64`)

หากต้องการสร้างไฟล์อาร์ไคว์และแพ็กเกจใหม่ในเครื่องจากไดเรกทอรี `target/` ที่มีอยู่:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # ต้องมี nfpm ใน PATH
```

### macOS (Homebrew)

สูตรที่พร้อมใช้งานจะถูกสร้างและแนบมากับทุกรีลีส:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

หากต้องการโฮสต์เป็น tap แทน ให้ย้ายไฟล์ไปที่
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb` แล้วรัน:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

อิมเมจหลายสถาปัตยกรรมเผยแพร่บน GitHub Container Registry สำหรับ
`linux/amd64`, `linux/arm64`, `linux/ppc64le` และ `linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` ช่วยให้มอนิเตอร์อ่านโปรเซส PHP-FPM จาก `/proc` ของโฮสต์ได้;
เมานต์การกำหนดค่าพูล FPM ด้วย `-v` เพื่อให้ค้นพบพูล (เส้นทาง `/etc` ตามปกติ
จะถูกสแกนด้วย)

## การใช้งาน

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     ไฟล์ php-fpm.conf หรือไดเรกทอรีพูลที่ต้องการวิเคราะห์
  -v, --verbose           แสดงรายละเอียดของแต่ละเวิร์กเกอร์ (pid, state, RSS)
      --color             บังคับใช้สี
      --no-color          ปิดการใช้สี
  -t, --tui               แดชบอร์ดแบบอินเทอร์แอกทีฟ (แท็บ, แท็บย่อย `←`/`→`)
      --interval <SEC>    ช่วงเวลารีเฟรชของ TUI (ค่าเริ่มต้น 1)
      --lang <LANG>       ภาษาของอินเทอร์เฟซ: en, fr, zh, ar, es, it, ja, de
                          (ค่าเริ่มต้น: en)
      --mock              ข้อมูลสาธิต (ทดสอบในเครื่อง)
  -h, --help              แสดงความช่วยเหลือนี้
```

### ภาษาของอินเทอร์เฟซ

`--lang` รองรับ `en` (สหรัฐอเมริกา, ค่าเริ่มต้น), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`
(รองรับรูปแบบ `--lang=fr` ด้วย) มีผลกับทั้งแดชบอร์ด TUI และเอาต์พุต CLI
(ส่วนหัว, สรุป, คำอธิบายสัญลักษณ์) รวมถึง `--help`

### แดชบอร์ด TUI

แดชบอร์ด TUI ใช้พื้นที่ทั้งเทอร์มินัล (ปรับขนาดแบบไดนามิก) และตอบสนองได้ตลอดเวลา:
ข้อมูลจะรีเฟรชเบื้องหลังตามช่วงเวลาที่กำหนด (`--interval`, ค่าเริ่มต้น 1 วินาที)
แม้การอ่าน `/proc` จะช้า การแสดงผลก็ยังดำเนินต่อไป

| ปุ่ม            | การทำงาน                             |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | สลับแท็บ                       |
| `←` / `→`      | นำทางแท็บย่อย (พูล/การกำหนดค่า)   |
| `↑` / `↓`      | เลื่อนมุมมองการตรวจสอบ         |
| `q`, `Ctrl-C`  | ออก                               |

## โครงสร้างโปรเจกต์

| ไฟล์              | หน้าที่                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, ตัวเลือก, การประสานงาน                         |
| `src/config.rs`   | การค้นหาและแยกวิเคราะห์การกำหนดค่า                 |
| `src/proc.rs`     | การอ่าน `/proc` (เวิร์กเกอร์, RSS, สถานะ, TCP backlog)  |
| `src/data.rs`     | การรวมข้อมูลเป็นแถวของตาราง                        |
| `src/logs.rs`     | การอ่านบันทึก PHP และคำสั่งที่ทำงานช้า                    |
| `src/render.rs`   | เอาต์พุตข้อความ (ตาราง CLI)                             |
| `src/tui.rs`      | แดชบอร์ดแบบอินเทอร์แอกทีฟ (4 แท็บ, กราฟ)              |
| `src/i18n.rs`     | การแปลภาษา (8 ภาษา, `--lang`)                |
| `src/term.rs`     | การตรวจจับสี / สไตล์                            |
| `tests/cli.rs`    | การทดสอบ CLI แบบครบวงจร (`--help`, `--mock`, i18n)      |
| `.github/workflows` | CI (การทดสอบ, ความครอบคลุม, 18 บิลด์, การทดสอบ QEMU) |

## การพัฒนา

```sh
cargo build          # บิลด์ (debug)
cargo test           # การทดสอบหน่วย + การทดสอบผสานรวม
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # การจัดรูปแบบ
```

CI รันชุดทดสอบบน Linux, macOS และ Windows บน MSRV (Rust 1.78)
เผยแพร่แบดจ์ความครอบคลุม คอมไพล์ข้ามแพลตฟอร์ม 18 เป้าหมายรีลีส และรันไบนารี
สำหรับสถาปัตยกรรมต่าง ๆ ภายใต้ QEMU

## สัญญาอนุญาต

[MIT](LICENSE)
