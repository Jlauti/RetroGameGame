# Task Report - NB-A2-009 (Topography Visual Readability Hotfix)

## Status
- **Ticket ID**: NB-A2-009
- **Assignee**: Pekka Kone (Agent 2)
- **Status**: COMPLETED (Revised)

## Root Cause

Three issues combined to create the visual regression:

1. **Rectangle shapes**: Topography sprites had no texture image (used Bevy's default white pixel), rendering as solid colored rectangles instead of hex shapes.
2. **Wall-to-wall coverage**: Every cell (including Tier 0, ~25% of grid) spawned a sprite, creating a dense colored carpet across the entire chunk.
3. **Camera2d bleed**: A leftover `Camera2d` from the menu/era-select screen persisted into the game scene, rendering all 2D sprites from a straight top-down perspective at camera order 0, visually dominating the 3D content.

## Changes Implemented

### New Asset
- **`assets/sprites/future/nebula_bouncer/hex_outline.png`** — Procedurally generated 128×128 hex outline (white on transparent). Generated via `scripts/gen_hex_outline.py`.

### Modified Files

#### `src/eras/era_future/nebula_bouncer/topography.rs` (rewritten)
- Sprites now use `hex_outline.png` texture → actual hex shapes instead of rectangles
- **Tier 0 skipped entirely** — lowest elevation renders nothing, reducing entity count ~25%
- **Tier 3 double-spawn removed** — simplified to single hex per cell
- **Reduced alpha values**: Tier 1=0.10, Tier 2=0.18, Tier 3=0.28
- **Hex size**: 88% of cell dimensions (slight gaps for readability)
- Function signature updated: now takes `&AssetServer` to load hex texture

#### `src/eras/era_future/nebula_bouncer/systems.rs`
- Updated `spawn_chunk_topography` call site to pass `asset_server`
- **Camera2d cleanup**: `setup_nebula_bouncer` now queries and despawns any leftover `Camera2d` entities from previous screens, preventing 2D sprites from rendering over 3D content

## Constants Changed (Before → After)

| Constant | Before (original) | After |
|---|---|---|
| Tier 0 alpha | 0.18 | Skipped (not rendered) |
| Tier 1 alpha | 0.38 | 0.10 |
| Tier 2 alpha | 0.48 | 0.18 |
| Tier 3 alpha | 0.70 | 0.28 |
| Hex sprite shape | Rectangle (no texture) | Hex outline (textured) |
| Hex sprite scale | 1.00 → 0.72 | 0.88 |
| Tier 3 glow overlay | Additional sprite at 0.10 alpha | Removed |

## What Was Preserved

- **Deterministic generation**: `generate_chunk_topography(height, global_seed, sequence_index)` unchanged
- **Depth ordering**: Topography at `BACKGROUND + 0.25`, player at `PLAYER = 30.0`
- **Projectile spawn/orientation**: Unaffected

## Quality Assurance
- [x] `cargo build --bin retro-game-game` — exit 0
- [x] `cargo test --lib nebula_bouncer` — 24/24 passed, 0 failed
- [x] No files changed outside Nebula runtime + script utility
