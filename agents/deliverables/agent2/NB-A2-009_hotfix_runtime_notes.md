# Deliverable - NB-A2-009 Runtime Hotfix Notes

## Scope

This hotfix wave addressed the critical Nebula rendering regressions that made gameplay unreadable:
- dense topography fill over the playfield
- camera/projection behavior collapsing the scene into a tiny center patch
- chase-camera clipping range not covering the actual camera distance

## Runtime Outcomes

1. Topography readability improved
- Hex visuals now render as outlines rather than solid rectangle fill.
- Low-value topography no longer floods the full chunk.
- Tier intensity was reduced to keep player/projectile readability.

2. Camera projection corrected
- Orthographic projection now uses fixed vertical viewport height for the chase-camera target scale.
- This prevents extreme world shrink caused by an oversized `scale` value with 2D-default projection behavior.

3. Camera clipping corrected
- Orthographic near/far bounds were widened so geometry at chase-camera distance remains visible.
- This resolves bottom-strip clipping and missing scene elements from the pitched camera setup.

## Core Constants / Policies

- Camera vertical viewport target: `15 * 128` world pixels.
- Orthographic `near/far`: widened to `-5000 / 5000` for the current chase-camera distance.
- Topography remains deterministic via `generate_chunk_topography(height, global_seed, sequence_index)`.

## Verification

- `cargo fmt -- --check` passed.
- `cargo check --bin retro-game-game` passed.
- `cargo test --lib nebula_bouncer` passed.

