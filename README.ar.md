<p align="center" dir="rtl">
  <img src="docs/logo.png" alt="fpm-monitor" width="160">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | **العربية** | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | [한국어](README.ko.md) | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

مراقب تجمّعات PHP-FPM مكتوب بلغة Rust (نسخة منقولة من `fpm-monitor.c`)، مع لوحة
تحكّم تفاعلية في الطرفية مستوحاة من [Ember](https://github.com/alexandre-daubois/ember).
يُوزَّع كملف تنفيذي واحد بلا أي اعتماديات ويعمل على **Linux وmacOS وWindows**،
وذلك عبر **18 بنية هدف**. الواجهة متاحة بـ 8 لغات (الافتراضية: الإنجليزية) عبر
`--lang`، وهذا التوثيق مترجم إلى 22 لغة.

<p align="center">
  <img src="docs/screenshot.ar.animated.svg" alt="لوحة تحكّم fpm-monitor التفاعلية (العربية)" width="700">
</p>

## الميزات

- **مخرجات سطر الأوامر (CLI)**: جدول التجمّعات (pool، type، workers، running،
  idle، backlog، max_children، max_requests، الذاكرة) مع تفاصيل كل عامل
  (`-v`: pid، الحالة، RSS).
- **لوحة تحكّم TUI** (`-t`): واجهة تفاعلية بأربعة تبويبات.
  - **المراقبة**: الحالة اللحظية للتجمّعات وعامليها (الذاكرة لكل تجمّع).
  - **الرسوم البيانية**: رسم بياني كبير واحد لكل تجمّع (بأسلوب Ember) مع تبويبات
    فرعية `←`/`→`: `running`، `workers`، `idle`، `backlog` (تُعرض دائمًا، حتى
    عندما تكون فارغة)، إضافة إلى خطوط عتبات متقطّعة: `max_children` (رمادي)،
    `max_requests` (أحمر)، `min_spare` (وردي) و`max_spare` (بنفسجي) في لوحة
    `idle`.
  - **السجلات**: لكل تجمّع، أحدث أسطر سجل PHP (`error_log`)، والاستعلامات
    البطيئة (`slowlog`) و**سجل الوصول** (`access.log`، مع تلوين رموز HTTP)، في
    تبويبات فرعية.
  - **الإعدادات**: التوجيهات المقروءة من الملفات (`global` + التجمّعات) في
    تبويبات فرعية.
- اكتشاف تلقائي للإعدادات (المواقع المعتادة + التوجيه `include`).
- بلا أي اتصالات شبكية: كل شيء يُقرأ مباشرة من `/proc` (العمليات، RSS، الحالة،
  قائمة انتظار القبول لمآخذ TCP). على macOS وBSD، تُقرأ البيانات نفسها من `ps`
  و`netstat`؛ ولا حاجة إلى أي إعداد لحالة FPM (`pm.status_path`).
- **محمول بالتصميم**: ملفات تنفيذية musl ثابتة لعشر بنيات Linux، وبناءات glibc
  للبنيات المتبقية، إضافة إلى macOS وWindows. لا اعتمادية على وقت التشغيل، ولا
  حاجة لمطابقة إصدار libc (باستثناء بناءات glibc الموثّقة).

## المتطلبات

- **Rust 1.78+** للترجمة من المصدر (تُختبر MSRV في CI).
- **Linux**: يُستخدم `/proc` للبيانات الحقيقية. بالنسبة لمآخذ TCP، تُقرأ قائمة
  انتظار القبول من `/proc/net/tcp`؛ وبالنسبة لمآخذ unix لا يُعرض سوى
  `listen.backlog` كحدّ أقصى.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: لا وجود لـ `/proc`؛ تُقرأ
  العمليات والمآخذ باستخدام `ps` و`netstat`.
- **Windows**: تُبنى واجهة CLI وتعمل (`--help`، `--mock`)، لكن PHP-FPM غير متاح
  على Windows، لذا يُرجع الاكتشاف المباشر للعمليات خطأً صريحًا.

## التثبيت

### ملفات تنفيذية جاهزة

يتضمّن كل [إصدار](https://github.com/filoucrackeur/fpm-monitor/releases) ملفات
تنفيذية خام لجميع الأهداف المدعومة، وأرشيفًا لكل هدف (`.tar.gz` على Unix،
`.zip` على Windows)، وحزم `.deb`/`.rpm`، وصيغة Homebrew.

الأرشيفات مكتفية بذاتها: يحتوي كل منها على الملف التنفيذي `fpm-monitor`
و`LICENSE` و`README.md`:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

أو استخدم الملف التنفيذي الخام مباشرة:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### المنصّات والبنيات المدعومة

| المنصّة | البنيات | أهداف Rust | التوزيع |
| -------- | ------------- | ------------ | ------------ |
| Linux (musl ثابت) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | ملف تنفيذي، deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | ملف تنفيذي، deb/rpm |
| macOS 11+ | Apple Silicon، Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | ملف تنفيذي، Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | صورة OCI |

### من المصدر

```sh
cargo build --release
# الملف التنفيذي موجود في target/release/fpm-monitor
```

### الترجمة المتقاطعة

تستخدم CI أداة [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
لإنتاج جميع ملفات Linux التنفيذية من مشغّل x86_64 واحد:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# الملف التنفيذي موجود في target/aarch64-unknown-linux-musl/release/fpm-monitor
```

استبدل الهدف بأي ثلاثية من الجدول أعلاه. للنشر داخل حاوية
`php:*-fpm-alpine`:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### حزم Linux (Red Hat / Debian)

تُرفق حزم `.deb` و`.rpm` بكل إصدار لـ amd64 وarm64 وarmhf وarmel وi386
وriscv64 وppc64el وppc64 وs390x وloong64 وsparc64. أما حزم musl فلا تملك أي
اعتمادية خارجية وتعمل على أي إصدار حديث من RHEL أو Fedora أو CentOS أو Debian
أو Ubuntu.

- **Red Hat وFedora وCentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian وUbuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- على البنيات الأخرى، استبدل `amd64`/`x86_64` باسم الحزمة المطابق (`arm64`،
  `armhf`، `armel`، `i386`، `riscv64`، `ppc64el`، `ppc64`، `s390x`، `loong64`،
  `sparc64`).

لإعادة بناء الأرشيفات والحزم محليًا انطلاقًا من دليل `target/` موجود:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # يحتاج إلى nfpm في PATH
```

### macOS (Homebrew)

تُولَّد صيغة جاهزة للاستخدام وتُرفق بكل إصدار:

```sh
brew install https://github.com/filoucrackeur/fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

لاستضافتها كـ tap بدلًا من ذلك، انقل الملف إلى
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`، ثم:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

تُنشر صورة متعدّدة البنيات على GitHub Container Registry لـ `linux/amd64`
و`linux/arm64` و`linux/ppc64le` و`linux/riscv64`:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

يتيح `--pid=host` للمراقب قراءة عمليات PHP-FPM من `/proc` الخاص بالمضيف؛
وتركيب إعدادات تجمّعات FPM باستخدام `-v` ليكتشف التجمّعات (تُفحص مسارات `/etc`
المعتادة أيضًا).

## الاستخدام

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     ملف php-fpm.conf أو دليل التجمّعات المراد تحليله
  -v, --verbose           إظهار تفاصيل كل عامل (pid، الحالة، RSS)
      --color             فرض الألوان
      --no-color          تعطيل الألوان
  -t, --tui               لوحة تحكّم تفاعلية (تبويبات، تبويبات فرعية `←`/`→`)
      --interval <SEC>    فاصل تحديث TUI (الافتراضي 1)
      --lang <LANG>       لغة الواجهة: en, fr, zh, ar, es, it, ja, de
                          (الافتراضي: en)
      --mock              بيانات تجريبية (اختبار محلي)
  -h, --help              عرض هذه المساعدة
```

### لغة الواجهة

يقبل `--lang` القيم `en` (الولايات المتحدة، الافتراضي)، `fr`، `zh`، `ar`، `es`،
`it`، `ja`، `de` (وتُقبل أيضًا الصيغة `--lang=fr`). تنطبق على لوحة تحكّم TUI
كما على مخرجات CLI (العناوين، الملخّص، مفتاح الرموز) وعلى `--help`.

### لوحة تحكّم TUI

تملأ TUI الطرفية بأكملها (تحجيم ديناميكي) وتبقى سريعة الاستجابة: تُحدَّث
البيانات في الخلفية وفق الفاصل المحدَّد (`--interval`، الافتراضي 1 ثانية)؛ وحتى
إذا كانت قراءة `/proc` بطيئة، يستمر العرض.

| المفتاح        | الإجراء                            |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | التبديل بين التبويبات            |
| `←` / `→`      | التنقّل بين التبويبات الفرعية (التجمّعات/الإعدادات) |
| `↑` / `↓`      | التمرير في عرض المراقبة            |
| `q`, `Ctrl-C`  | الخروج                              |

## بنية المشروع

| الملف              | الدور                                               |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI، الخيارات، التنسيق العام                        |
| `src/config.rs`   | اكتشاف الإعدادات وتحليلها                           |
| `src/proc.rs`     | قراءة `/proc` (العاملون، RSS، الحالة، backlog لـ TCP) |
| `src/data.rs`     | دمج البيانات في صفوف الجدول                         |
| `src/logs.rs`     | قراءة سجل PHP والاستعلامات البطيئة                  |
| `src/render.rs`   | الإخراج النصي (جدول CLI)                            |
| `src/tui.rs`      | لوحة التحكّم التفاعلية (4 تبويبات، رسوم بيانية)     |
| `src/i18n.rs`     | الترجمة (8 لغات، `--lang`)                          |
| `src/term.rs`     | اكتشاف الألوان / الأنماط                            |
| `tests/cli.rs`    | اختبارات CLI من طرف إلى طرف (`--help`، `--mock`، i18n) |
| `.github/workflows` | CI (الاختبارات، التغطية، 18 بناءً، اختبارات دخان QEMU) |

## التطوير

```sh
cargo build          # البناء (وضع التصحيح)
cargo test           # اختبارات الوحدات + التكامل
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # التنسيق
```

تشغّل CI مجموعة الاختبارات على Linux وmacOS وWindows، وعلى MSRV (Rust 1.78)،
وتنشر شارة التغطية، وترجّم 18 هدف إصدار ترجمة متقاطعة، وتنفّذ الملفات التنفيذية
الخاصة بالبنيات الأجنبية تحت QEMU.

## الترخيص

[MIT](LICENSE)
