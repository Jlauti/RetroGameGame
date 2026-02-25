### Suggestions from NB-QA-020

- **Strict Data Layer Enforcement**: Future QA waves should include a "Wiring Check" to ensure that data layers implemented in A3-style tickets are actually consumed by the A2-style runtime integration.
- **Coordinate System Clarity**: Recommend adding a section to `DESIGN.md` explicitly defining "Forward", "Up", and "Right" in world space for all era-specific cameras to avoid axis confusion in contracts.
- **Automated Fmt Gate**: Ensure `cargo fmt -- --check` is part of the local CI/pre-commit check for all agents to avoid style-only signoff failures.
