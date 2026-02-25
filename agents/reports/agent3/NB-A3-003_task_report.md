# Task Report: NB-A3-003
## Agent: Ilmari Maasto (agent3)
## Date: 2026-02-25

### Changes Made
1. Defined `ChunkTopography` struct in `procgen.rs` to encapsulate the generated deterministic tier data:
   - `cols: i32`
   - `rows: i32`
   - `hex_width: f32`
   - `hex_height: f32`
   - `tiers: Vec<u8>` (flat array containing values 0, 1, 2, or 3)

2. Added `global_seed` and `chunks_spawned` to `ProcGenState` to ensure deterministic generation across map scrolls.

3. Implemented `generate_chunk_topography` in `procgen.rs` which deterministically evaluates discrete height tiers for the given hex dimensions based on a folded hash of the global seed and sequence index.

4. Exposed `fold_hash` in `topography.rs` as a public function for use by the procgen logic.

### Tests Added & Validated
- `test_topography_determinism`: Validates that generating map tiers for the same chunk sequence index and global seed yields identical results, and different indices/seeds yield different grids.
- `test_topography_tier_bounds`: Iterates over the generated `tiers` to enforce that no value falls outside the expected bounds of `[0, 1, 2, 3]`.


---

### 🤝 Runtime Handoff for Pekka Kone (agent2)

This section provides details for integrating the procedural topography layer with the chunk renderer during gameplay.

#### Data Fields Structure
You will receive the computed deterministic grid via the new `ChunkTopography` structure during chunk instantiation.
```rust
pub struct ChunkTopography {
    pub cols: i32,
    pub rows: i32,
    pub hex_width: f32,
    pub hex_height: f32,
    /// Flat array of size (cols * rows). Contains tier levels for each hex.
    pub tiers: Vec<u8>, 
}
```

#### Expected Tier Ranges
The `tiers` array contains integers strictly between `0` and `3` inclusively. The mapping expectation per visual contract is:
* **0**: Lowest depth (Bottom Tier)
* **1**: Shallow depth 
* **2**: Standard height
* **3**: Elevated neon hex

#### Example Mapping Notes
1. **Generating the Topography at Runtime**: During `spawn_next_chunk` in `systems.rs` (or equivalent chunks logic), query the state parameters to fetch the deterministic grid instead of calling a mock height formula.
   ```rust
   let chunk_topography = generate_chunk_topography(
       selected.height, 
       state.global_seed, 
       state.chunks_spawned
   );
   ```
2. **Reading the Array**: Hex iteration is strictly row-major matching your `cols` and `rows` layout. Read directly from `chunk_topography.tiers` in the nested loop:
   ```rust
   let tier = chunk_topography.tiers[(r * chunk_topography.cols + c) as usize];
   ```
3. **Important Logic Shift**: The deterministic hash seed is now decoupled from the absolute world position `chunk_y`. The map will smoothly scroll and hex tiers will match perfectly without the risk of floating-point precision issues drifting the noise generation over long play sessions. You MUST remember to increment `state.chunks_spawned += 1;` after generating a chunk so the next chunk sequences differently.
