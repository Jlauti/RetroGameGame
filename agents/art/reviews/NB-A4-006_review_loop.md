# NB-A4-006 Sprite Review Loop

Use this loop while Aino is producing Nebula gameplay sprites.

## 0. Python Dependencies (one-time per environment)

```bash
python3 -m pip install -r /home/jl/git/RetroGameGame/assets/scripts/requirements.txt
```

## 1. Generate Metadata + Gallery

```bash
python3 /home/jl/git/RetroGameGame/assets/scripts/sprite_inspector.py init \
  --root /home/jl/git/RetroGameGame/assets/sprites/future/nebula_bouncer \
  --metadata /home/jl/git/RetroGameGame/agents/art/reviews/NB-A4-006_sprite_metadata.json

python3 /home/jl/git/RetroGameGame/assets/scripts/sprite_inspector.py render \
  --metadata /home/jl/git/RetroGameGame/agents/art/reviews/NB-A4-006_sprite_metadata.json \
  --output /home/jl/git/RetroGameGame/agents/art/reviews/NB-A4-006_sprite_inspector.html \
  --title "NB-A4-006 Sprite Review"
```

## 2. Validate Background Quality

```bash
python3 /home/jl/git/RetroGameGame/assets/scripts/check_bg.py \
  /home/jl/git/RetroGameGame/assets/sprites/future/nebula_bouncer \
  --strict
```

## 3. Human Review Rules

- Approve only strict top-down sprites with clear silhouette.
- Reject diagonal/isometric perspective.
- Reject white fringe, opaque corners, or chroma-key holes.
- Record `orientation_offset_deg` when sprite forward axis differs from north/up.
- Require metadata entries for canonical runtime files:
  - `sprite_player_ship.png`
  - `sprite_enemy_scout.png`
  - `sprite_enemy_heavy.png`
  - `sprite_enemy_interceptor.png`
  - `sprite_wall_tile.png`
  - `sprite_ground_tile.png`
  - `sprite_player_orb.png`

## 4. Runtime Rotation Hand-off

When review is complete, update:

- `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/sprite_orientation.json`

using approved `orientation_offset_deg` values so runtime facing is correct without art edits.
