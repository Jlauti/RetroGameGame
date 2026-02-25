# Suggestions Inbox

Append your suggestions below after each completed task. The principal engineer will review and curate.

## Format

```markdown
### Suggestion — YYYY-MM-DD
- **Type**: memory | mandate | observation
- **Summary**: <one-line summary>
- **Detail**: <what you learned and why it matters>
- **Proposed Action**: <what should be added to memory.md or team policy>
```

---

<!-- Append suggestions below this line -->

### Suggestion — 2026-02-24
- **Type**: observation
- **Summary**: Automated glTF metadata extraction is efficient via simple byte parsing.
- **Detail**: Validating `.glb` baseline data (scale, orientation) for the new pipeline can be effectively done without heavy external tools by parsing the internal JSON chunk directly in Python. Both new 3D models were confirmed to lack baked-in transforms, delegating scale/rotation responsibilities directly to the Runtime Integration agent (Pekka).
- **Proposed Action**: Add a note to memory that `TechFighter.glb` and `AlienFighter.glb` rely on Bevy's transform components for their final in-game size and orientation since they default to identity transforms.
### Suggestion — 2026-02-25
- **Type**: observation
- **Summary**: Upstream tasks blocked downstream validations.
- **Detail**: NB-A4-009 was unblocked from the queue to validate isometric baselines before NB-A2-006 actually performed the camera migration. I was able to write the deliverable outlining the expected changes required, but couldn't visibly validate them.
- [previous lines unchanged]
- **Proposed Action**: Enforce stricter queue gates, or add an explicit "Blocked" tag to immediately stop work and re-queue without throwing errors or needing human unblocking.

### Suggestion — 2026-02-25
- **Type**: mandate
- **Summary**: Use Intensity Mapping (Alpha) to separate terrain from entities in neon-heavy palettes.
- **Detail**: In "Dark Synthwave" aesthetics where both terrain and ships use the same Neon Cyan/Magenta highlights, hue separation is insufficient. Terrain topography must be capped at 50% luminosity/alpha compared to entities to maintain game-world depth and focal readability.
- [previous lines unchanged]
- **Proposed Action**: Add a "Readability Guardrails" section to `PRINCIPLES.md` or a global UI/VFX spec.

### Suggestion — 2026-02-25
- **Type**: observation
- **Summary**: Pitching a camera down compresses visual geometry, requiring drastic alpha reductions.
- **Detail**: The change from -45° to -30° camera pitch for Nebula Bouncer caused previously safe hex graphics to bunch up visually, creating an opaque "wall" effect and occluding 3D models. To compensate, 2D terrain graphics on a Z=0 plane need a ~50% drop in opacity and removal of internal fills to maintain readability.
- **Proposed Action**: Ensure any future camera tilt adjustments trigger a mandatory Z-sorting and alpha-density review pass.
