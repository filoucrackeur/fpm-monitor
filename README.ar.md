# fpm-monitor

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | **العربية** | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md)

[![CI](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/ci.yml) [![Release](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml/badge.svg)](https://github.com/filoucrackeur/php-fpm-monitor/actions/workflows/release.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/php-fpm-monitor?sort=semver)](https://github.com/filoucrackeur/php-fpm-monitor/releases) [![codecov](https://img.shields.io/codecov/c/github/filoucrackeur/php-fpm-monitor)](https://codecov.io/github/filoucrackeur/php-fpm-monitor) [![Maintainability](https://img.shields.io/codeclimate/maintainability/filoucrackeur/php-fpm-monitor)](https://codeclimate.com/github/filoucrackeur/php-fpm-monitor) [![License](https://img.shields.io/github/license/filoucrackeur/php-fpm-monitor)](LICENSE) ![Static](https://img.shields.io/badge/static-musl-blue) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/php-fpm-monitor)](https://github.com/filoucrackeur/php-fpm-monitor/commits/main)

مراقب تجمعات PHP-FPM بلغة Rust (منفذ من `fpm-monitor.c`)، مع لوحة تحكم تفاعلية
في الطرفية مستوحاة من [Ember](https://github.com/alexandre-daubois/ember). الواجهة متوفرة
بـ 8 لغات (الافتراضي: الإنجليزية) عبر `--lang`.

## الميزات

- **مخرجات CLI**: جدول التجمعات (pool، type، workers، running، idle، backlog،
  max_children، max_requests، الذاكرة) مع تفاصيل كل عامل (`-v`: pid، الحالة، RSS).
- **لوحة التحكم TUI** (`-t`): واجهة تفاعلية بأربعة تبويبات.
  - **المراقبة**: الحالة اللحظية للتجمعات وعمالها (الذاكرة لكل تجمع).
  - **الرسوم البيانية**: رسم كبير لكل تجمع (بأسلوب Ember) مع تبويبات فرعية ← →:
    `running`، `workers`، `idle`، `backlog` (يُعرض دائمًا حتى لو كان فارغًا)، وخطوط
    عتبات متقطعة: `max_children` (رمادي)، `max_requests` (أحمر)، `min_spare` (وردي)
    و`max_spare` (بنفسجي) على لوحة `idle`.
  - **السجلات**: لكل تجمع، آخر أسطر سجل PHP (`error_log`)، والاستعلامات البطيئة
    (`slowlog`) وسجل **الوصول** (`access.log`، رموز HTTP ملوّنة)، في تبويبات فرعية.
  - **الإعدادات**: التوجيهات المقروءة من الملفات (`global` + التجمعات) في تبويبات فرعية.
- اكتشاف تلقائي للإعدادات (الأماكن المعتادة + توجيه `include`).
- بلا اتصالات شبكية: يُقرأ كل شيء مباشرة من `/proc` (العمليات، RSS، الحالة،
  قائمة انتظار القبول لسوكيتات TCP).

## المتطلبات

- Rust (stable) للترجمة.
- **Linux مع `/proc`** للبيانات الحقيقية. لا حاجة لإعداد حالة FPM
  (`pm.status_path`): البيانات تأتي من `/proc` ومن الإعداد. لسوكيتات TCP، يُقرأ
  backlog من `/proc/net/tcp` (قائمة انتظار القبول)؛ لسوكيتات unix، يُعرض فقط
  `listen.backlog` كحد أقصى.

## التثبيت

```sh
cargo build --release
# الملف التنفيذي في target/release/fpm-monitor
```

للنشر داخل حاوية `php:*-fpm-alpine` (musl، aarch64):

```sh
RUSTC=$HOME/.cargo/bin/rustc ~/.cargo/bin/cargo build --release --target aarch64-unknown-linux-musl
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### حزم Linux (Red Hat / Debian)

يُرفق مع كل [إصدار](https://github.com/filoucrackeur/php-fpm-monitor/releases) حزم `.deb` و`.rpm` ثابتة
(musl). بلا تبعيات خارجية، وتعمل على أي إصدار حديث من RHEL وFedora وCentOS وDebian أو
Ubuntu (x86_64 وARM64).

- **Red Hat، Fedora، CentOS**:
  ```sh
  sudo dnf install fpm-monitor_<version>_amd64.rpm
  ```
- **Debian، Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- على ARM64، استخدم حزم `arm64` بدلاً من ذلك.

### macOS (Homebrew)

يُولَّد ويُرفق مع كل إصدار ملف صيغة جاهز للاستخدام:

```sh
brew install https://github.com/filoucrackeur/php-fpm-monitor/releases/download/v<version>/fpm-monitor.rb
```

لاستضافته كـ tap، ضع الملف في
`<owner>/homebrew-fpm-monitor/Formula/fpm-monitor.rb`، ثم:

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

## الاستخدام

```
fpm-monitor [خيارات]

الخيارات:
  -c, --config <المسار>  ملف php-fpm.conf أو مجلد التجمعات للتحليل
  -v, --verbose          عرض تفاصيل كل عامل (pid، الحالة، RSS)
      --color            فرض الألوان
      --no-color         تعطيل الألوان
  -t, --tui              لوحة تحكم تفاعلية (تبويبات، تبويبات فرعية ← →)
      --interval <ث>     فترة تحديث الواجهة (الافتراضي 1)
      --lang <اللغة>     لغة الواجهة: en، fr، zh، ar، es، it، ja، de
                         (الافتراضي: en)
      --mock             بيانات تجريبية (اختبار محلي)
  -h, --help             عرض هذه المساعدة
```

### لغة الواجهة

يقبل `--lang` القيم `en` (أمريكية، الافتراضي)، `fr`، `zh`، `ar`، `es`، `it`، `ja`، `de`
(الصيغة `--lang=fr` مقبولة أيضًا). تنطبق على لوحة تحكم TUI وكذلك على مخرجات CLI
(الترويسات، الملخص، المفتاح الرموزي) وعلى `--help`.

### لوحة التحكم TUI

تملأ TUI كامل الطرفية (تغيير الحجم ديناميكيًا) وتبقى سريعة الاستجابة: تُحدَّث البيانات
في الخلفية بالوتيرة المضبوطة (`--interval`، الافتراضي 1 ثانية)؛ حتى لو كانت قراءة
`/proc` بطيئة، يستمر العرض.

| المفتاح          | الإجراء                              |
| ---------------- | ------------------------------------ |
| `1` – `4`، `Tab` | تبديل التبويب                        |
| `←` / `→`        | التنقل في التبويبات الفرعية (التجمعات/الإعدادات) |
| `↑` / `↓`        | تمرير عرض المراقبة                   |
| `q`، `Ctrl-C`    | الخروج                               |

## بنية المشروع

| الملف              | الدور                                     |
| ------------------ | ----------------------------------------- |
| `src/main.rs`      | CLI، الخيارات، التنسيق                    |
| `src/config.rs`    | اكتشاف الإعدادات وتحليلها                 |
| `src/proc.rs`      | قراءة `/proc` (العمال، RSS، الحالة، backlog TCP) |
| `src/data.rs`      | دمج البيانات في صفوف الجدول              |
| `src/logs.rs`      | قراءة سجل PHP والاستعلامات البطيئة        |
| `src/render.rs`    | مخرجات نصية (جدول CLI)                    |
| `src/tui.rs`       | لوحة التحكم التفاعلية (4 تبويبات، رسوم)   |
| `src/i18n.rs`      | التوطين (8 لغات، `--lang`)                |
| `src/term.rs`      | كشف الألوان / الأنماط                     |

## التطوير

```sh
cargo build          # ترجمة (debug)
cargo test           # تشغيل اختبارات الوحدة
cargo clippy         # فحص الكود
cargo fmt --check    # التنسيق
```

## الترخيص

[MIT](LICENSE)
