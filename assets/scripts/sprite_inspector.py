#!/usr/bin/env python3
"""Sprite inspection tool with per-sprite metadata and HTML gallery rendering."""

from __future__ import annotations

import argparse
import datetime as dt
import html
import json
import os
import sys
from pathlib import Path
from typing import Any, Dict, List

try:
    from PIL import Image
except ImportError:  # pragma: no cover - runtime environment dependent
    Image = None


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_ROOT = REPO_ROOT / "assets/sprites/future/nebula_bouncer"
DEFAULT_META = REPO_ROOT / "agents/art/reviews/NB-A4-006_sprite_metadata.json"
DEFAULT_HTML = REPO_ROOT / "agents/art/reviews/NB-A4-006_sprite_inspector.html"


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()


def rel_to_repo(path: Path) -> str:
    return path.resolve().relative_to(REPO_ROOT).as_posix()


def infer_class(name: str) -> str:
    lower = name.lower()
    if "player" in lower:
        return "player"
    if "scout" in lower:
        return "enemy_scout"
    if "heavy" in lower or "interceptor" in lower:
        return "enemy_heavy"
    if "wall" in lower:
        return "wall_tile"
    if "ground" in lower:
        return "ground_tile"
    if "projectile" in lower or "orb" in lower:
        return "projectile_core"
    return "unknown"


def infer_usage(asset_class: str) -> str:
    mapping = {
        "player": "player ship sprite",
        "enemy_scout": "fast/light enemy sprite",
        "enemy_heavy": "heavy or elite enemy sprite",
        "wall_tile": "tileable wall texture",
        "ground_tile": "tileable ground texture",
        "projectile_core": "projectile core VFX",
    }
    return mapping.get(asset_class, "TODO")


def default_entry(path: Path) -> Dict[str, Any]:
    asset_class = infer_class(path.name)
    return {
        "path": rel_to_repo(path),
        "filename": path.name,
        "class": asset_class,
        "usage": infer_usage(asset_class),
        "status": "PENDING",
        "pivot": "center",
        "facing": "up",
        "orientation_offset_deg": -90.0,
        "notes": "",
        "tags": [],
    }


def read_dimensions(path: Path) -> Dict[str, int]:
    if Image is None or not path.exists():
        return {"width": 0, "height": 0}
    with Image.open(path) as img:
        width, height = img.size
    return {"width": int(width), "height": int(height)}


def iter_pngs(root: Path) -> List[Path]:
    if not root.exists():
        return []
    return sorted(p for p in root.rglob("*.png") if p.is_file())


def load_metadata(path: Path, root: Path) -> Dict[str, Any]:
    if not path.exists():
        return {
            "version": 1,
            "generated_at": utc_now(),
            "root": rel_to_repo(root),
            "entries": [],
        }
    with path.open("r", encoding="utf-8") as f:
        data = json.load(f)
    if not isinstance(data, dict):
        raise ValueError(f"Metadata at {path} is not a JSON object")
    data.setdefault("version", 1)
    data.setdefault("generated_at", utc_now())
    data.setdefault("root", rel_to_repo(root))
    data.setdefault("entries", [])
    return data


def save_metadata(path: Path, data: Dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=True)
        f.write("\n")


def cmd_init(args: argparse.Namespace) -> int:
    root = Path(args.root).resolve()
    meta = Path(args.metadata).resolve()
    data = load_metadata(meta, root)

    indexed: Dict[str, Dict[str, Any]] = {}
    for entry in data.get("entries", []):
        if isinstance(entry, dict) and "path" in entry:
            indexed[str(entry["path"])] = entry

    seen = set()
    added = 0
    updated = 0

    for img_path in iter_pngs(root):
        key = rel_to_repo(img_path)
        seen.add(key)
        entry = indexed.get(key)
        if entry is None:
            entry = default_entry(img_path)
            indexed[key] = entry
            added += 1
        dims = read_dimensions(img_path)
        if entry.get("width") != dims["width"] or entry.get("height") != dims["height"]:
            entry["width"] = dims["width"]
            entry["height"] = dims["height"]
            updated += 1

    removed = 0
    if args.prune:
        for key in list(indexed.keys()):
            if key not in seen:
                del indexed[key]
                removed += 1

    data["generated_at"] = utc_now()
    data["root"] = rel_to_repo(root)
    data["entries"] = sorted(indexed.values(), key=lambda e: str(e.get("path", "")))
    save_metadata(meta, data)

    print(
        f"metadata updated: entries={len(data['entries'])}, added={added}, "
        f"dim_updates={updated}, removed={removed}"
    )
    print(f"metadata file: {meta}")
    return 0


def status_class(status: str) -> str:
    status_upper = (status or "PENDING").upper()
    if status_upper == "APPROVED":
        return "status-approved"
    if status_upper == "REJECTED":
        return "status-rejected"
    return "status-pending"


def html_row(output_html: Path, entry: Dict[str, Any]) -> str:
    rel_path = str(entry.get("path", ""))
    abs_path = REPO_ROOT / rel_path
    
    # Calculate relative path from the HTML file to the image for portable links
    try:
        # output_html is the file path, we need its parent dir
        img_rel = Path(os.path.relpath(abs_path, output_html.parent)).as_posix()
    except ValueError:
        # Fallback if paths are on different drives or completely disjoint
        img_rel = abs_path.as_uri()

    status = str(entry.get("status", "PENDING")).upper()
    tags = ", ".join(str(t) for t in entry.get("tags", []))
    notes = str(entry.get("notes", ""))
    usage = str(entry.get("usage", ""))
    sprite_class = str(entry.get("class", "unknown"))
    size = f"{entry.get('width', 0)}x{entry.get('height', 0)}"

    image_block = (
        f'<img loading="lazy" src="{html.escape(img_rel)}" alt="{html.escape(rel_path)}" />'
        if abs_path.exists()
        else '<div class="missing">MISSING</div>'
    )
    
    # ID for JS to find this entry
    card_id = html.escape(rel_path)

    return f"""
<article class="card {status_class(status)}" data-path="{card_id}" data-status="{status}">
  <div class="thumb">{image_block}</div>
  <div class="meta">
    <div class="row"><strong>File:</strong> <code>{html.escape(rel_path)}</code></div>
    <div class="row"><strong>Class:</strong> {html.escape(sprite_class)}</div>
    <div class="row"><strong>Size:</strong> {html.escape(size)}</div>
    <div class="row"><strong>Status:</strong> <span class="status-text">{html.escape(status)}</span></div>
    
    <div class="actions">
      <button onclick="setStatus(this, 'APPROVED')">Approve</button>
      <button onclick="setStatus(this, 'REJECTED')">Reject</button>
      <button onclick="setStatus(this, 'PENDING')">Reset</button>
    </div>
    
    <textarea placeholder="Notes..." oninput="updateNote(this)">{html.escape(notes)}</textarea>
  </div>
</article>
"""


def cmd_render(args: argparse.Namespace) -> int:
    meta = Path(args.metadata).resolve()
    out = Path(args.output).resolve()
    if not meta.exists():
        print(f"ERROR: metadata file not found: {meta}")
        return 2
    with meta.open("r", encoding="utf-8") as f:
        data = json.load(f)
    entries = list(data.get("entries", []))
    title = args.title or "Sprite Inspector"

    cards = "\n".join(html_row(out, entry) for entry in entries)
    
    js_block = """
    <script>
      const state = {};

      function setStatus(btn, newStatus) {
        const card = btn.closest('.card');
        const path = card.getAttribute('data-path');
        
        // Update visual
        card.classList.remove('status-pending', 'status-approved', 'status-rejected');
        card.classList.add('status-' + newStatus.toLowerCase());
        card.querySelector('.status-text').innerText = newStatus;
        card.setAttribute('data-status', newStatus);
        
        updateState(path, { status: newStatus });
      }

      function updateNote(area) {
        const card = area.closest('.card');
        const path = card.getAttribute('data-path');
        updateState(path, { notes: area.value });
      }

      function updateState(path, changes) {
        if (!state[path]) state[path] = {};
        Object.assign(state[path], changes);
        generateReport();
      }

      function generateReport() {
        // Collect all changes from the DOM state
        const lines = [];
        document.querySelectorAll('.card').forEach(card => {
            const path = card.getAttribute('data-path');
            const status = card.getAttribute('data-status');
            const note = card.querySelector('textarea').value;
            
            // Only report interesting things (approved/rejected or notes)
            // Or just report everything? Let's report changed items or items with status
            if (status !== 'PENDING' || note.trim().length > 0) {
                lines.push({ path, status, notes: note });
            }
        });
        
        const jsonStr = JSON.stringify(lines, null, 2);
        document.getElementById('report-out').value = jsonStr;
      }
      
      function copyReport() {
        const el = document.getElementById('report-out');
        el.select();
        document.execCommand('copy');
        alert('Copied JSON report to clipboard!');
      }
    </script>
    """

    body = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{html.escape(title)}</title>
  <style>
    body {{
      margin: 0;
      background: #11161f;
      color: #d5def0;
      font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, sans-serif;
    }}
    header {{
      position: sticky;
      top: 0;
      background: #0c1118;
      border-bottom: 1px solid #1f2a3a;
      padding: 14px 20px;
      z-index: 100;
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      gap: 20px;
    }}
    header h1 {{ margin: 0; font-size: 18px; }}
    header p {{ margin: 6px 0 0; font-size: 13px; color: #93a5c2; }}
    
    .controls {{
        display: flex;
        flex-direction: column;
        align-items: flex-end;
    }}
    textarea.report {{
        width: 300px;
        height: 60px;
        background: #000;
        color: #0f0;
        font-family: monospace;
        font-size: 10px;
        border: 1px solid #333;
    }}
    
    main {{
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
      gap: 14px;
      padding: 14px;
    }}
    .card {{
      background: #161e2a;
      border: 2px solid #253247;
      border-radius: 8px;
      overflow: hidden;
      display: flex;
      flex-direction: column;
    }}
    .status-approved {{ border-color: #2a7f5f; }}
    .status-rejected {{ border-color: #8d3f3f; }}
    .status-pending {{ border-color: #6c5e32; }}
    
    .thumb {{
      background: #0f141d;
      min-height: 200px;
      display: flex;
      align-items: center;
      justify-content: center;
      border-bottom: 1px solid #253247;
      padding: 10px;
    }}
    .thumb img {{
      max-width: 100%;
      max-height: 200px;
      object-fit: contain;
      image-rendering: pixelated;
    }}
    .missing {{ color: #d67777; font-weight: bold; }}
    
    .meta {{
      padding: 10px;
      font-size: 12px;
      display: flex;
      flex-direction: column;
      gap: 5px;
      flex: 1;
    }}
    .row {{ margin-bottom: 2px; }}
    code {{ color: #a8c8ff; word-break: break-all; }}
    
    .actions {{
        margin-top: 10px;
        display: flex;
        gap: 5px;
    }}
    button {{
        cursor: pointer;
        background: #253247;
        color: #fff;
        border: none;
        padding: 4px 8px;
        border-radius: 4px;
    }}
    button:hover {{ background: #354a6b; }}
    
    textarea {{
        background: #0f141d;
        color: #ccc;
        border: 1px solid #253247;
        margin-top: 5px;
        width: 95%;
        height: 40px;
        padding: 4px;
    }}
  </style>
</head>
<body>
  <header>
    <div>
      <h1>{html.escape(title)}</h1>
      <p>Generated at {html.escape(utc_now())}. Entries: {len(entries)}</p>
    </div>
    <div class="controls">
       <button onclick="copyReport()">Copy Report JSON</button>
       <textarea id="report-out" class="report" readonly>Make changes to generate report...</textarea>
    </div>
  </header>
  <main>{cards}</main>
  {js_block}
</body>
</html>
"""
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(body, encoding="utf-8")
    print(f"wrote HTML inspector: {out}")
    return 0


def cmd_report(args: argparse.Namespace) -> int:
    meta = Path(args.metadata).resolve()
    if not meta.exists():
        print(f"ERROR: metadata file not found: {meta}")
        return 2
    with meta.open("r", encoding="utf-8") as f:
        data = json.load(f)
    entries = list(data.get("entries", []))
    print(f"entries: {len(entries)}")
    by_status: Dict[str, int] = {}
    for entry in entries:
        status = str(entry.get("status", "PENDING")).upper()
        by_status[status] = by_status.get(status, 0) + 1
    for key in sorted(by_status.keys()):
        print(f"{key}: {by_status[key]}")
    for entry in entries:
        print(
            f"{entry.get('status', 'PENDING'):>9} | "
            f"{entry.get('class', 'unknown'):<14} | "
            f"{entry.get('usage', ''):<28} | "
            f"{entry.get('path', '')}"
        )
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Inspect sprite candidates with metadata.")
    sub = parser.add_subparsers(dest="command")

    p_init = sub.add_parser("init", help="Initialize/update metadata from sprite files.")
    p_init.add_argument("--root", default=str(DEFAULT_ROOT), help="Root folder to scan for PNG files.")
    p_init.add_argument("--metadata", default=str(DEFAULT_META), help="Metadata JSON path.")
    p_init.add_argument(
        "--prune",
        action="store_true",
        help="Remove metadata entries that no longer have a backing PNG file.",
    )

    p_render = sub.add_parser("render", help="Render HTML gallery from metadata.")
    p_render.add_argument("--metadata", default=str(DEFAULT_META), help="Metadata JSON path.")
    p_render.add_argument("--output", default=str(DEFAULT_HTML), help="Output HTML path.")
    p_render.add_argument("--title", default="NB-A4-005 Sprite Inspector", help="Page title.")

    p_report = sub.add_parser("report", help="Print metadata summary table.")
    p_report.add_argument("--metadata", default=str(DEFAULT_META), help="Metadata JSON path.")

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    if args.command == "init":
        return cmd_init(args)
    if args.command == "render":
        return cmd_render(args)
    if args.command == "report":
        return cmd_report(args)
    parser.print_help()
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
