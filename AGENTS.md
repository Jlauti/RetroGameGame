# Agent Execution Guardrails

This repository uses an AI agent team for development. See `agents/INDEX.md` for the full control plane map.

## Build Rules

- **Cargo**: Use plain `cargo` for all build/test/check operations
- **Python**: Use `py` (not `python3`)
- **Platform**: Windows (`c:\Users\jlaut\git\RetroGameGame`)

## Standard Commands

- Build: `cargo build`
- Build one binary: `cargo build --bin retro-game-game`
- Run: `cargo run --bin retro-game-game`
- Test: `cargo test`
- Check: `cargo check`
- Clippy: `cargo clippy --all-targets --all-features`
- Format gate: `cargo fmt -- --check`

## Branching Mandate

- Core branches are `develop` and `main`.
- Ticket branches must start from `develop` and use the `codex/` prefix.
- Ticket merges go to `develop`; only promoted, fully gated releases go to `main`.
- Principal engineer is the sole merger to `main`.

## Art Pipeline

- Core 2D and 3D assets are **human-created** `.glb` models
- Assets placed in `assets/models/<era>/<game>/`
- Agent team handles integration, validation, and consistency — not creation
