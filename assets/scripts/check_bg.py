#!/usr/bin/env python3
"""Validate sprite background quality and flag white-background contamination."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, List

try:
    from PIL import Image
except ImportError:  # pragma: no cover - runtime environment dependent
    Image = None

REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_SPRITE_DIR = REPO_ROOT / "assets/sprites/future/nebula_bouncer"


@dataclass
class Result:
    path: Path
    kind: str
    width: int
    height: int
    transparent_ratio: float
    border_opaque_ratio: float
    border_white_ratio: float
    corners_transparent: int
    ok: bool
    reason: str


def iter_pngs(paths: Iterable[str]) -> List[Path]:
    out: List[Path] = []
    for raw in paths:
        p = Path(raw)
        if p.is_dir():
            out.extend(sorted(x for x in p.rglob("*.png") if x.is_file()))
        elif p.is_file() and p.suffix.lower() == ".png":
            out.append(p)
    return out


def infer_kind(name: str) -> str:
    lower = name.lower()
    if any(k in lower for k in ("tile", "ground", "wall")):
        return "tile"
    return "sprite"


def analyze(path: Path, border: int, alpha_t: int, white_t: int) -> Result:
    img = Image.open(path).convert("RGBA")
    w, h = img.size
    pix = list(img.getdata())

    def idx(x: int, y: int) -> int:
        return y * w + x

    transparent_count = 0
    border_opaque = 0
    border_white_opaque = 0

    border_mask = [[False] * w for _ in range(h)]
    b = max(1, border)
    for y in range(h):
        for x in range(w):
            if y < b or y >= h - b or x < b or x >= w - b:
                border_mask[y][x] = True

    border_pixels = 0
    for y in range(h):
        for x in range(w):
            r, g, bl, a = pix[idx(x, y)]
            if a <= alpha_t:
                transparent_count += 1
            if border_mask[y][x]:
                border_pixels += 1
                if a > alpha_t:
                    border_opaque += 1
                    if r >= white_t and g >= white_t and bl >= white_t:
                        border_white_opaque += 1

    corners = [(0, 0), (w - 1, 0), (0, h - 1), (w - 1, h - 1)]
    corners_transparent = 0
    for x, y in corners:
        _, _, _, a = pix[idx(x, y)]
        if a <= alpha_t:
            corners_transparent += 1

    kind = infer_kind(path.name)
    total_pixels = max(1, w * h)
    tr = transparent_count / total_pixels
    bor = border_opaque / max(1, border_pixels)
    bwr = border_white_opaque / max(1, border_pixels)

    # Strict sprite checks:
    # - Transparent corners
    # - Low opaque border occupancy
    # - Low white opaque border occupancy
    if kind == "sprite":
        if corners_transparent < 4:
            return Result(path, kind, w, h, tr, bor, bwr, corners_transparent, False, "opaque corners")
        if bor > 0.06:
            return Result(path, kind, w, h, tr, bor, bwr, corners_transparent, False, "opaque border")
        if bwr > 0.02:
            return Result(path, kind, w, h, tr, bor, bwr, corners_transparent, False, "white border contamination")
    else:
        # Tiles can be opaque, but flag suspicious mostly-white borders.
        if bwr > 0.70:
            return Result(path, kind, w, h, tr, bor, bwr, corners_transparent, False, "tile is mostly white border")

    return Result(path, kind, w, h, tr, bor, bwr, corners_transparent, True, "ok")


def main() -> int:
    if Image is None:
        print("ERROR: Pillow is required. Activate the project venv and install dependencies.")
        print("Hint: source .venv/bin/activate && pip install pillow")
        return 2

    ap = argparse.ArgumentParser(description="Validate sprite alpha/background quality.")
    ap.add_argument(
        "paths",
        nargs="*",
        default=[str(DEFAULT_SPRITE_DIR)],
        help="PNG files or directories containing PNGs",
    )
    ap.add_argument("--border", type=int, default=2, help="Border thickness in pixels")
    ap.add_argument("--alpha-threshold", type=int, default=8, help="Alpha <= threshold is transparent")
    ap.add_argument("--white-threshold", type=int, default=245, help="RGB >= threshold counts as near-white")
    ap.add_argument("--strict", action="store_true", help="Exit non-zero on any failing file")
    args = ap.parse_args()

    files = iter_pngs(args.paths)
    if not files:
        print("No PNG files found.")
        return 2

    results = [analyze(p, args.border, args.alpha_threshold, args.white_threshold) for p in files]

    failures = [r for r in results if not r.ok]
    for r in results:
        status = "PASS" if r.ok else "FAIL"
        print(
            f"{status} | {r.path} | kind={r.kind} | size={r.width}x{r.height} | "
            f"corners_transparent={r.corners_transparent}/4 | border_opaque={r.border_opaque_ratio:.3f} | "
            f"border_white={r.border_white_ratio:.3f} | reason={r.reason}"
        )

    if failures:
        print(f"\nBackground validation: {len(failures)} fail / {len(results)} total")
    else:
        print(f"\nBackground validation: all {len(results)} files passed")

    if args.strict and failures:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
