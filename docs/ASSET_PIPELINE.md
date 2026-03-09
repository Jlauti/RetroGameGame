# Asset And Direction Pipeline

## Current Pipeline

Nebula Bouncer uses a human-led asset pipeline.

The active flow is:

1. chapter direction is defined
2. 2D concept sheets are created to clarify silhouettes, faction language, and ground identity
3. human-created 3D `.glb` assets are produced from those concepts
4. runtime integration and validation happen in the repo

## Role Split

- `Aino Kuvitus`: concept sheets, visual direction, chapter music briefs, consistency review
- human creator: final 3D asset creation
- `Pekka Kone`: runtime integration
- `Ilmari Maasto`: chapter/faction/ground planning inputs

## Repository Rule

- Treat old Nebula sprite-generation documents and AI image workflows as historical material.
- Do not use them as the active pipeline for Nebula.
- Final 3D gameplay assets belong under `assets/models/`

## Current Asset Expectations

- concept artifacts should clarify shape language, faction identity, and chapter mood
- runtime assets should be `.glb` and human-created
- integration work should focus on loading, orientation, scale, and gameplay readability

## Active Nebula Environment Direction

- `NB-A1-010` explicitly allows Nebula's ground/environment pass to use procedural neon geometry, materials, and environment VFX as the active battlefield language.
- This allowance covers abstract/systemic terrain contours, ridge/bank geometry, side cage presentation, and breakable environment fixtures.
- Ships and enemies remain human-created `.glb` assets; the procedural allowance does not replace the human-made gameplay actor pipeline.
- Aino's concept work for this lane should clarify motif readability, emissive hierarchy, and motion language for procedural terrain roles rather than turning the current ground pass into a modeled-environment-first workflow.
