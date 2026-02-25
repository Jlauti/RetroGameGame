# RetroGameGame — Design Document

## Vision

**RetroGameGame** is a personal love letter to the history of video games. The player
journeys through the decades of gaming — starting in the DOS era of the 1980s and
progressing through the golden age of the 1990s and beyond — experiencing mini-games
inspired by the real classics that shaped a generation of gamers.

As the player progresses through time, *everything evolves*: the visual style, the
complexity of gameplay, the UI chrome, and even the "hardware" framing the screen
(arcade cabinet → home console → PC monitor → laptop/tablet).

---

## Design Philosophy: "Modern Mechanics, Vintage Aesthetics"

The goal of **RetroGameGame** is to *feel* the way we remember these games, rather than precisely how they were. We preserve the original aesthetics (EGA palettes, scanlines, limited resolutions) while utilizing modern engine capabilities to improve the playing experience.

- **VFX & Juice**: Use modern particle systems and lighting, but restrict them to era-appropriate colors.
- **Input & UX**: Implement modern conveniences like input buffering, smooth transitions, and intuitive menus.
- **AI & Complexity**: Game logic/AI can be as sophisticated as modern standards, provided it doesn't break the visual illusion.

> [!NOTE]
> Each mini-game is treated as its own project with a dedicated guidance document in the `specs/` folder. This helps maintain focus and prevents context bloat when working on specific game logic.

---

## Core Loop

```
┌──────────────┐
│   TIMELINE   │  ← Hub screen: a horizontal timeline of gaming eras
│   HUB        │     Player selects an era to enter
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  ERA SELECT  │  ← Inside an era: shows 3-5 mini-games as "arcade cabinets"
│  (Arcade)    │     or "cartridges" you can walk up to and play
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  MINI-GAME   │  ← The actual playable game, styled to match the era
│  (Play!)     │     Has its own scoring, lives, and win/lose conditions
└──────┬───────┘
       │
       ▼
┌──────────────┐
│  RESULTS     │  ← Score screen, unlocks next mini-game or era
│  & PROGRESS  │     Earns "tokens" used in the hub
└──────────────┘
```

## Progression System

- Each era contains **3-5 mini-games**
- Completing a mini-game (reaching a score threshold) **unlocks the next one**
- Completing all mini-games in an era **unlocks the next era**
- Each mini-game awards **Tokens** based on performance
- Tokens are cosmetic / fun — no hard gating beyond completion
- High scores are saved per mini-game

---

## Era Breakdown

### 🕹️ Era 1: The 1980s — "The DOS Age"

**Visual Style:** EGA palette (16-color), CRT scanline effect, chunky pixels
**Frame:** DOS command prompt / early PC monitor border
**Audio Style:** PC speaker beeps, simple square wave melodies

| Mini-Game | Spec | Inspiration |
|-----------|------|-------------|
| **Tunnel Miner** | [Spec](file:///c:/Users/jlaut/git/RetroGameGame/specs/tunnel_miner.md) | Digger (1983) |
| **Cosmic Captain** | [Spec](file:///c:/Users/jlaut/git/RetroGameGame/specs/cosmic_captain.md) | Captain Comic (1988) |
| **Star Goose** | [Spec](file:///c:/Users/jlaut/git/RetroGameGame/specs/star_goose.md) | Star Goose (1988) |

---

### 🎮 Era 2: The 1990s — "The Golden Age"

**Visual Style:** 16-bit pixel art, richer palettes, parallax scrolling
**Frame:** TV with console underneath / Windows 95-style desktop
**Audio Style:** Chiptune melodies, MIDI music

| Mini-Game | Spec | Inspiration |
|-----------|------|-------------|
| **Worm Wars** | [Spec](file:///c:/Users/jlaut/git/RetroGameGame/specs/worm_wars.md) | Worms (1995) |
| **Ice Blitz** | [Spec](file:///c:/Users/jlaut/git/RetroGameGame/specs/ice_blitz.md) | NHL 98 (1997) |
| **Depths of Doom** | [Spec](file:///c:/Users/jlaut/git/RetroGameGame/specs/depths_of_doom.md) | ADOM (1994) |

### 💿 Era 3: The 2000s — *TBD*

*Games for this era will be selected later.*

### 🕹️ Era 4: The 2010s — "The Indie Renaissance"

**Visual Style:** HD vector graphics, neon glow, particle overload (Juice).
**Frame:** Borderless window / Steam Deck style overlay.
**Audio Style:** Synthwave, heavy bass, dynamic mixing.

| Mini-Game | Spec | Inspiration |
|-----------|------|-------------|
| **Nebula Bouncer** | [Spec](file:///c:/Users/jlaut/git/RetroGameGame/specs/nebula_bouncer.md) | Geometry Wars x Breakout x Roguelites |

---

## Technical Architecture (Bevy ECS)

### State Machine

```
                    ┌─────────┐
                    │  Boot   │
                    └────┬────┘
                         │
                    ┌────▼────┐
               ┌────│  Menu   │────┐
               │    └────┬────┘    │
               │         │         │
          ┌────▼───┐ ┌───▼────┐ ┌──▼──────┐
          │Settings│ │Timeline│ │Credits  │
          └────────┘ └───┬────┘ └─────────┘
                         │
                    ┌────▼────┐
                    │EraSelect│
                    └────┬────┘
                         │
                    ┌────▼────┐
                    │ Playing │  ← Active mini-game
                    └────┬────┘
                         │
                    ┌────▼────┐
             ┌──────┤ Results │
             │      └─────────┘
             ▼
      ┌──────────┐
      │ Settings │ ← Accessible via Esc from Playing
      └──────────┘
```

### Plugin Architecture

```
RetroGameGamePlugin (root)
├── CorePlugin
│   ├── GameState management
│   ├── Progression / save system
│   ├── Audio manager
│   └── Input abstraction
├── UiPlugin
│   ├── Main menu
│   ├── Timeline hub
│   ├── Era select screen
│   ├── Settings screen        # Global config UI
│   ├── Results / score screen
│   └── Shared UI components (buttons, transitions)
├── EffectsPlugin
│   ├── CRT shader / scanlines
│   ├── Screen transitions (fade, static, etc.)
│   └── Era-appropriate post-processing
├── Era80sPlugin
│   ├── TunnelMinerPlugin      (Digger)
│   ├── CosmicCaptainPlugin    (Captain Comic)
│   └── StarGoosePlugin        (Star Goose)
├── Era90sPlugin
│   ├── WormWarsPlugin         (Worms)
│   ├── IceBlitzPlugin         (NHL 98)
│   └── DepthsOfDoomPlugin     (ADOM)
├── Era2000sPlugin             (TBD)
└── Era2010sPlugin             (TBD)
```

### Shared Components (ECS)

```rust
// Core components shared across all mini-games
Position { x: f32, y: f32 }
Velocity { x: f32, y: f32 }
Health { current: i32, max: i32 }
Score { value: u64 }
Player                              // Marker component
Collider { shape: ColliderShape }   // Simple collision

// Progression
EraId(u8)                          // 1=80s, 2=90s, 3=2000s, 4=2010s
MiniGameId { era: u8, game: u8 }   // Identifies a specific mini-game
Unlocked(bool)                     // Whether a game/era is unlocked
HighScore { game: MiniGameId, score: u64 }
```

### Asset Pipeline

- **Topography Data Flow**: Procgen systems generate 0.0 - 1.0 height-map data on the hex grid. This data is consumed by the rendering system which quantizes it into 4 discrete visual tiers (see [NB-A1-006](file:///c:/Users/jlaut/git/RetroGameGame/agents/deliverables/agent1/NB-A1-006_camera_topography_contract.md)) while maintaining a consistent `Y=0` gameplay plane for physics.
- **Asset Pipeline**: Models created as `.glb` (glTF binary), placed in `assets/models/era_2010s/nebula_bouncer/`. Materials embedded. Y-up, facing +Z. 1 unit = 1 game tile.

The AI agent team handles integration, validation, and consistency — **not** asset creation.

### Directory Structure

```
RetroGameGame/
├── Cargo.toml
├── AGENTS.md                  # Agent build rules
├── .cargo/
│   └── config.toml            # Fast linker config (Windows)
├── agents/                    # AI agent control plane
│   ├── INDEX.md               # Master document map (START HERE)
│   └── ...                    # See INDEX.md for full structure
├── assets/
│   ├── fonts/
│   ├── models/                # .glb 3D models (human-created)
│   │   ├── era_80s/
│   │   │   ├── tunnel_miner/
│   │   │   ├── cosmic_captain/
│   │   │   └── star_goose/
│   │   ├── era_90s/
│   │   │   ├── worm_wars/
│   │   │   ├── ice_blitz/
│   │   │   └── depths_of_doom/
│   │   └── era_2010s/
│   │       └── nebula_bouncer/
│   ├── sprites/               # 2D sprites (era-appropriate games)
│   │   ├── era_80s/
│   │   └── era_90s/
│   ├── audio/
│   │   ├── sfx/
│   │   └── music/
│   └── shaders/
│       └── crt.wgsl           # CRT scanline post-process
├── src/
│   ├── main.rs                # Entry point, app builder
│   ├── lib.rs                 # Re-exports, root plugin
│   ├── core/
│   │   ├── mod.rs
│   │   ├── states.rs          # GameState enum
│   │   ├── progression.rs     # Save/load, unlocks
│   │   └── input.rs           # Unified input handling
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── menu.rs            # Main menu
│   │   ├── timeline.rs        # Era timeline hub
│   │   ├── era_select.rs      # Mini-game selection within an era
│   │   └── results.rs         # Score / results screen
│   ├── effects/
│   │   ├── mod.rs
│   │   ├── crt.rs             # CRT scanline effect
│   │   └── transitions.rs     # Screen transitions
│   ├── eras/
│   │   ├── mod.rs
│   │   ├── shared.rs          # Shared mini-game components
│   │   ├── era_80s/
│   │   │   ├── mod.rs
│   │   │   ├── tunnel_miner.rs
│   │   │   ├── cosmic_captain.rs
│   │   │   └── star_goose.rs
│   │   └── era_90s/
│   │       ├── mod.rs
│   │       ├── worm_wars.rs
│   │       ├── ice_blitz.rs
│   │       └── depths_of_doom.rs
│   └── shared/
│       ├── mod.rs
│       ├── components.rs      # Shared ECS components
│       ├── physics.rs         # Simple 2D physics
│       └── collision.rs       # Collision detection
├── specs/                      # Individual mini-game spec documents
│   ├── tunnel_miner.md
│   ├── cosmic_captain.md
│   ├── star_goose.md
│   ├── worm_wars.md
│   ├── ice_blitz.md
│   ├── depths_of_doom.md
│   └── nebula_bouncer.md
└── docs/
    └── architecture/
        └── DESIGN.md          # This file
```

---

## Development Roadmap

### Phase 1: Foundation
- [x] Project concept & design document
- [ ] Cargo project setup with Bevy
- [ ] Game state machine
- [ ] Main menu (basic)
- [ ] Timeline hub (placeholder)

### Phase 2: First Playable — Tunnel Miner (80s)
- [ ] Tunnel Miner spec document
- [ ] Grid-based movement & digging
- [ ] Emerald collection & gold bag physics
- [ ] Enemy AI (Nobbins, Hobbins)
- [ ] Scoring, lives, cherry bonus mode
- [ ] CRT visual effect & DOS-style frame

### Phase 3: Complete the 80s
- [ ] Cosmic Captain spec & implementation (platformer)
- [ ] Star Goose spec & implementation (vertical shooter)
- [ ] Era completion screen & 90s unlock

### Phase 4: The 90s
- [ ] Worm Wars spec & implementation (artillery strategy)
- [ ] Ice Blitz spec & implementation (hockey)
- [ ] Depths of Doom spec & implementation (roguelike)

### Phase 5+: Later Eras & Polish
- [ ] 2000s era game selection & implementation
- [ ] 2010s era game selection & implementation
- [ ] Audio & music per era
- [ ] Save system & progression polish
- [ ] Visual effects per era (evolving shaders)

---

## Technical Notes

### Why Bevy?
- **Code-only architecture** — no opaque editor, everything is text files
- **ECS pattern** — each mini-game is a clean plugin with its own systems
- **AI-friendly** — game state is structured data, fully inspectable
- **Rust** — performance, safety, modern tooling

### Development Approach
- Each mini-game gets its own **spec document** before implementation
- Mini-games are developed as **independent Bevy plugins**
- Shared systems (physics, collision, input) are extracted as they emerge
- Visual effects are layered on top — games work without them first
