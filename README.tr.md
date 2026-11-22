<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | **Türkçe** | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Rust ile yazılmış bir PHP-FPM havuz monitörü (`fpm-monitor.c` portu) olup
[Ember](https://github.com/alexandre-daubois/ember) esinli etkileşimli bir terminal
panosu sunar. Bağımlılık gerektirmeyen tek bir ikili dosya olarak dağıtılır ve
**Linux, macOS ve Windows** üzerinde **18 hedef mimaride** çalışır. Arayüz, `--lang`
ile 22 dilde (varsayılan: İngilizce) kullanılabilir ve bu belge 22 dile çevrilmiştir.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — etkileşimli pano (İngilizce)" width="700">
</p>

## Özellikler

- **CLI çıktısı**: havuz tablosu (havuz, tür, worker sayısı, çalışan, boşta, birikim,
  max_children, max_requests, bellek) ve worker başına ayrıntı (`-v`: pid, durum, RSS).
- **TUI panosu** (`-t`): 4 sekmeli etkileşimli arayüz.
  - **İzleme**: havuzların ve çalışanlarının gerçek zamanlı durumu (havuz başına bellek).
  - **Grafikler**: havuz başına tek bir büyük grafik (Ember tarzı); `←`/`→` ile
    `running`, `workers`, `idle`, `backlog` alt sekmeleri (boş olsa bile her zaman
    gösterilir), ayrıca kesikli eşik çizgileri: `max_children` (gri),
    `max_requests` (kırmızı), `min_spare` (pembe) ve `max_spare` (mor) `idle`
    panelinde.
  - **Günlükler**: havuz başına PHP günlüğünün (`error_log`), yavaş sorguların
    (`slowlog`) ve **erişim günlüğünün** (`access.log`, renklendirilmiş HTTP
    kodları) son satırları, alt sekmelerde.
  - **Yapılandırmalar**: dosyalardan okunan yönergeler (`global` + havuzlar),
    alt sekmelerde.
- Otomatik yapılandırma keşfi (olağan konumlar + `include` yönergesi).
- Ağ çağrısı yok: her şey doğrudan `/proc` üzerinden okunur (süreçler, RSS, durum,
  TCP soketi kabul kuyruğu). macOS ve BSD'de aynı veriler `ps` ve `netstat`
  üzerinden okunur; FPM durum yapılandırması (`pm.status_path`) gerekmez.
- **Tasarım gereği taşınabilir**: 10 Linux mimarisi için statik musl ikilileri,
  kalanlar için glibc derlemeleri, ayrıca macOS ve Windows. Çalışma zamanı
  bağımlılığı yok, eşleştirilecek libc sürümü yok (belgelenen glibc derlemeleri
  hariç).

## Gereksinimler

- Kaynaktan derlemek için **Rust 1.78+** (MSRV CI'da test edilir).
- **Linux**: gerçek veriler için `/proc` kullanılır. TCP soketlerinde kabul kuyruğu
  `/proc/net/tcp` üzerinden okunur; unix soketlerinde yalnızca `listen.backlog`
  üst sınır olarak gösterilir.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: `/proc` yoktur; süreçler ve
  soketler `ps` ile `netstat` kullanılarak okunur.
- **Windows**: CLI derlenir ve çalışır (`--help`, `--mock`), ancak Windows'ta
  PHP-FPM bulunmadığından canlı süreç keşfi açık bir hata döndürür.

## Kurulum

### Hazır ikili dosyalar

Her [sürüm](https://github.com/filoucrackeur/fpm-monitor/releases), desteklenen tüm
hedefler için ham ikili dosyalar, hedef başına bir arşiv (Unix'te `.tar.gz`,
Windows'ta `.zip`), `.deb`/`.rpm` paketleri ve bir Homebrew formülü içerir.

Arşivler kendi kendine yeterlidir: her biri `fpm-monitor` çalıştırılabilir dosyasını,
`LICENSE` ve `README.md` dosyasını içerir:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

Ya da ham ikili dosyayı doğrudan kullanın:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### Desteklenen platformlar ve mimariler

| Platform | Mimariler | Rust hedefleri | Dağıtım |
| -------- | ------------- | ------------ | ------------ |
| Linux (statik musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | ikili dosya, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | ikili dosya, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | ikili dosya, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI imajı |

### Kaynaktan

```sh
cargo build --release
# ikili dosya target/release/fpm-monitor konumundadır
```

### Çapraz derleme

CI, tüm Linux ikili dosyalarını tek bir x86_64 çalıştırıcısından üretmek için
[cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) kullanır:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# ikili dosya target/aarch64-unknown-linux-musl/release/fpm-monitor konumundadır
```

Hedefi yukarıdaki tablodaki herhangi bir üçlüyle değiştirin. Bir `php:*-fpm-alpine`
konteyneri içine dağıtmak için:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux paketleri (Red Hat / Debian)

Her sürüme amd64, arm64, armhf, armel, i386, riscv64, ppc64el, ppc64, s390x,
loong64 ve sparc64 için `.deb` ve `.rpm` paketleri eklenir. musl paketlerinin harici
bağımlılığı yoktur ve güncel herhangi bir RHEL, Fedora, CentOS, Debian veya Ubuntu
sürümünde çalışır.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- Diğer mimarilerde `amd64`/`x86_64` yerine ilgili paket adını (`arm64`, `armhf`,
  `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`, `s390x`, `loong64`, `sparc64`)
  kullanın.

Arşivleri ve paketleri mevcut bir `target/` dizininden yerel olarak yeniden
oluşturmak için:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # PATH üzerinde nfpm gerekir
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

`linux/amd64`, `linux/arm64`, `linux/ppc64le` ve `linux/riscv64` için GitHub
Container Registry üzerinde çok mimarili bir imaj yayımlanır:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host`, monitörün PHP-FPM süreçlerini ana makinenin `/proc` dizininden
okumasını sağlar; havuzların keşfedilmesi için FPM havuz yapılandırmanızı `-v` ile
bağlayın (olağan `/etc` yolları da taranır).

## Kullanım

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     Analiz edilecek php-fpm.conf dosyası veya havuz dizini
  -v, --verbose           Her çalışanın ayrıntılarını göster (pid, durum, RSS)
      --color             Rengi zorla
      --no-color          Rengi devre dışı bırak
  -t, --tui               Etkileşimli pano (sekmeler, `←`/`→` alt sekmeler)
      --interval <SEC>    TUI yenileme aralığı (varsayılan 1)
      --lang <LANG>       Arayüz dili: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (varsayılan: en)
      --mock              Demo verileri (yerel test)
  -h, --help              Bu yardımı göster
```

### Arayüz dili

`--lang`, `en` (ABD, varsayılan), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`
değerlerini kabul eder (`--lang=fr` biçimi de kabul edilir). Hem TUI panosuna hem de
CLI çıktısına (başlıklar, özet, açıklama) ve `--help` çıktısına uygulanır.

### TUI panosu

TUI tüm terminali kaplar (dinamik yeniden boyutlandırma) ve yanıt vermeye devam
eder: veriler arka planda yapılandırılan aralıkta (`--interval`, varsayılan 1 sn)
yenilenir; bir `/proc` okuması yavaş olsa bile çizim sürer.

| Tuş            | Eylem                              |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | Sekme değiştir                   |
| `←` / `→`      | Alt sekmeler arasında gezin (havuzlar/yapılandırma) |
| `↑` / `↓`      | İzleme görünümünü kaydır           |
| `q`, `Ctrl-C`  | Çık                                |

## Proje yapısı

| Dosya             | Görev                                               |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, seçenekler, düzenleme                          |
| `src/config.rs`   | Yapılandırma keşfi ve ayrıştırma                    |
| `src/proc.rs`     | `/proc` okuma (çalışanlar, RSS, durum, TCP birikimi) |
| `src/data.rs`     | Verileri tablo satırlarında birleştirme             |
| `src/logs.rs`     | PHP günlüğü ve yavaş sorguları okuma                |
| `src/render.rs`   | Metin çıktısı (CLI tablosu)                         |
| `src/tui.rs`      | Etkileşimli pano (4 sekme, grafikler)               |
| `src/i18n.rs`     | Yerelleştirme (22 dil, `--lang`)                     |
| `src/term.rs`     | Renk algılama / stiller                             |
| `tests/cli.rs`    | Uçtan uca CLI testleri (`--help`, `--mock`, i18n)   |
| `.github/workflows` | CI (testler, kapsam, 18 derleme, QEMU duman testleri) |

## Geliştirme

```sh
cargo build          # derle (hata ayıklama)
cargo test           # birim + entegrasyon testleri
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # biçimlendirme
```

CI, test paketini Linux, macOS ve Windows üzerinde çalıştırır, MSRV'yi (Rust 1.78)
doğrular, kapsam rozetini yayımlar, 18 sürüm hedefini çapraz derler ve yabancı
mimari ikililerini QEMU altında çalıştırır.

## Katkıda bulunma

Katkılar memnuniyetle karşılanır! Bir issue veya pull request açmadan önce
[CONTRIBUTING.md](CONTRIBUTING.md) dosyasını okuyun, [Davranış Kuralları](CODE_OF_CONDUCT.md)
kurallarına uyun ve kullanım soruları için [SUPPORT.md](SUPPORT.md) dosyasına bakın.
Güvenlik sorunlarını [SECURITY.md](SECURITY.md) dosyasında açıklandığı şekilde özel
olarak bildirin.

## Lisans

[MIT](LICENSE)
