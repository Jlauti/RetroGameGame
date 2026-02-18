#!/usr/bin/env python3
"""Audit and optionally prune sibling RetroGameGame* work directories."""

from __future__ import annotations

import argparse
import shutil
import subprocess
from dataclasses import dataclass
from datetime import date, datetime
from pathlib import Path


@dataclass
class WorkdirInfo:
    path: Path
    branch: str
    dirty: bool
    ahead_of_origin_develop: int | None
    stash_count: int
    prune_safe: bool
    reason: str


def run_git(path: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", "-C", str(path), *args],
        check=False,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def collect_info(path: Path, canonical_name: str) -> WorkdirInfo:
    branch = run_git(path, "branch", "--show-current") or "unknown"
    dirty = bool(run_git(path, "status", "--porcelain"))
    stash_count = len(run_git(path, "stash", "list").splitlines())

    ahead = None
    ahead_raw = run_git(path, "rev-list", "--left-right", "--count", "origin/develop...HEAD")
    if ahead_raw:
        try:
            behind, ahead_n = ahead_raw.split()
            _ = int(behind)
            ahead = int(ahead_n)
        except Exception:
            ahead = None

    if path.name == canonical_name:
        return WorkdirInfo(path, branch, dirty, ahead, stash_count, False, "canonical workspace")
    if dirty:
        return WorkdirInfo(path, branch, dirty, ahead, stash_count, False, "dirty worktree")
    if stash_count > 0:
        return WorkdirInfo(path, branch, dirty, ahead, stash_count, False, "has stashes")
    if ahead is None:
        return WorkdirInfo(path, branch, dirty, ahead, stash_count, False, "ahead/behind unknown")
    if ahead > 0:
        return WorkdirInfo(path, branch, dirty, ahead, stash_count, False, "has local commits ahead")
    return WorkdirInfo(path, branch, dirty, ahead, stash_count, True, "safe to prune")


def write_report(report_path: Path, infos: list[WorkdirInfo], pruned: list[Path]) -> None:
    lines: list[str] = []
    lines.append(f"# Workspace Audit ({date.today().isoformat()})")
    lines.append("")
    lines.append(f"- generated_at: {datetime.now().isoformat(timespec='seconds')}")
    lines.append(f"- pruned_count: {len(pruned)}")
    lines.append("")
    lines.append("## Workspaces")
    lines.append("")
    for info in infos:
        lines.append(
            f"- {info.path}: branch={info.branch} dirty={info.dirty} "
            f"ahead_of_origin_develop={info.ahead_of_origin_develop} stash_count={info.stash_count} "
            f"prune_safe={info.prune_safe} reason={info.reason}"
        )
    lines.append("")
    lines.append("## Pruned")
    lines.append("")
    if pruned:
        for path in pruned:
            lines.append(f"- {path}")
    else:
        lines.append("- none")
    lines.append("")

    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Audit/prune RetroGameGame sibling workdirs")
    parser.add_argument("--root", default="/home/jl/git/RetroGameGame")
    parser.add_argument("--report", default="")
    parser.add_argument("--prune-safe", action="store_true")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    parent = root.parent
    canonical_name = root.name
    candidates = sorted(
        p for p in parent.glob("RetroGameGame*") if p.is_dir() and (p / ".git").exists()
    )
    infos = [collect_info(path, canonical_name) for path in candidates]

    pruned: list[Path] = []
    if args.prune_safe:
        for info in infos:
            if info.prune_safe:
                shutil.rmtree(info.path)
                pruned.append(info.path)

    default_report = (
        root
        / "agents"
        / "status"
        / "release"
        / f"workdir_audit_{date.today().isoformat()}.md"
    )
    report = Path(args.report).resolve() if args.report else default_report
    write_report(report, infos, pruned)
    print(f"workdir_audit={report}")
    print(f"workdir_candidates={len(infos)} pruned={len(pruned)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
