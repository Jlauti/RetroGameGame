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

### �️ Era 1: The 1980s — "The DOS Age"

**Visual Style:** EGA palette (16-color), CRT scanline effect, chunky pixels
**Frame:** DOS command prompt / early PC monitor border
**Audio Style:** PC speaker beeps, simple square wave melodies

> **Design Note:** We originally considered supporting both CGA and EGA palettes,
> but the cyan-heavy CGA aesthetic was only representative of non-EGA graphics
> adapters. Since the games we're inspired by (Digger, Captain Comic, Star Goose)
> all supported EGA, we're going with the richer EGA 16-color palette exclusively.
> This gives us a broader, more vibrant color range while still looking authentically 80s.

| Mini-Game | Inspired By | Core Mechanic |
|-----------|-------------|---------------|
| **Tunnel Miner** | Digger (1983, Windmill Software) | Dig tunnels, collect emeralds, crush enemies with gold bags, avoid Nobbins & Hobbins |
| **Cosmic Captain** | Captain Comic (1988, Michael Denio) | Side-scrolling platformer, collect items to gain abilities, explore alien world |
| **Star Goose** | Star Goose (1988, Logotron) | Vertical-scrolling shooter, collect crystals, manage shields/ammo/fuel, terrain affects bullets |

#### Tunnel Miner (Digger)
- **Grid-based movement** through underground maze
- **Dig** through earth in any cardinal direction
- **Collect emeralds** (bonus for 8 in a row)
- **Gold bags** can be pushed and dropped to crush enemies
- **Enemies**: Nobbins (slow, follow tunnels) → transform into Hobbins (fast, can dig)
- **Weapon**: Single-shot with recharge delay (longer on higher levels)
- **Cherry bonus mode**: eat enemies for limited time (Pac-Man style)
- **Progression**: levels get harder — more enemies, slower recharge

#### Cosmic Captain (Captain Comic)
- **Side-scrolling exploration** across multiple themed zones
- **Start unarmed** — find Blastola Cola to gain shooting ability
- **Collectible power-ups**: keys, high-jump boots, lantern (dark areas), wave beam
- **Shield-based health** (12 points, 2 per hit)
- **Non-linear world**: 8 areas × 3 zones each
- **Zone transitions** act as checkpoints
- **Goal**: recover three stolen artifacts

#### Star Goose
- **Vertical-scrolling shooter** over alien terrain
- **Collect 48 crystals** across 8 levels (6 per level)
- **Resource management**: shields, ammunition, fuel
- **Terrain contours** affect bullet trajectory
- **Tunnel sequences**: switch to 3D perspective to resupply
- **Difficulty ramp**: increasingly dense enemy patterns

### 🎮 Era 2: The 1990s — "The Golden Age"

**Visual Style:** 16-bit pixel art, richer palettes, parallax scrolling
**Frame:** TV with console underneath / Windows 95-style desktop
**Audio Style:** Chiptune melodies, MIDI music

| Mini-Game | Inspired By | Core Mechanic |
|-----------|-------------|---------------|
| **Worm Wars** | Worms (1995, Team17) | Turn-based artillery, destructible terrain, weapon arsenal |
| **Ice Blitz** | NHL 98 (1997, EA Sports) | Top-down ice hockey, fast-paced arcade sports |
| **Depths of Doom** | ADOM (1994, Thomas Biskup) | Turn-based roguelike, deep dungeon crawling, permadeath |

#### Worm Wars (Worms)
- **Turn-based artillery** strategy
- **Destructible 2D terrain**
- **Team of worms** with individual health
- **Weapon selection**: bazooka, grenades, shotgun, air strikes, banana bombs, etc.
- **Wind** affects projectile trajectories
- **Movement phase**: walk, jump, backflip, use rope/jetpack
- **Water = death** at the bottom of the map

#### Ice Blitz (NHL 98)
- **Top-down ice hockey** (classic EA-style overhead view)
- **Arcade-paced**: fast skating, one-timers, body checks
- **Team control**: switch between players
- **Periods & scoring**: full game structure
- **Special moves**: spin-o-rama, slap shots, goalie control
- **Simple AI opponents**

#### Depths of Doom (ADOM)
- **Turn-based roguelike** RPG
- **Procedurally generated dungeons**
- **Character stats & classes**
- **Deep inventory management**: weapons, armor, potions, scrolls
- **Permadeath**: one life, consequences matter
- **ASCII-inspired** tile visuals (fitting the era)
- **Multiple dungeon branches** with increasing difficulty

### 💿 Era 3: The 2000s — *TBD*

*Games for this era will be selected later.*

### 🕹️ Era 4: The 2010s — *TBD*

*Games for this era will be selected later.*

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
                    │ Results │
                    └─────────┘
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

### Directory Structure

```
RetroGameGame/
├── Cargo.toml
├── .cargo/
│   └── config.toml          # Fast linker config (Windows)
├── assets/
│   ├── fonts/
│   ├── sprites/
│   │   ├── era_80s/
│   │   │   ├── tunnel_miner/
│   │   │   ├── cosmic_captain/
│   │   │   └── star_goose/
│   │   └── era_90s/
│   │       ├── worm_wars/
│   │       ├── ice_blitz/
│   │       └── depths_of_doom/
│   ├── audio/
│   │   ├── sfx/
│   │   └── music/
│   └── shaders/
│       └── crt.wgsl          # CRT scanline post-process
├── src/
│   ├── main.rs               # Entry point, app builder
│   ├── lib.rs                # Re-exports, root plugin
│   ├── core/
│   │   ├── mod.rs
│   │   ├── states.rs         # GameState enum
│   │   ├── progression.rs    # Save/load, unlocks
│   │   └── input.rs          # Unified input handling
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── menu.rs           # Main menu
│   │   ├── timeline.rs       # Era timeline hub
│   │   ├── era_select.rs     # Mini-game selection within an era
│   │   └── results.rs        # Score / results screen
│   ├── effects/
│   │   ├── mod.rs
│   │   ├── crt.rs            # CRT scanline effect
│   │   └── transitions.rs    # Screen transitions
│   ├── eras/
│   │   ├── mod.rs
│   │   ├── shared.rs         # Shared mini-game components
│   │   ├── era_80s/
│   │   │   ├── mod.rs
│   │   │   ├── tunnel_miner.rs   # Digger clone
│   │   │   ├── cosmic_captain.rs # Captain Comic clone
│   │   │   └── star_goose.rs     # Star Goose clone
│   │   └── era_90s/
│   │       ├── mod.rs
│   │       ├── worm_wars.rs      # Worms clone
│   │       ├── ice_blitz.rs      # NHL 98 clone
│   │       └── depths_of_doom.rs # ADOM clone
│   └── shared/
│       ├── mod.rs
│       ├── components.rs     # Shared ECS components
│       ├── physics.rs        # Simple 2D physics
│       └── collision.rs      # Collision detection
├── specs/                     # Individual mini-game spec documents
│   ├── tunnel_miner.md
│   ├── cosmic_captain.md
│   ├── star_goose.md
│   ├── worm_wars.md
│   ├── ice_blitz.md
│   └── depths_of_doom.md
└── DESIGN.md                 # This file
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
