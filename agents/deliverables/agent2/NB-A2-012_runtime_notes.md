# NB-A2-012 Runtime Notes

Date: 2026-03-07
Owner: agent2 / Pekka Kone

## Scope Implemented

- constrained Nebula hex extrusions to the authored combat envelope so they no longer read outside the intended playable bounds
- replaced the previous road-like floor striping read with shoulder-side ricochet banks, pocket braces, and chicane braces
- shifted boundary presentation from a slab read to a thinner neon cage read using rails, posts, and diagonal wire braces
- preserved the existing terrain-follow, soft-boundary, and ricochet runtime contract

## Engineering Gates

- `cargo check`
- `cargo build --bin retro-game-game`
- `cargo test --lib nebula_bouncer`
- `cargo fmt -- --check`

All four commands passed on 2026-03-07.

## BRP-Assisted Runtime Validation

Boot configuration:

- `RETRO_DEV_BOOT=nebula`
- `BEVY_BRP_ENABLE=1`
- `BEVY_BRP_PORT=15702`

Validation calls used:

- `world.query`
- `brp_extras/screenshot`
- `brp_extras/send_keys`

Artifacts:

- `agents/deliverables/agent2/NB-A2-012_ground_patterns.png`
- `agents/deliverables/agent2/NB-A2-012_boundary_wires.png`
- `agents/deliverables/agent2/NB-A2-012_brp_validation.json`
- `agents/deliverables/agent2/NB-A2-012_brp_run.out.log`
- `agents/deliverables/agent2/NB-A2-012_brp_run.err.log`

Captured runtime facts from `NB-A2-012_brp_validation.json`:

- player initial translation: `[0.0, -200.10214233398438, 38.0]`
- player boundary translation after right-input probe: `[421.8750305175781, -200.10214233398438, 38.0]`
- extrusion limit x: `542.79296`
- live extrusion count: `215`
- max live extrusion abs x: `525.4720458984375`
- bounds violation count: `0`

Observed read from the captured images:

- center play space now reads as open traversal space instead of a road/runway
- readable structure comes from side-biased hex banks, pocket braces, and ricochet-ready extrusion shoulders
- boundary wall read comes from rails/posts/diagonals with only a faint glow spine behind them

## Notes

- the live capture window contained `RicochetExtrusion` samples only; this ticket's acceptance centered on bounds, boundary read, and structured ground patterning, which were validated live
