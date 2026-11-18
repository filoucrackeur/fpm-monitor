#!/usr/bin/env python3
"""Validate the translated documentation.

Every README*.md file must link to all the other translations, reference only
local images that exist, and must not contain leftover placeholders.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def main() -> int:
    readmes = sorted(ROOT.glob("README*.md"))
    if len(readmes) < 2:
        print("error: no translated README found", file=sys.stderr)
        return 1

    names = {p.name for p in readmes}
    errors: list[str] = []

    for path in readmes:
        text = path.read_text(encoding="utf-8")

        for name in sorted(names - {path.name}):
            if f"]({name})" not in text:
                errors.append(f"{path.name}: missing link to {name}")

        for src in re.findall(r'src="([^"]+)"', text):
            if src.startswith(("http://", "https://", "data:")):
                continue
            if not (ROOT / src).is_file():
                errors.append(f"{path.name}: missing image {src}")

        for marker in ("TRANSLATE_ME", "TODO:", "LOREM IPSUM"):
            if marker in text:
                errors.append(f"{path.name}: leftover placeholder {marker!r}")

    for error in errors:
        print(f"error: {error}", file=sys.stderr)

    if errors:
        return 1

    print(f"ok: {len(readmes)} README files, all links and images are consistent")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
