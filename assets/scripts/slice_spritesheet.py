#!/usr/bin/env python3
import argparse
import json
import os
from dataclasses import dataclass
from typing import List, Tuple, Optional

import numpy as np
from PIL import Image

@dataclass
class Box:
    x0: int
    y0: int
    x1: int  # inclusive
    y1: int  # inclusive

    def expand(self, pad: int, w: int, h: int) -> "Box":
        return Box(
            max(0, self.x0 - pad),
            max(0, self.y0 - pad),
            min(w - 1, self.x1 + pad),
            min(h - 1, self.y1 + pad),
        )

    def to_xywh(self) -> Tuple[int, int, int, int]:
        return (self.x0, self.y0, self.x1 - self.x0 + 1, self.y1 - self.y0 + 1)

def connected_components(mask: np.ndarray) -> List[Box]:
    """8-connected components over a boolean mask."""
    h, w = mask.shape
    visited = np.zeros((h, w), dtype=np.uint8)
    boxes: List[Box] = []

    neigh = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]

    ys, xs = np.where(mask)
    if len(xs) == 0:
        return []

    for y, x in zip(ys, xs):
        if visited[y, x]:
            continue

        stack = [(y, x)]
        visited[y, x] = 1

        minx = maxx = x
        miny = maxy = y

        while stack:
            cy, cx = stack.pop()

            if cx < minx: minx = cx
            if cx > maxx: maxx = cx
            if cy < miny: miny = cy
            if cy > maxy: maxy = cy

            for dy, dx in neigh:
                ny, nx = cy + dy, cx + dx
                if 0 <= ny < h and 0 <= nx < w and (not visited[ny, nx]) and mask[ny, nx]:
                    visited[ny, nx] = 1
                    stack.append((ny, nx))

        boxes.append(Box(minx, miny, maxx, maxy))

    return boxes

def merge_boxes(boxes: List[Box], max_gap: int) -> List[Box]:
    """Merge boxes whose expanded regions overlap (helps with tiny disconnected pixels)."""
    if not boxes or max_gap <= 0:
        return boxes

    boxes = boxes[:]
    merged = True
    while merged:
        merged = False
        out: List[Box] = []
        used = [False] * len(boxes)

        for i, a in enumerate(boxes):
            if used[i]:
                continue

            ax0, ay0, ax1, ay1 = a.x0, a.y0, a.x1, a.y1

            gx0, gy0, gx1, gy1 = ax0 - max_gap, ay0 - max_gap, ax1 + max_gap, ay1 + max_gap

            changed = True
            while changed:
                changed = False
                for j, b in enumerate(boxes):
                    if used[j] or j == i:
                        continue
                    if not (b.x1 < gx0 or b.x0 > gx1 or b.y1 < gy0 or b.y0 > gy1):
                        used[j] = True
                        ax0 = min(ax0, b.x0)
                        ay0 = min(ay0, b.y0)
                        ax1 = max(ax1, b.x1)
                        ay1 = max(ay1, b.y1)
                        gx0, gy0, gx1, gy1 = ax0 - max_gap, ay0 - max_gap, ax1 + max_gap, ay1 + max_gap
                        changed = True
                        merged = True

            used[i] = True
            out.append(Box(ax0, ay0, ax1, ay1))

        boxes = out

    return boxes

def parse_bg_colors(s: str) -> List[Tuple[int,int,int]]:
    # "r,g,b; r,g,b; ..."
    parts = [p.strip() for p in s.split(";") if p.strip()]
    out = []
    for p in parts:
        r,g,b = [int(x.strip()) for x in p.split(",")]
        out.append((r,g,b))
    return out

def top_colors(rgb: np.ndarray, k: int) -> List[Tuple[int,int,int]]:
    """Return k most common RGB colors."""
    flat = rgb.reshape(-1, 3)
    # Pack to uint32 for fast counting
    packed = (flat[:,0].astype(np.uint32) << 16) | (flat[:,1].astype(np.uint32) << 8) | flat[:,2].astype(np.uint32)
    uniq, counts = np.unique(packed, return_counts=True)
    idx = np.argsort(counts)[::-1][:k]
    colors = []
    for u in uniq[idx]:
        r = (u >> 16) & 255
        g = (u >> 8) & 255
        b = u & 255
        colors.append((int(r), int(g), int(b)))
    return colors

def corner_colors(rgb: np.ndarray, sample: int = 5) -> List[Tuple[int,int,int]]:
    """Sample small squares at corners to get likely background colors."""
    h, w, _ = rgb.shape
    s = max(1, sample)
    corners = [
        rgb[0:s, 0:s, :],
        rgb[0:s, w-s:w, :],
        rgb[h-s:h, 0:s, :],
        rgb[h-s:h, w-s:w, :],
    ]
    vals = np.concatenate([c.reshape(-1,3) for c in corners], axis=0)
    # Unique
    packed = (vals[:,0].astype(np.uint32) << 16) | (vals[:,1].astype(np.uint32) << 8) | vals[:,2].astype(np.uint32)
    uniq = np.unique(packed)
    out = []
    for u in uniq[:64]:  # cap
        r = (u >> 16) & 255
        g = (u >> 8) & 255
        b = u & 255
        out.append((int(r), int(g), int(b)))
    return out

def build_mask_from_bg(rgb: np.ndarray, bg_colors: List[Tuple[int,int,int]], tol: int) -> np.ndarray:
    """Foreground mask: pixel is foreground if it's NOT within tol of ANY bg color."""
    h, w, _ = rgb.shape
    rgbf = rgb.astype(np.int16)

    bg = np.array(bg_colors, dtype=np.int16)  # Nx3
    # Compute per-pixel min L1 distance to bg colors (cheap and good for pixel art)
    # dist = min(|r-rb| + |g-gb| + |b-bb|)
    # shape: (H,W,N)
    diffs = np.abs(rgbf[:,:,None,:] - bg[None,None,:,:]).sum(axis=3)
    mind = diffs.min(axis=2)
    return mind > tol

def main():
    ap = argparse.ArgumentParser(description="Slice spritesheet into individual PNGs by transparency OR background colors.")
    ap.add_argument("input", help="Input PNG spritesheet")
    ap.add_argument("-o", "--outdir", default="out_slices", help="Output directory")
    ap.add_argument("--alpha-threshold", type=int, default=8,
                    help="Alpha > threshold counts as foreground (0-255). Default 8")
    ap.add_argument("--min-area", type=int, default=50,
                    help="Drop components with area smaller than this (noise). Default 50")
    ap.add_argument("--pad", type=int, default=1,
                    help="Padding pixels around each crop. Default 1")
    ap.add_argument("--merge-gap", type=int, default=0,
                    help="Merge boxes within this pixel gap (helps disconnected pixels). Default 0")
    ap.add_argument("--prefix", default="sprite", help="Output filename prefix")
    ap.add_argument("--write-manifest", action="store_true", help="Write manifest.json with boxes in output order")

    # Background handling
    ap.add_argument("--bg-auto", action="store_true",
                    help="Auto-detect background colors (use for baked checkerboards / opaque sheets).")
    ap.add_argument("--bg-colors", type=str, default=None,
                    help='Manual background colors: "r,g,b; r,g,b; ..."')
    ap.add_argument("--bg-tolerance", type=int, default=12,
                    help="Tolerance (L1 RGB distance) for background match. Default 12")
    ap.add_argument("--bg-topk", type=int, default=8,
                    help="When bg-auto: consider top K frequent colors. Default 8")
    ap.add_argument("--bg-corner-sample", type=int, default=6,
                    help="When bg-auto: sample NxN pixels at corners. Default 6")

    args = ap.parse_args()

    img = Image.open(args.input).convert("RGBA")
    w, h = img.size
    rgba = np.array(img, dtype=np.uint8)

    alpha = rgba[:,:,3]
    rgb = rgba[:,:,:3]

    # Decide mode:
    # - If bg-colors or bg-auto specified -> use background masking (and force alpha to 0 for bg pixels for output)
    # - Else -> use alpha masking
    bg_colors: Optional[List[Tuple[int,int,int]]] = None

    if args.bg_colors:
        bg_colors = parse_bg_colors(args.bg_colors)

    if args.bg_auto:
        # Corner colors are strong signal for background
        cc = corner_colors(rgb, sample=args.bg_corner_sample)
        tc = top_colors(rgb, k=args.bg_topk)

        # Intersect-ish: keep top colors that also appear in corners
        cc_set = set(cc)
        picked = [c for c in tc if c in cc_set]

        # Fallback: if none intersect, just take top 2 (common for checkerboard)
        if len(picked) == 0:
            picked = tc[:2]

        # Also add any corner colors that are extremely common (approx: in topk)
        bg_colors = picked

    if bg_colors:
        mask = build_mask_from_bg(rgb, bg_colors, tol=args.bg_tolerance)
        # Make bg transparent for nicer outputs
        rgba2 = rgba.copy()
        rgba2[~mask, 3] = 0
        img = Image.fromarray(rgba2, "RGBA")
    else:
        mask = alpha > args.alpha_threshold

    boxes = connected_components(mask)

    # Filter by area
    filtered: List[Box] = []
    for b in boxes:
        bw = b.x1 - b.x0 + 1
        bh = b.y1 - b.y0 + 1
        area = bw * bh
        if area >= args.min_area:
            filtered.append(b)

    filtered = merge_boxes(filtered, args.merge_gap)

    # Sort top-to-bottom, left-to-right
    filtered.sort(key=lambda b: (b.y0, b.x0))

    os.makedirs(args.outdir, exist_ok=True)

    manifest = {
        "input": os.path.abspath(args.input),
        "size": {"w": w, "h": h},
        "mode": "bg-colors" if bg_colors else "alpha",
        "bg_colors": bg_colors,
        "slices": []
    }

    for idx, b in enumerate(filtered):
        b2 = b.expand(args.pad, w, h)
        x, y, cw, ch = b2.to_xywh()
        crop = img.crop((x, y, x + cw, y + ch))

        name = f"{args.prefix}_{idx:03d}.png"
        outpath = os.path.join(args.outdir, name)
        crop.save(outpath)

        manifest["slices"].append({
            "name": name,
            "box": {"x": int(x), "y": int(y), "w": int(cw), "h": int(ch)}
        })

    if args.write_manifest:
        with open(os.path.join(args.outdir, "manifest.json"), "w", encoding="utf-8") as f:
            json.dump(manifest, f, indent=2)

    print(f"Saved {len(filtered)} slices to: {args.outdir}")
    if bg_colors:
        print(f"Background colors used: {bg_colors} (tolerance={args.bg_tolerance})")

if __name__ == "__main__":
    main()
