#!/usr/bin/env bash
# Build the .deb and .rpm packages for every Linux architecture.
#
# Usage: scripts/build-packages.sh <version> <binary-dir> <output-dir>
#
# nfpm must be on PATH (or set NFPM=/path/to/nfpm). The binary directory is
# expected to contain fpm-monitor-<target> files produced by the build jobs.
set -euo pipefail

if [ "$#" -ne 3 ]; then
  echo "usage: $0 <version> <binary-dir> <output-dir>" >&2
  exit 2
fi

VERSION="$1"
BIN_DIR="$2"
OUT_DIR="$3"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NFPM="${NFPM:-nfpm}"

command -v "$NFPM" >/dev/null 2>&1 || {
  echo "nfpm not found; install it from https://github.com/goreleaser/nfpm" >&2
  exit 1
}

mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"
BIN_DIR="$(cd "$BIN_DIR" && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

# target | deb arch | rpm arch
PACKAGES='
x86_64-unknown-linux-musl      amd64    x86_64
aarch64-unknown-linux-musl     arm64    aarch64
armv7-unknown-linux-musleabihf armhf    armv7hl
armv5te-unknown-linux-musleabi armel    armv5tel
i686-unknown-linux-musl        i386     i686
riscv64gc-unknown-linux-musl   riscv64  riscv64
powerpc64le-unknown-linux-musl ppc64el  ppc64le
powerpc64-unknown-linux-musl   ppc64    ppc64
s390x-unknown-linux-gnu        s390x    s390x
loongarch64-unknown-linux-gnu  loong64  loongarch64
sparc64-unknown-linux-gnu      sparc64  sparc64
'

build=0
while read -r target deb rpm; do
  [ -n "$target" ] || continue
  src="$BIN_DIR/fpm-monitor-$target"
  if [ ! -f "$src" ]; then
    echo "skip: $target (binary not found)"
    continue
  fi

  cfg="$work/nfpm-$deb.yaml"
  sed -e "s|__ARCH__|$deb|g" \
      -e "s|__DEB_ARCH__|$deb|g" \
      -e "s|__RPM_ARCH__|$rpm|g" \
      -e "s|__VERSION__|$VERSION|g" \
      -e "s|__SRC__|$src|g" \
      "$ROOT_DIR/nfpm.yaml" > "$cfg"

  "$NFPM" package --packager deb --config "$cfg" --target "$OUT_DIR"
  "$NFPM" package --packager rpm --config "$cfg" --target "$OUT_DIR"

  deb_file="$OUT_DIR/fpm-monitor_${VERSION}_${deb}.deb"
  rpm_file="$OUT_DIR/fpm-monitor-${VERSION}-1.${rpm}.rpm"
  [ -f "$deb_file" ] || { echo "missing $deb_file" >&2; exit 1; }
  [ -f "$rpm_file" ] || { echo "missing $rpm_file" >&2; exit 1; }

  if command -v dpkg-deb >/dev/null 2>&1; then
    got="$(dpkg-deb -f "$deb_file" Architecture)"
    [ "$got" = "$deb" ] || { echo "$deb_file: arch $got != $deb" >&2; exit 1; }
    dpkg-deb -x "$deb_file" "$work/root"
    [ -x "$work/root/usr/local/bin/fpm-monitor" ] || {
      echo "$deb_file: binary missing or not executable" >&2
      exit 1
    }
    rm -rf "$work/root"
  fi

  if command -v rpm >/dev/null 2>&1; then
    got="$(rpm -qp --queryformat '%{ARCH}' "$rpm_file" 2>/dev/null)"
    [ "$got" = "$rpm" ] || { echo "$rpm_file: arch $got != $rpm" >&2; exit 1; }
  fi

  echo "package: $deb_file"
  echo "package: $rpm_file"
done <<< "$PACKAGES"

count="$(find "$OUT_DIR" -maxdepth 1 -type f \( -name '*.deb' -o -name '*.rpm' \) | wc -l | tr -d ' ')"
if [ "$count" -eq 0 ]; then
  echo "no package built" >&2
  exit 1
fi
echo "created $count package(s) in $OUT_DIR"
