#!/usr/bin/env python3
"""Shared parsing helpers for agent-loop markdown artifacts."""

from __future__ import annotations

import re
from pathlib import Path
from typing import Dict, List

# Support canonical "- Key: Value" and common variants seen in agent outputs.
KV_PATTERNS = [
    re.compile(r"^\s*[-*]\s*\*\*([A-Za-z0-9 _/()\-]+):\*\*\s*(.*?)\s*$"),
    re.compile(r"^\s*\*\*([A-Za-z0-9 _/()\-]+):\*\*\s*(.*?)\s*$"),
    re.compile(r"^\s*[-*]\s*(?:\*\*)?([A-Za-z0-9 _/()\-]+?)(?:\*\*)?:\s*(.*?)\s*$"),
    re.compile(r"^\s*\*\*([A-Za-z0-9 _/()\-]+?)\*\*:\s*(.*?)\s*$"),
]
HEADING_RE = re.compile(r"^#{1,6}\s+(.*)$")


def normalize_key(key: str) -> str:
    cleaned = key.strip().replace("`", "")
    cleaned = re.sub(r"\s+", " ", cleaned)
    return cleaned.lower().replace(" ", "_").replace("/", "_")


def parse_kv_line(line: str) -> tuple[str, str] | None:
    for pattern in KV_PATTERNS:
        match = pattern.match(line)
        if not match:
            continue
        key = normalize_key(match.group(1))
        value = match.group(2).strip()
        if key and value:
            return key, value
    return None


def parse_metadata(text: str) -> Dict[str, str]:
    meta: Dict[str, str] = {}
    lines = text.splitlines()

    # Prefer explicit "## Metadata" section when present.
    in_metadata = False
    saw_metadata = False
    for line in lines:
        heading = HEADING_RE.match(line)
        if heading:
            title = heading.group(1).strip().lower()
            if in_metadata and title != "metadata":
                break
            in_metadata = title == "metadata"
            saw_metadata = saw_metadata or in_metadata
            continue
        if in_metadata:
            kv = parse_kv_line(line)
            if kv:
                key, value = kv
                meta[key] = value
    if meta or saw_metadata:
        return meta

    # Fallback for reports that put metadata in the file preamble before sections.
    for line in lines:
        if re.match(r"^##\s+", line):
            break
        kv = parse_kv_line(line)
        if kv:
            key, value = kv
            meta[key] = value

    if meta:
        return meta

    # Last-resort parse over the entire file.
    for line in lines:
        kv = parse_kv_line(line)
        if kv:
            key, value = kv
            meta[key] = value
    return meta


def parse_section_bullets(text: str, section_title: str) -> List[str]:
    lines = text.splitlines()
    section_title = section_title.strip().lower()
    in_section = False
    values: List[str] = []

    for line in lines:
        heading_match = HEADING_RE.match(line)
        if heading_match:
            title = heading_match.group(1).strip().lower()
            if in_section and title != section_title:
                break
            in_section = title == section_title
            continue

        if in_section:
            bullet = re.match(r"^\s*-\s+(.*\S)\s*$", line)
            if bullet:
                values.append(bullet.group(1))

    return values


def markdown_files(path: Path) -> List[Path]:
    if not path.exists():
        return []
    return sorted(p for p in path.glob("*.md") if p.is_file())
