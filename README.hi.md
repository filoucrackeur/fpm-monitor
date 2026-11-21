<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | **हिन्दी** | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Rust में लिखा गया PHP-FPM पूल मॉनिटर (`fpm-monitor.c` का पोर्ट), जिसमें
[Ember](https://github.com/alexandre-daubois/ember) से प्रेरित एक इंटरैक्टिव
टर्मिनल डैशबोर्ड है। यह एक ही, बिना किसी निर्भरता वाली बाइनरी के रूप में वितरित होता
है और **Linux, macOS तथा Windows** पर, **18 लक्ष्य आर्किटेक्चर** पर चलता है।
`--lang` के माध्यम से इंटरफ़ेस 22 भाषाओं में उपलब्ध है (डिफ़ॉल्ट: अंग्रेज़ी), और यह
दस्तावेज़ 22 भाषाओं में अनुवादित है।

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — इंटरैक्टिव डैशबोर्ड (अंग्रेज़ी)" width="700">
</p>

## विशेषताएँ

- **CLI आउटपुट**: पूल तालिका (pool, type, workers, running, idle, backlog,
  max_children, max_requests, मेमोरी) तथा प्रत्येक worker का विवरण
  (`-v`: pid, स्थिति, RSS)।
- **TUI डैशबोर्ड** (`-t`): 4 टैब वाला इंटरैक्टिव इंटरफ़ेस।
  - **मॉनिटरिंग**: पूल और उनके worker की रीयल-टाइम स्थिति (प्रति पूल मेमोरी)।
  - **ग्राफ़**: प्रत्येक पूल के लिए एक बड़ा चार्ट (Ember शैली), जिसमें `←`/`→`
    उप-टैब हैं: `running`, `workers`, `idle`, `backlog` (हमेशा दिखते हैं, खाली होने
    पर भी), साथ ही डैशदार थ्रेशोल्ड रेखाएँ: `max_children` (धूसर), `max_requests`
    (लाल), और `idle` पैनल पर `min_spare` (गुलाबी) तथा `max_spare` (बैंगनी)।
  - **लॉग**: प्रत्येक पूल के लिए PHP लॉग (`error_log`), धीमी क्वेरी (`slowlog`) और
    **एक्सेस लॉग** (`access.log`, रंगीन HTTP कोड) की नवीनतम पंक्तियाँ, उप-टैब में।
  - **कॉन्फ़िगरेशन**: फ़ाइलों से पढ़े गए निर्देश (`global` + पूल), उप-टैब में।
- **वेब डैशबोर्ड** (`--watch`): ब्राउज़र में वही चार व्यू, बिना किसी बाहरी
  निर्भरता वाले HTTP सर्वर द्वारा `http://127.0.0.1:8471` पर उपलब्ध। पोर्ट
  `--port` से बदला जाता है; डेटा `--interval` पर पृष्ठभूमि में एकत्र होता है
  और पेज अपने ग्राफ़ के लिए 60 इतिहास बिंदु रखता है।
- कॉन्फ़िगरेशन की स्वचालित खोज (सामान्य स्थान + `include` निर्देश)।
- कोई नेटवर्क कॉल नहीं: सब कुछ सीधे `/proc` से पढ़ा जाता है (प्रक्रियाएँ, RSS,
  स्थिति, TCP सॉकेट स्वीकृति कतार)। macOS और BSD पर वही डेटा `ps` और `netstat` से
  पढ़ा जाता है; किसी FPM स्टेटस कॉन्फ़िगरेशन (`pm.status_path`) की आवश्यकता नहीं है।
- **डिज़ाइन से पोर्टेबल**: 10 Linux आर्किटेक्चर के लिए स्थिर musl बाइनरी, शेष के
  लिए glibc बिल्ड, साथ ही macOS और Windows। कोई रनटाइम निर्भरता नहीं, किसी libc
  संस्करण से मेल खाने की आवश्यकता नहीं (दस्तावेज़ में बताए गए glibc बिल्ड को छोड़कर)।

## आवश्यकताएँ

- स्रोत से कंपाइल करने के लिए **Rust 1.78+** (MSRV का परीक्षण CI में किया जाता है)।
- **Linux**: वास्तविक डेटा के लिए `/proc` का उपयोग होता है। TCP सॉकेट के लिए
  स्वीकृति कतार `/proc/net/tcp` से पढ़ी जाती है; unix सॉकेट के लिए केवल
  `listen.backlog` अधिकतम के रूप में दिखाया जाता है।
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: कोई `/proc` नहीं; प्रक्रियाएँ
  और सॉकेट `ps` तथा `netstat` से पढ़े जाते हैं।
- **Windows**: CLI बनता और चलता है (`--help`, `--mock`), लेकिन Windows पर PHP-FPM
  उपलब्ध नहीं है, इसलिए लाइव प्रक्रिया खोज एक स्पष्ट त्रुटि लौटाती है।

## स्थापना

### पूर्व-निर्मित बाइनरी

प्रत्येक [release](https://github.com/filoucrackeur/fpm-monitor/releases) में सभी
समर्थित लक्ष्यों के लिए मूल बाइनरी, प्रति लक्ष्य एक आर्काइव (Unix पर `.tar.gz`,
Windows पर `.zip`), `.deb`/`.rpm` पैकेज और एक Homebrew फ़ॉर्मूला शामिल होता है।

आर्काइव आत्म-निहित हैं: प्रत्येक में `fpm-monitor` निष्पादन योग्य फ़ाइल, `LICENSE`
और `README.md` होते हैं:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

या सीधे मूल बाइनरी का उपयोग करें:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### समर्थित प्लेटफ़ॉर्म और आर्किटेक्चर

| प्लेटफ़ॉर्म | आर्किटेक्चर | Rust लक्ष्य | वितरण |
| ----------- | ----------- | ----------- | ------ |
| Linux (स्थिर musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | बाइनरी, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | बाइनरी, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | बाइनरी, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI इमेज |

### स्रोत से

```sh
cargo build --release
# बाइनरी target/release/fpm-monitor पर है
```

### क्रॉस-कंपाइलेशन

CI एक ही x86_64 रनर से सभी Linux बाइनरी बनाने के लिए
[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) का उपयोग करता है:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# बाइनरी target/aarch64-unknown-linux-musl/release/fpm-monitor पर है
```

लक्ष्य को ऊपर की तालिका के किसी भी triple से बदलें। `php:*-fpm-alpine` कंटेनर के
अंदर तैनात करने के लिए:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux पैकेज (Red Hat / Debian)

प्रत्येक रिलीज़ के साथ amd64, arm64, armhf, armel, i386, riscv64, ppc64el, ppc64,
s390x, loong64 और sparc64 के लिए `.deb` और `.rpm` पैकेज संलग्न होते हैं। musl वाले
पैकेजों की कोई बाहरी निर्भरता नहीं होती और वे किसी भी हालिया RHEL, Fedora, CentOS,
Debian या Ubuntu संस्करण पर काम करते हैं।

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- अन्य आर्किटेक्चर पर, `amd64`/`x86_64` को संबंधित पैकेज नाम (`arm64`, `armhf`,
  `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`, `s390x`, `loong64`, `sparc64`)
  से बदलें।

मौजूदा `target/` निर्देशिका से स्थानीय रूप से आर्काइव और पैकेज फिर से बनाने के लिए:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # PATH में nfpm आवश्यक है
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

`linux/amd64`, `linux/arm64`, `linux/ppc64le` और `linux/riscv64` के लिए GitHub
Container Registry पर मल्टी-आर्क इमेज प्रकाशित है:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host` मॉनिटर को होस्ट के `/proc` से PHP-FPM प्रक्रियाएँ पढ़ने देता है; अपने
FPM पूल कॉन्फ़िगरेशन को `-v` से माउंट करें ताकि पूल खोजे जा सकें (सामान्य `/etc`
पथ भी स्कैन किए जाते हैं)।

## उपयोग

```
fpm-monitor [OPTIONS]

विकल्प:
  -c, --config <PATH>     विश्लेषण करने के लिए php-fpm.conf फ़ाइल या पूल निर्देशिका
  -v, --verbose           प्रत्येक worker का विवरण दिखाएँ (pid, स्थिति, RSS)
      --color             रंग बाध्य करें
      --no-color          रंग अक्षम करें
  -t, --tui               इंटरैक्टिव डैशबोर्ड (टैब, `←`/`→` उप-टैब)
      --interval <SEC>    TUI रीफ़्रेश अंतराल (डिफ़ॉल्ट 1)
      --watch             CLI व्यू के बजाय वेब डैशबोर्ड
      --port <PORT>       वेब डैशबोर्ड पोर्ट (डिफ़ॉल्ट 8471)
      --lang <LANG>       इंटरफ़ेस भाषा: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (डिफ़ॉल्ट: en)
      --mock              डेमो डेटा (स्थानीय परीक्षण)
  -h, --help              यह सहायता दिखाएँ
```

### इंटरफ़ेस भाषा

`--lang` `en` (US, डिफ़ॉल्ट), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi` स्वीकार
करता है (`--lang=fr` रूप भी स्वीकार्य है)। यह TUI डैशबोर्ड के साथ-साथ CLI आउटपुट
(हेडर, सारांश, लीजेंड) और `--help` पर भी लागू होता है।

### TUI डैशबोर्ड

TUI पूरे टर्मिनल को भरता है (गतिशील आकार परिवर्तन) और उत्तरदायी बना रहता है: डेटा
निर्धारित अंतराल (`--interval`, डिफ़ॉल्ट 1 s) पर पृष्ठभूमि में रीफ़्रेश होता है; यदि
`/proc` पढ़ने में देरी हो, तब भी रेंडरिंग जारी रहती है।

| कुंजी | क्रिया |
| ----- | ------ |
| `1` – `4`, `Tab` | टैब बदलें |
| `←` / `→` | उप-टैब में नेविगेट करें (पूल/कॉन्फ़िग) |
| `↑` / `↓` | मॉनिटरिंग दृश्य स्क्रॉल करें |
| `q`, `Ctrl-C` | बाहर निकलें |

### वेब डैशबोर्ड

`--watch` CLI तालिका छापने के बजाय वही डेटा सिंगल-पेज वेब ऐप के रूप में उपलब्ध
करता है:

```sh
fpm-monitor --watch                 # http://127.0.0.1:8471
fpm-monitor --watch --port 9000     # कस्टम पोर्ट
fpm-monitor --watch --mock          # PHP-FPM के बिना डेमो डेटा
```

सर्वर केवल लोकल इंटरफ़ेस पर सुनता है, बिना किसी बाहरी निर्भरता के, और पेज
`/api/state` से पूल, वर्कर विवरण, लॉग, कॉन्फ़िगरेशन निर्देश और अंतिम 60
नमूने लेता है। डेटा `--interval` (डिफ़ॉल्ट 1 सेकंड) पर पृष्ठभूमि में एकत्र होता
है, इसलिए धीमा स्कैन कभी अनुरोध नहीं रोकता। चार टैब हैं **मॉनिटरिंग**,
**ग्राफ़**, **लॉग** और **कॉन्फ़िगरेशन**; `1`–`4` और `←`/`→` टैब और सब-टैब
चुनते हैं।

`--port` `1`–`65535` स्वीकार करता है (`--port=9000` भी); कोई अन्य मान कोड `2`
के साथ बाहर निकलता है। सर्वर कभी कोई आउटगोइंग अनुरोध नहीं करता।

## परियोजना संरचना

| फ़ाइल | भूमिका |
| ----- | ------- |
| `src/main.rs` | CLI, विकल्प, ऑर्केस्ट्रेशन |
| `src/config.rs` | कॉन्फ़िगरेशन की खोज और पार्सिंग |
| `src/proc.rs` | `/proc` पढ़ना (worker, RSS, स्थिति, TCP backlog) |
| `src/data.rs` | डेटा को तालिका पंक्तियों में मिलाना |
| `src/logs.rs` | PHP लॉग और धीमी क्वेरी पढ़ना |
| `src/render.rs` | टेक्स्ट आउटपुट (CLI तालिका) |
| `src/tui.rs` | इंटरैक्टिव डैशबोर्ड (4 टैब, ग्राफ़) |
| `src/web.rs`       | वेब सर्वर, पृष्ठभूमि सैंपलिंग, JSON स्नैपशॉट           |
| `src/web/index.html` | एम्बेडेड सिंगल-पेज वेब डैशबोर्ड                  |
| `src/i18n.rs` | स्थानीयकरण (22 भाषाएँ, `--lang`) |
| `src/term.rs` | रंग पहचान / शैलियाँ |
| `tests/cli.rs` | एंड-टू-एंड CLI परीक्षण (`--help`, `--mock`, i18n) |
| `.github/workflows` | CI (परीक्षण, कवरेज, 18 बिल्ड, QEMU स्मोक परीक्षण) |

## विकास

```sh
cargo build          # बिल्ड (डीबग)
cargo test           # यूनिट + इंटीग्रेशन परीक्षण
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # फ़ॉर्मेटिंग
```

CI Linux, macOS और Windows पर MSRV (Rust 1.78) पर परीक्षण सूट चलाता है, कवरेज बैज
प्रकाशित करता है, 18 रिलीज़ लक्ष्यों को क्रॉस-कंपाइल करता है और विदेशी आर्किटेक्चर
की बाइनरी को QEMU के अंतर्गत निष्पादित करता है।

## लाइसेंस

[MIT](LICENSE)
