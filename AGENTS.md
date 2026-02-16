# Agent Execution Guardrails

This repository uses `cargo-safe` as the default Cargo entrypoint for all compile/test/build operations to avoid abrupt process termination under memory pressure.

## Default Rule

- Always prefer `cargo-safe` over plain `cargo` for build/test/check/run/lint/doc flows.
- Use plain `cargo` only when you intentionally want to bypass memory guardrails.

## Standard Commands

- Build: `cargo-safe build`
- Build one binary: `cargo-safe build --bin retro-game-game`
- Run: `cargo-safe run --bin retro-game-game`
- Test: `cargo-safe test`
- Check: `cargo-safe check`
- Clippy: `cargo-safe clippy --all-targets --all-features`
- Clean: `cargo-safe clean`
- Doc: `cargo-safe doc --no-deps`
- Bench: `cargo-safe bench`
- Format gate: `cargo-safe fmt -- --check`

## Per-Run Overrides

- More memory: `MEM_HIGH=11G MEM_MAX=12G cargo-safe build`
- Fewer/more jobs: `CARGO_BUILD_JOBS=6 cargo-safe test`
- Both: `MEM_HIGH=11G MEM_MAX=12G CARGO_BUILD_JOBS=6 cargo-safe build`

## Notes

- Current baseline remains `jobs=8`, `codegen-units=8`, memory guardrails around 9-10G.
- If `cargo-safe` is missing, treat as environment misconfiguration and escalate.
