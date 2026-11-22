<p align="center">
  <img src="docs/logo.png" alt="fpm-monitor" width="760">
</p>

# fpm-monitor

[![CI](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml/badge.svg)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![Version](https://img.shields.io/github/v/release/filoucrackeur/fpm-monitor?sort=semver)](https://github.com/filoucrackeur/fpm-monitor/releases)
[![Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/filoucrackeur/fpm-monitor/badges/coverage.json)](https://github.com/filoucrackeur/fpm-monitor/actions/workflows/ci.yml) [![License](https://img.shields.io/github/license/filoucrackeur/fpm-monitor)](LICENSE) [![MSRV](https://img.shields.io/badge/MSRV-1.78-blue)](Cargo.toml)
![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust) ![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue) ![Architectures](https://img.shields.io/badge/architectures-18%20targets-blueviolet) ![Static musl](https://img.shields.io/badge/static-musl-blue)
[![Docker](https://img.shields.io/badge/docker-ghcr.io-2496ED?logo=docker&logoColor=white)](https://github.com/filoucrackeur/fpm-monitor/pkgs/container/fpm-monitor) [![Downloads](https://img.shields.io/github/downloads/filoucrackeur/fpm-monitor/total)](https://github.com/filoucrackeur/fpm-monitor/releases) [![Issues](https://img.shields.io/github/issues/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/issues) [![PRs welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)](https://github.com/filoucrackeur/fpm-monitor/pulls) [![Last commit](https://img.shields.io/github/last-commit/filoucrackeur/fpm-monitor)](https://github.com/filoucrackeur/fpm-monitor/commits/main)

[English](README.md) | [Français](README.fr.md) | [简体中文](README.zh.md) | [العربية](README.ar.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [Deutsch](README.de.md) | [Русский](README.ru.md) | [हिन्दी](README.hi.md) | [Bahasa Indonesia](README.id.md) | **한국어** | [Norsk](README.no.md) | [Svenska](README.sv.md) | [Suomi](README.fi.md) | [Dansk](README.da.md) | [Ελληνικά](README.el.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md) | [Українська](README.uk.md) | [ไทย](README.th.md) | [Tiếng Việt](README.vi.md)

Rust로 작성된 PHP-FPM 풀 모니터입니다(`fpm-monitor.c`의 포팅). [Ember](https://github.com/alexandre-daubois/ember)에서
영감을 받은 대화형 터미널 대시보드를 제공합니다.
의존성이 없는 단일 바이너리로 배포되며 **Linux, macOS, Windows**에서
**18개 대상 아키텍처**를 지원합니다. 인터페이스는 `--lang`을 통해 22개
언어(기본값: 영어)로 제공되며, 이 문서는
22개 언어로 번역되어 있습니다.

<p align="center">
  <img src="docs/screenshot.en.animated.svg" alt="fpm-monitor TUI — 대화형 대시보드 (영어)" width="700">
</p>

## 기능

- **CLI 출력**: 풀 표(pool, type, workers, running, idle, backlog,
  max_children, max_requests, 메모리)와 워커별 상세 정보(`-v`: pid, 상태, RSS).
- **TUI 대시보드** (`-t`): 4개 탭을 갖춘 대화형 인터페이스.
  - **모니터링**: 풀과 해당 워커의 실시간 상태(풀별 메모리).
  - **그래프**: 풀마다 하나의 큰 차트(Ember 스타일)와 `←`/`→` 하위 탭:
    `running`, `workers`, `idle`, `backlog`(비어 있어도 항상 표시), 그리고
    점선 임계값 선: `max_children`(회색), `max_requests`(빨강),
    `min_spare`(분홍), `max_spare`(보라)가 `idle` 패널에 표시됩니다.
  - **로그**: 풀별로 PHP 로그(`error_log`), 느린 쿼리(`slowlog`),
    **액세스 로그**(`access.log`, HTTP 코드 색상화)의 최신 줄을 하위 탭으로
    표시합니다.
  - **설정**: 파일에서 읽은 지시문(`global` + 풀)을 하위 탭으로 표시합니다.
- 자동 설정 검색(일반적인 위치 + `include` 지시문).
- 네트워크 호출 없음: 모든 데이터를 `/proc`에서 직접 읽습니다(프로세스, RSS,
  상태, TCP 소켓 accept 큐). macOS와 BSD에서는 동일한 데이터를
  `ps`와 `netstat`에서 읽으며, FPM 상태 설정(`pm.status_path`)이 필요하지
  않습니다.
- **이식성 중심 설계**: 10개 Linux 아키텍처용 정적 musl 바이너리,
  나머지 아키텍처용 glibc 빌드, 그리고 macOS와 Windows 지원. 런타임
  의존성이 없고, libc 버전을 맞출 필요도 없습니다(문서화된 glibc 빌드 제외).

## 요구 사항

- 소스에서 컴파일하려면 **Rust 1.78+**(MSRV는 CI에서 테스트됨).
- **Linux**: 실제 데이터에는 `/proc`이 사용됩니다. TCP 소켓의 accept 큐는
  `/proc/net/tcp`에서 읽고, unix 소켓은 `listen.backlog`만 최댓값으로
  표시합니다.
- **macOS / FreeBSD / OpenBSD / NetBSD / DragonFly**: `/proc`이 없으며,
  프로세스와 소켓을 `ps`와 `netstat`으로 읽습니다.
- **Windows**: CLI는 빌드되고 실행되지만(`--help`, `--mock`), Windows에서는
  PHP-FPM을 사용할 수 없으므로 실시간 프로세스 검색은 명시적인 오류를
  반환합니다.

## 설치

### 미리 빌드된 바이너리

모든 [릴리스](https://github.com/filoucrackeur/fpm-monitor/releases)에는 지원되는
모든 대상의 원시 바이너리, 대상별 아카이브(Unix에서는 `.tar.gz`, Windows에서는
`.zip`), `.deb`/`.rpm` 패키지, Homebrew 포뮬러가 포함됩니다.

아카이브는 자체 완결형이며, 각 아카이브에는 `fpm-monitor` 실행 파일,
`LICENSE`, `README.md`가 들어 있습니다:

```sh
tar -xzf fpm-monitor-<version>-x86_64-unknown-linux-musl.tar.gz
./fpm-monitor-<version>-x86_64-unknown-linux-musl/fpm-monitor --help
```

또는 원시 바이너리를 직접 사용하세요:

```sh
chmod +x fpm-monitor-x86_64-unknown-linux-musl
./fpm-monitor-x86_64-unknown-linux-musl --help
```

### 지원 플랫폼과 아키텍처

| 플랫폼 | 아키텍처 | Rust 대상 | 배포 |
| -------- | ------------- | ------------ | ------------ |
| Linux (정적 musl) | amd64, arm64, armv7, armv6, armv5, 386, i586, riscv64, ppc64le, ppc64 | `x86_64-…-musl`, `aarch64-…-musl`, `armv7-…-musleabihf`, `arm-…-musleabihf`, `armv5te-…-musleabi`, `i686-…-musl`, `i586-…-musl`, `riscv64gc-…-musl`, `powerpc64le-…-musl`, `powerpc64-…-musl` | 바이너리, deb/rpm |
| Linux (glibc ≥ 2.17) | s390x, loong64, sparc64 | `s390x-unknown-linux-gnu`, `loongarch64-unknown-linux-gnu`, `sparc64-unknown-linux-gnu` | 바이너리, deb/rpm |
| macOS 11+ | Apple Silicon, Intel | `aarch64-apple-darwin`, `x86_64-apple-darwin` | 바이너리, Homebrew |
| Windows 10+ | x86_64, x86, arm64 | `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc` | `.exe` |
| Docker | amd64, arm64, ppc64le, riscv64 | `ghcr.io/filoucrackeur/fpm-monitor` | OCI 이미지 |

### 소스에서 빌드

```sh
cargo build --release
# 바이너리는 target/release/fpm-monitor 에 있습니다
```

### 교차 컴파일

CI는 [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)를 사용해
단일 x86_64 러너에서 모든 Linux 바이너리를 생성합니다:

```sh
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
# 바이너리는 target/aarch64-unknown-linux-musl/release/fpm-monitor 에 있습니다
```

위 표의 아무 트리플로나 대상을 바꾸세요. `php:*-fpm-alpine` 컨테이너
내부에 배포하려면:

```sh
docker cp target/aarch64-unknown-linux-musl/release/fpm-monitor php-fpm:/usr/local/bin/fpm-monitor
```

### Linux 패키지 (Red Hat / Debian)

`.deb` 및 `.rpm` 패키지가 각 릴리스에 amd64, arm64, armhf, armel, i386,
riscv64, ppc64el, ppc64, s390x, loong64, sparc64용으로 첨부됩니다. musl
패키지는 외부 의존성이 없어 최신 RHEL, Fedora, CentOS, Debian, Ubuntu 어느
버전에서든 작동합니다.

- **Red Hat, Fedora, CentOS**:
  ```sh
  sudo dnf install fpm-monitor-<version>-1.x86_64.rpm
  ```
- **Debian, Ubuntu**:
  ```sh
  sudo apt install ./fpm-monitor_<version>_amd64.deb
  ```
- 다른 아키텍처에서는 `amd64`/`x86_64`를 해당 패키지 이름(`arm64`, `armhf`,
  `armel`, `i386`, `riscv64`, `ppc64el`, `ppc64`, `s390x`, `loong64`,
  `sparc64`)으로 바꾸세요.

기존 `target/` 디렉터리에서 아카이브와 패키지를 로컬로 다시 빌드하려면:

```sh
mkdir -p dist/bin
for t in x86_64-unknown-linux-musl aarch64-unknown-linux-musl; do
  cp "target/$t/release/fpm-monitor" "dist/bin/fpm-monitor-$t"
done
scripts/release-artifacts.sh <version> dist/bin dist
scripts/build-packages.sh <version> dist/bin dist   # PATH에 nfpm 필요
```

### macOS (Homebrew)

```sh
brew tap filoucrackeur/fpm-monitor
brew install fpm-monitor
```

### Docker

멀티 아키텍처 이미지가 `linux/amd64`, `linux/arm64`, `linux/ppc64le`,
`linux/riscv64`용으로 GitHub Container Registry에 게시됩니다:

```sh
docker pull ghcr.io/filoucrackeur/fpm-monitor:latest
docker run --rm --pid=host -v /etc/php-fpm.d:/etc/php-fpm.d:ro \
  -it ghcr.io/filoucrackeur/fpm-monitor:latest -t
```

`--pid=host`를 사용하면 모니터가 호스트 `/proc`에서 PHP-FPM 프로세스를 읽을 수
있습니다. `-v`로 FPM 풀 설정을 마운트하면 풀이 검색됩니다(일반적인 `/etc`
경로도 스캔됩니다).

## 사용법

```
fpm-monitor [OPTIONS]

Options:
  -c, --config <PATH>     분석할 php-fpm.conf 파일 또는 풀 디렉터리
  -v, --verbose           각 워커의 상세 정보 표시 (pid, 상태, RSS)
      --color             색상 강제 적용
      --no-color          색상 비활성화
  -t, --tui               대화형 대시보드 (탭, `←`/`→` 하위 탭)
      --interval <SEC>    TUI 새로 고침 간격 (기본값 1)
      --lang <LANG>       인터페이스 언어: en, fr, zh, ar, es, it, ja, de,
                          ru, hi, id, ko, no, sv, fi, da, el, tr, pl, uk, th, vi
                          (기본값: en)
      --mock              데모 데이터 (로컬 테스트)
  -h, --help              이 도움말 표시
```

### 인터페이스 언어

`--lang`은 `en`(미국, 기본값), `fr`, `zh`, `ar`, `es`, `it`, `ja`, `de`, `ru`, `hi`, `id`, `ko`, `no`, `sv`, `fi`, `da`, `el`, `tr`, `pl`, `uk`, `th`, `vi`를
허용합니다(`--lang=fr` 형식도 허용됨). TUI 대시보드는 물론 CLI 출력(헤더,
요약, 범례)과 `--help`에도 적용됩니다.

### TUI 대시보드

TUI는 터미널 전체를 채우고(동적 크기 조정) 반응성을 유지합니다. 데이터는
설정된 간격(`--interval`, 기본 1초)으로 백그라운드에서 갱신되며, `/proc` 읽기가
느리더라도 렌더링은 계속됩니다.

| 키            | 동작                             |
| -------------- | ---------------------------------- |
| `1` – `4`, `Tab` | 탭 전환                       |
| `←` / `→`      | 하위 탭 탐색 (풀/설정)   |
| `↑` / `↓`      | 모니터링 보기 스크롤         |
| `q`, `Ctrl-C`  | 종료                               |

## 프로젝트 구조

| 파일              | 역할                                                |
| ----------------- | --------------------------------------------------- |
| `src/main.rs`     | CLI, 옵션, 오케스트레이션                         |
| `src/config.rs`   | 설정 검색 및 파싱                 |
| `src/proc.rs`     | `/proc` 읽기 (워커, RSS, 상태, TCP backlog)  |
| `src/data.rs`     | 데이터를 표 행으로 병합                        |
| `src/logs.rs`     | PHP 로그 및 느린 쿼리 읽기                    |
| `src/render.rs`   | 텍스트 출력 (CLI 표)                             |
| `src/tui.rs`      | 대화형 대시보드 (4개 탭, 그래프)              |
| `src/i18n.rs`     | 현지화 (22개 언어, `--lang`)                |
| `src/term.rs`     | 색상 감지 / 스타일                            |
| `tests/cli.rs`    | 엔드투엔드 CLI 테스트 (`--help`, `--mock`, i18n)      |
| `.github/workflows` | CI (테스트, 커버리지, 18개 빌드, QEMU 스모크 테스트) |

## 개발

```sh
cargo build          # 빌드 (디버그)
cargo test           # 단위 + 통합 테스트
cargo clippy --all-targets -- -D warnings
cargo fmt --check    # 포맷 검사
```

CI는 Linux, macOS, Windows에서 테스트 스위트를 실행하고, MSRV(Rust 1.78)에서
검증하며, 커버리지 배지를 게시하고, 18개 릴리스 대상을 교차 컴파일하며,
QEMU에서 외래 아키텍처 바이너리를 실행합니다.

## 기여

기여를 환영합니다! issue 또는 pull request를 열기 전에
[CONTRIBUTING.md](CONTRIBUTING.md)를 읽고 [행동 강령](CODE_OF_CONDUCT.md)을 따라
주세요. 사용 관련 질문은 [SUPPORT.md](SUPPORT.md)를 참고하세요. 보안 문제는
[SECURITY.md](SECURITY.md)에 설명된 대로 비공개로 신고해 주세요.

## 라이선스

[MIT](LICENSE)
