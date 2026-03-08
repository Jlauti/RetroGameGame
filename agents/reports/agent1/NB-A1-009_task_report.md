# Task Report: NB-A1-009
**Agent**: Aarne Tasapaino
**Date**: 2026-03-07

## Summary
Completed the Nebula Enemy Combat AI + Return-Fire Contract (`NB-A1-009`). The objective was to define an enemy behavior baseline that provides structured combat pressure without turning the gameplay into an unfair or unreadable experience.

## Deliverables Created/Updated
1. **`agents/deliverables/agent1/NB-A1-009_enemy_ai_return_fire_contract.md`**: Defined three clear initial combat roles (Blockers, Flankers, Snipers) with strict movement intents. Implemented rules for aiming (telegraph requirements, cooldown windows, lead/no-lead policy), hostile projectile handling (no enemy ricochets, strict cull on terrain hit, count limits), and combat fairness rules (max 3 simultaneous attackers on screen).
2. **`specs/nebula_bouncer.md`**: Appended a summary of the new enemy combat roles and return-fire behavior constraints.
3. **`docs/architecture/DESIGN.md`**: Added architecture requirements around state machines, telemetry hooks, and the token-based concurrent attack limitation system required to implement this contract cleanly logic-wise.
4. **`agents/backlog/NB-A1-009.md`**: Marked ticket as DONE.

## Notes
- To prevent unfair deaths, omniscient tracking and immediate zero-cooldown enemy "lasers" are strictly banned. The focus is squarely on telegraphing shots and rewarding the player for evasive movement within the boundaries defined in previous contracts.
- Enemies do not share the player's ricochet ability; their projecticles are direct threats meant to be dodged or intercepted by moving out of their line of sight.
- Handoff complete. Ready for Pekka (`NB-A2-013`) to implement the runtime behavior.
