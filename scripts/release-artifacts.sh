#!/usr/bin/env bash
# Build release archives for every cross-compiled binary.
#
# Usage: scripts/release-artifacts.sh <version> <binary-dir> <output-dir>
#
# Each binary named fpm-monitor-<target>[.exe] is packed with LICENSE and
# README.md as:
#   fpm-monitor-<version>-<target>.tar.gz   (Unix)
#   fpm-monitor-<version>-<target>.zip      (Windows)
set -euo pipefail

if [ "$#" -ne 3 ]; then
  echo "usage: $0 <version> <binary-dir> <output-dir>" >&2
  exit 2
fi

VERSION="$1"
BIN_DIR="$2"
OUT_DIR="$3"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

mkdir -p "$OUT_DIR"
# Resolve to absolute paths: the Windows branch changes directory before
# invoking zip, and a relative output directory would then point inside the
# temporary staging tree.
OUT_DIR="$(cd "$OUT_DIR" && pwd)"
BIN_DIR="$(cd "$BIN_DIR" && pwd)"

shopt -s nullglob
found=0
for src in "$BIN_DIR"/fpm-monitor-*; do
  base="$(basename "$src")"
  case "$base" in
    *.tar.gz | *.tgz | *.zip | *.deb | *.rpm | *.rb | *.sha256) continue ;;
  esac
  [ -f "$src" ] || continue

  target="${base#fpm-monitor-}"
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT

  if [ "${base##*.}" = "exe" ]; then
    target="${target%.exe}"
    pkg="fpm-monitor-${VERSION}-${target}"
    mkdir -p "$tmp/$pkg"
    cp "$src" "$tmp/$pkg/fpm-monitor.exe"
    cp "$ROOT_DIR/LICENSE" "$ROOT_DIR/README.md" "$tmp/$pkg/"
    (cd "$tmp" && zip -q -r "$OUT_DIR/$pkg.zip" "$pkg")
    listing="$(unzip -Z1 "$OUT_DIR/$pkg.zip" | sed -e "s|^$pkg/||" -e "s|^$pkg$||" | grep -v '^$' | LC_ALL=C sort)"
    [ "$listing" = "$(printf 'LICENSE\nREADME.md\nfpm-monitor.exe')" ] || {
      echo "$pkg.zip contains unexpected files:" >&2
      printf '%s\n' "$listing" >&2
      exit 1
    }
    echo "archive: $OUT_DIR/$pkg.zip"
  else
    pkg="fpm-monitor-${VERSION}-${target}"
    mkdir -p "$tmp/$pkg"
    cp "$src" "$tmp/$pkg/fpm-monitor"
    chmod 0755 "$tmp/$pkg/fpm-monitor"
    cp "$ROOT_DIR/LICENSE" "$ROOT_DIR/README.md" "$tmp/$pkg/"
    tar -czf "$OUT_DIR/$pkg.tar.gz" -C "$tmp" "$pkg"
    listing="$(tar -tzf "$OUT_DIR/$pkg.tar.gz" | sed -e "s|^$pkg/||" -e "s|^$pkg$||" | grep -v '^$' | LC_ALL=C sort)"
    [ "$listing" = "$(printf 'LICENSE\nREADME.md\nfpm-monitor')" ] || {
      echo "$pkg.tar.gz contains unexpected files:" >&2
      printf '%s\n' "$listing" >&2
      exit 1
    }
    echo "archive: $OUT_DIR/$pkg.tar.gz"
  fi

  rm -rf "$tmp"
  trap - EXIT
  found=$((found + 1))
done

if [ "$found" -eq 0 ]; then
  echo "no fpm-monitor-* binary found in $BIN_DIR" >&2
  exit 1
fi
echo "created $found archive(s) in $OUT_DIR"
