# Tunnel Miner (Inspired by Digger, 1983)

## Aesthetic Goal
- **Era**: 1980s (DOS Age)
- **Palette**: EGA 16-color
- **Visuals**: CRT scanlines, chunky pixels, DOS-style frame
- **Audio**: PC speaker beeps, simple square wave melodies

## Modernized Mechanics
While the look is 1983, the feel can benefit from modern UX:
- **Smooth Input**: Buffering for turns so the player doesn't feel the "grid" as harshly.
- **Improved AI**: Nobbin/Hobbin behavior can be more sophisticated than just simple pathing.
- **Particle Effects**: Modern pixel-art particle systems for digging and explosions, while keeping the colors strictly EGA.

## Core Loop
- **Grid-based movement** through underground maze.
- **Digging**: Remove "earth" tiles as the player moves.
- **Collect Emeralds**: Primary score source. Bonus for collecting 8 in a row.
- **Gold Bags**: Physics-affected. Can be pushed or dropped to crush enemies.
- **Enemies**: Nobbins (slow, follow tunnels) → transform into Hobbins (fast, can dig).
- **Weapon**: Single-shot with recharge delay.
- **Cherry Mode**: Power-up collected to allow eating enemies for a limited time.

## Progression
- Increasing enemy count and speed.
- Decreasing weapon recharge speed.
- More complex maze layouts.
