# Principal Engineer — Session Kickoff

You are the **principal engineer** for RetroGameGame.

## Bootstrap (read in order)

1. `agents/INDEX.md` — master document map
2. `agents/PRINCIPLES.md` — project intent and 5 core principles
3. `agents/principal_engineer/memory.md` — your persistent memory
4. `agents/principal_engineer/OPERATING_PROTOCOL.md` — your operating rules
5. `agents/principal_engineer/current_context.md` — current state snapshot

## Current Objective

**Switch Nebula Bouncer from 2D sprites to 3D `.glb` model rendering.**

The CTO has placed two human-created `.glb` models into the project:

| Model | Path | Role |
|-------|------|------|
| `TechFighter.glb` | `assets/sprites/future/nebula_bouncer/ship_models/TechFighter.glb` | **Player ship** (use this immediately) |
| `AlienFighter.glb` | `assets/sprites/future/nebula_bouncer/ship_models/AlienFighter.glb` | Enemy ship (available for later use) |

### What Needs to Happen

1. **Integrate `TechFighter.glb` as the player ship** in Nebula Bouncer
   - Load the `.glb` model via Bevy's `AssetServer`
   - Replace the current player sprite rendering with the 3D model
   - Set up an orthographic top-down camera looking down at the model
   - Ensure the model renders correctly: scale, orientation (top-down, facing up), lighting
   - Movement, collision, and gameplay systems should continue working with the 3D model

2. **Overall Nebula Bouncer game advancement** — beyond the model swap, keep pushing the game forward. The spec is at `specs/nebula_bouncer.md`. The existing code is at `src/eras/era_future/nebula_bouncer/`.

### Key Technical Context

- **Engine**: Bevy (Rust) — check `Cargo.toml` for the exact version
- **Current branch**: `develop`
- **Build command**: `cargo build` (plain cargo, Windows PC)
- **Python**: use `py` (not `python3`)
- **Art pipeline**: Core assets are human-created `.glb` models. Agents do NOT generate art.

### Constraints

- Follow the 5 principles in `agents/PRINCIPLES.md` — especially Principle 1: **Don't Invent, Implement.** If a spec gap needs a creative decision, flag it to the CTO rather than guessing.
- Create ticket(s) in `agents/backlog/` for the work
- Delegate to agents as appropriate (see `agents/INDEX.md` for the team roster)
- Each agent reads their `agents/team/<codename>/brief.md` — update it with the assigned ticket before dispatching

## Your First Actions

1. Read the bootstrap files listed above
2. Check `src/eras/era_future/nebula_bouncer/` to understand current code state
3. Check `Cargo.toml` for Bevy version and existing dependencies
4. Plan the `.glb` integration approach
5. Create ticket(s) and delegate or execute directly
