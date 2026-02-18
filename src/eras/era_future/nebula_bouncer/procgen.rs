use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

/// Dimensions and constants for procgen
pub const CHUNK_WIDTH: f32 = 800.0;
pub const PROFILE_RESOLUTION: usize = 10; // Number of "slots" to check for edge matching
pub const DEFAULT_PROCGEN_SOAK_SNAPSHOT_PATH: &str =
    "specs/future/nebula_bouncer/procgen_soak_snapshot_v1.json";

pub const REJECTION_PROFILE_MISMATCH: &str = "profile_mismatch";
pub const REJECTION_SOFTLOCK_CONSTRAINT: &str = "softlock_constraint";
pub const REJECTION_INVALID_WEIGHT: &str = "invalid_weight";
pub const REJECTION_NO_CANDIDATE: &str = "no_candidate";
pub const MIN_EXIT_ANGLE_RADIANS: f32 = 0.55;
pub const EXIT_ANGLE_CHECK_DISTANCE: f32 = 150.0;
pub const CONCAVE_TRAP_RIGHT_ANGLE_TOLERANCE_RADIANS: f32 = 0.22;
pub const CONCAVE_TRAP_MAX_DISTANCE: f32 = 110.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
pub enum SpawnType {
    Enemy,
    Resource,
    Hazard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect, Default)]
pub enum ChunkPacing {
    #[default]
    Open, // Low hazard, high visibility
    Transition, // Moderate hazard, narrow paths
    Dense,      // High hazard, complex bounce geometry
}

impl ChunkPacing {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Transition => "transition",
            Self::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct WallDef {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct SpawnDef {
    pub spawn_type: SpawnType,
    pub position: Vec2,
}

/// A pre-authored chunk schema
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct ChunkSchema {
    pub name: String,
    pub height: f32,
    pub walls: Vec<WallDef>,
    pub spawns: Vec<SpawnDef>,
    pub weight: f32,
    pub pacing: ChunkPacing,
    /// Discrete profile representing wall presence at the top edge (Y = height/2)
    pub top_profile: [bool; PROFILE_RESOLUTION],
    /// Discrete profile representing wall presence at the bottom edge (Y = -height/2)
    pub bottom_profile: [bool; PROFILE_RESOLUTION],
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ChunkLibrary {
    pub chunks: Vec<ChunkSchema>,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ProcGenState {
    pub last_chunk_bottom_profile: [bool; PROFILE_RESOLUTION],
    pub next_spawn_y: f32,
    pub current_pacing: ChunkPacing,
    pub previous_pacing: ChunkPacing,
    pub chunks_in_current_pacing: usize,
}

/// Component to mark entities belonging to a specific chunk for cleanup
#[derive(Component)]
pub struct ChunkMember;

/// Result of edge matching validation
pub enum ValidationResult {
    Pass,
    Fail(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcgenSoakConfig {
    pub seed: u64,
    pub steps: usize,
    pub start_profile: [bool; PROFILE_RESOLUTION],
    pub start_current_pacing: ChunkPacing,
    pub start_previous_pacing: ChunkPacing,
    pub start_chunks_in_current_pacing: usize,
}

impl Default for ProcgenSoakConfig {
    fn default() -> Self {
        Self {
            seed: 1,
            steps: 128,
            start_profile: [false; PROFILE_RESOLUTION],
            start_current_pacing: ChunkPacing::Open,
            start_previous_pacing: ChunkPacing::Open,
            start_chunks_in_current_pacing: 0,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcgenSoakSummary {
    pub seed: u64,
    pub steps_requested: usize,
    pub steps_completed: usize,
    pub pacing_pick_counts: BTreeMap<String, usize>,
    pub chunk_pick_counts: BTreeMap<String, usize>,
    pub rejection_counts: BTreeMap<String, usize>,
    pub longest_same_pacing_streak: usize,
    pub pacing_sequence: Vec<ChunkPacing>,
    pub chunk_sequence: Vec<String>,
    pub pacing_fallback_count: usize,
}

struct DeterministicSelection<'a> {
    chunk: &'a ChunkSchema,
    used_pacing_fallback: bool,
}

fn bump_counter(map: &mut BTreeMap<String, usize>, key: &str) {
    *map.entry(key.to_string()).or_insert(0) += 1;
}

/// Mirrors runtime pacing progression policy used by spawn logic.
pub fn determine_target_pacing(
    current_pacing: ChunkPacing,
    previous_pacing: ChunkPacing,
    chunks_in_current_pacing: usize,
) -> ChunkPacing {
    match current_pacing {
        ChunkPacing::Open => {
            if chunks_in_current_pacing >= 2 {
                ChunkPacing::Transition
            } else {
                ChunkPacing::Open
            }
        }
        ChunkPacing::Transition => {
            if chunks_in_current_pacing >= 1 {
                if previous_pacing == ChunkPacing::Open {
                    ChunkPacing::Dense
                } else {
                    ChunkPacing::Open
                }
            } else {
                ChunkPacing::Transition
            }
        }
        ChunkPacing::Dense => {
            if chunks_in_current_pacing >= 1 {
                ChunkPacing::Transition
            } else {
                ChunkPacing::Dense
            }
        }
    }
}

fn deterministic_weighted_pick<'a>(
    candidates: &[&'a ChunkSchema],
    rng: &mut StdRng,
) -> Option<&'a ChunkSchema> {
    if candidates.is_empty() {
        return None;
    }

    let total_weight: f32 = candidates
        .iter()
        .map(|chunk| chunk.weight.max(0.0))
        .sum::<f32>();

    if total_weight <= f32::EPSILON {
        return candidates.last().copied();
    }

    let mut pick = rng.random::<f32>() * total_weight;
    for chunk in candidates {
        pick -= chunk.weight.max(0.0);
        if pick <= 0.0 {
            return Some(chunk);
        }
    }

    candidates.last().copied()
}

fn select_chunk_deterministic<'a>(
    library: &'a ChunkLibrary,
    current_profile: &[bool; PROFILE_RESOLUTION],
    target_pacing: ChunkPacing,
    rng: &mut StdRng,
    rejection_counts: &mut BTreeMap<String, usize>,
) -> Option<DeterministicSelection<'a>> {
    let mut profile_valid: Vec<&ChunkSchema> = Vec::new();
    let mut pacing_candidates: Vec<&ChunkSchema> = Vec::new();

    for chunk in &library.chunks {
        if let ValidationResult::Fail(_) = validate_edge_match(&chunk.top_profile, current_profile)
        {
            bump_counter(rejection_counts, REJECTION_PROFILE_MISMATCH);
            continue;
        }

        if let ValidationResult::Fail(_) = validate_softlock_constraints(&chunk.walls) {
            bump_counter(rejection_counts, REJECTION_SOFTLOCK_CONSTRAINT);
            continue;
        }

        if !chunk.weight.is_finite() || chunk.weight <= 0.0 {
            bump_counter(rejection_counts, REJECTION_INVALID_WEIGHT);
            continue;
        }

        profile_valid.push(chunk);
        if chunk.pacing == target_pacing {
            pacing_candidates.push(chunk);
        }
    }

    let (active_candidates, used_fallback) = if pacing_candidates.is_empty() {
        (&profile_valid, !profile_valid.is_empty())
    } else {
        (&pacing_candidates, false)
    };

    let picked = deterministic_weighted_pick(active_candidates, rng);
    if picked.is_none() {
        bump_counter(rejection_counts, REJECTION_NO_CANDIDATE);
    }

    picked.map(|chunk| DeterministicSelection {
        chunk,
        used_pacing_fallback: used_fallback,
    })
}

/// Runs deterministic chunk selection for offline balancing and pacing analysis.
pub fn simulate_procgen_soak(
    library: &ChunkLibrary,
    config: &ProcgenSoakConfig,
) -> ProcgenSoakSummary {
    let mut summary = ProcgenSoakSummary {
        seed: config.seed,
        steps_requested: config.steps,
        ..Default::default()
    };

    let mut rng = StdRng::seed_from_u64(config.seed);
    let mut profile = config.start_profile;
    let mut current_pacing = config.start_current_pacing;
    let mut previous_pacing = config.start_previous_pacing;
    let mut chunks_in_current_pacing = config.start_chunks_in_current_pacing;

    let mut last_pacing: Option<ChunkPacing> = None;
    let mut current_streak = 0usize;

    for _ in 0..config.steps {
        let target_pacing =
            determine_target_pacing(current_pacing, previous_pacing, chunks_in_current_pacing);

        let Some(selection) = select_chunk_deterministic(
            library,
            &profile,
            target_pacing,
            &mut rng,
            &mut summary.rejection_counts,
        ) else {
            break;
        };

        if selection.used_pacing_fallback {
            summary.pacing_fallback_count += 1;
        }

        let selected = selection.chunk;

        *summary
            .pacing_pick_counts
            .entry(selected.pacing.as_str().to_string())
            .or_insert(0) += 1;
        *summary
            .chunk_pick_counts
            .entry(selected.name.clone())
            .or_insert(0) += 1;

        summary.pacing_sequence.push(selected.pacing);
        summary.chunk_sequence.push(selected.name.clone());

        if last_pacing == Some(selected.pacing) {
            current_streak += 1;
        } else {
            current_streak = 1;
            last_pacing = Some(selected.pacing);
        }
        summary.longest_same_pacing_streak = summary.longest_same_pacing_streak.max(current_streak);

        if selected.pacing != current_pacing {
            previous_pacing = current_pacing;
            current_pacing = selected.pacing;
            chunks_in_current_pacing = 1;
        } else {
            chunks_in_current_pacing += 1;
        }

        profile = selected.bottom_profile;
        summary.steps_completed += 1;
    }

    summary
}

/// Serialize a soak summary to JSON for design review and balancing iteration.
pub fn write_procgen_soak_snapshot(
    summary: &ProcgenSoakSummary,
    output_path: impl AsRef<Path>,
) -> io::Result<()> {
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(summary)
        .map_err(|err| io::Error::other(format!("failed to serialize soak summary: {err}")))?;
    fs::write(output_path, format!("{json}\n"))
}

/// Convenience wrapper to write the default project snapshot artifact path.
pub fn write_default_procgen_soak_snapshot(summary: &ProcgenSoakSummary) -> io::Result<()> {
    write_procgen_soak_snapshot(summary, DEFAULT_PROCGEN_SOAK_SNAPSHOT_PATH)
}

fn normalize_with_bounds(raw: &[f32], min_weight: f32, max_weight: f32) -> Vec<f32> {
    if raw.is_empty() {
        return Vec::new();
    }

    let sum_raw = raw.iter().sum::<f32>();
    let mut weights = if sum_raw > f32::EPSILON {
        raw.iter().map(|w| *w / sum_raw).collect::<Vec<_>>()
    } else {
        vec![1.0 / raw.len() as f32; raw.len()]
    };

    for weight in &mut weights {
        *weight = weight.clamp(min_weight, max_weight);
    }

    const EPS: f32 = 1e-6;
    for _ in 0..64 {
        let total = weights.iter().sum::<f32>();
        let delta = 1.0 - total;
        if delta.abs() <= EPS {
            break;
        }

        if delta > 0.0 {
            let adjustable: Vec<usize> = weights
                .iter()
                .enumerate()
                .filter_map(|(idx, value)| (*value < max_weight - EPS).then_some(idx))
                .collect();
            if adjustable.is_empty() {
                break;
            }

            let share = delta / adjustable.len() as f32;
            for idx in adjustable {
                weights[idx] = (weights[idx] + share).min(max_weight);
            }
        } else {
            let adjustable: Vec<usize> = weights
                .iter()
                .enumerate()
                .filter_map(|(idx, value)| (*value > min_weight + EPS).then_some(idx))
                .collect();
            if adjustable.is_empty() {
                break;
            }

            let share = (-delta) / adjustable.len() as f32;
            for idx in adjustable {
                weights[idx] = (weights[idx] - share).max(min_weight);
            }
        }
    }

    let final_sum = weights.iter().sum::<f32>();
    if (final_sum - 1.0).abs() > 1e-4 {
        let n = weights.len();
        return vec![1.0 / n as f32; n];
    }

    weights
}

/// Proposes normalized per-chunk weights from observed selection drift.
///
/// `observed_pick_counts` should map chunk name to observed picks from soak results.
/// Returned weights always sum to 1.0 and respect `[min_weight, max_weight]` if feasible.
pub fn rebalance_chunk_weights(
    library: &ChunkLibrary,
    observed_pick_counts: &BTreeMap<String, usize>,
    min_weight: f32,
    max_weight: f32,
) -> BTreeMap<String, f32> {
    let chunk_count = library.chunks.len();
    if chunk_count == 0 {
        return BTreeMap::new();
    }

    let mut min_weight = min_weight.clamp(0.0, 1.0);
    let mut max_weight = max_weight.clamp(min_weight, 1.0);

    let n = chunk_count as f32;
    if min_weight * n > 1.0 || max_weight * n < 1.0 {
        let equal = 1.0 / n;
        min_weight = equal;
        max_weight = equal;
    }

    let baseline_weights: Vec<f32> = library
        .chunks
        .iter()
        .map(|chunk| {
            if chunk.weight.is_finite() && chunk.weight > 0.0 {
                chunk.weight
            } else {
                1.0
            }
        })
        .collect();

    let baseline_total = baseline_weights.iter().sum::<f32>().max(f32::EPSILON);
    let observed_total = observed_pick_counts.values().copied().sum::<usize>() as f32;

    // Gain below 1.0 avoids overcorrecting noisy soak runs.
    let gain = 0.75;

    let mut raw_proposed = Vec::with_capacity(chunk_count);
    for (idx, chunk) in library.chunks.iter().enumerate() {
        let expected_share = baseline_weights[idx] / baseline_total;
        let observed_share = if observed_total > 0.0 {
            observed_pick_counts.get(&chunk.name).copied().unwrap_or(0) as f32 / observed_total
        } else {
            expected_share
        };

        let factor = (1.0 + (expected_share - observed_share) * gain).clamp(0.25, 1.75);
        raw_proposed.push((expected_share * factor).max(0.0001));
    }

    let normalized = normalize_with_bounds(&raw_proposed, min_weight, max_weight);

    let mut out = BTreeMap::new();
    for (chunk, weight) in library.chunks.iter().zip(normalized) {
        out.insert(chunk.name.clone(), weight);
    }

    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkValidationReason {
    ProfileMismatch,
    ConcaveTrap,
    ExitAngleFail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkValidationError {
    pub reason: ChunkValidationReason,
    pub detail: String,
}

#[derive(Debug, Clone, Copy)]
pub struct ProcgenValidationPolicy {
    pub min_exit_angle_radians: f32,
    pub exit_angle_check_distance: f32,
    pub concave_trap_max_distance: f32,
    pub concave_right_angle_tolerance_radians: f32,
}

impl Default for ProcgenValidationPolicy {
    fn default() -> Self {
        Self {
            min_exit_angle_radians: MIN_EXIT_ANGLE_RADIANS,
            exit_angle_check_distance: EXIT_ANGLE_CHECK_DISTANCE,
            concave_trap_max_distance: CONCAVE_TRAP_MAX_DISTANCE,
            concave_right_angle_tolerance_radians: CONCAVE_TRAP_RIGHT_ANGLE_TOLERANCE_RADIANS,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ValidationCounters {
    pub profile_mismatch: u64,
    pub concave_trap: u64,
    pub exit_angle_fail: u64,
}

impl ValidationCounters {
    pub fn record(&mut self, reason: ChunkValidationReason) {
        match reason {
            ChunkValidationReason::ProfileMismatch => self.profile_mismatch += 1,
            ChunkValidationReason::ConcaveTrap => self.concave_trap += 1,
            ChunkValidationReason::ExitAngleFail => self.exit_angle_fail += 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcgenPreflightSummary {
    pub total_chunks: usize,
    pub valid_chunks: usize,
    pub invalid_chunks: usize,
    pub counters: ValidationCounters,
}

fn normalize_angle_diff(a: f32, b: f32) -> f32 {
    let raw = (a - b).abs();
    if raw > std::f32::consts::PI {
        (2.0 * std::f32::consts::PI - raw).abs()
    } else {
        raw
    }
}

pub fn validate_edge_match_detailed(
    top: &[bool; PROFILE_RESOLUTION],
    bottom: &[bool; PROFILE_RESOLUTION],
) -> Result<(), ChunkValidationError> {
    for i in 0..PROFILE_RESOLUTION {
        if top[i] != bottom[i] {
            return Err(ChunkValidationError {
                reason: ChunkValidationReason::ProfileMismatch,
                detail: format!(
                    "Profile mismatch at index {} (expected={}, got={})",
                    i, bottom[i], top[i]
                ),
            });
        }
    }
    Ok(())
}

/// Validates that two profiles match
pub fn validate_edge_match(
    top: &[bool; PROFILE_RESOLUTION],
    bottom: &[bool; PROFILE_RESOLUTION],
) -> ValidationResult {
    match validate_edge_match_detailed(top, bottom) {
        Ok(()) => ValidationResult::Pass,
        Err(err) => ValidationResult::Fail(err.detail),
    }
}

/// Selects a chunk from the library that matches the current edge profile, prioritizing the target pacing.
pub fn select_chunk<'a>(
    library: &'a ChunkLibrary,
    current_profile: &[bool; PROFILE_RESOLUTION],
    target_pacing: ChunkPacing,
) -> Option<&'a ChunkSchema> {
    let (selected, _) = select_chunk_validated(
        library,
        current_profile,
        target_pacing,
        &ProcgenValidationPolicy::default(),
    );
    selected
}

fn pick_weighted_chunk<'a>(candidates: &[&'a ChunkSchema]) -> Option<&'a ChunkSchema> {
    if candidates.is_empty() {
        return None;
    }

    let total_weight: f32 = candidates.iter().map(|c| c.weight.max(0.0)).sum();
    if total_weight <= f32::EPSILON {
        return candidates.first().copied();
    }

    let mut pick = rand::random::<f32>() * total_weight;
    for chunk in candidates {
        let weight = chunk.weight.max(0.0);
        if pick <= weight {
            return Some(chunk);
        }
        pick -= weight;
    }

    candidates.last().copied()
}

/// Validates anti-softlock constraints:
/// 1. Reject deep right-angle concave traps.
/// 2. Enforce a minimum viable exit angle for nearby wall pairs.
pub fn validate_softlock_constraints_detailed(
    walls: &[WallDef],
    policy: &ProcgenValidationPolicy,
) -> Result<(), ChunkValidationError> {
    for (i, w1) in walls.iter().enumerate() {
        for (j, w2) in walls.iter().enumerate() {
            if i >= j {
                continue;
            }

            let dist = w1.position.distance(w2.position);
            let angle_diff = normalize_angle_diff(w1.rotation, w2.rotation);

            if dist <= policy.concave_trap_max_distance {
                let right_angle_delta = (angle_diff - std::f32::consts::FRAC_PI_2).abs();
                if right_angle_delta <= policy.concave_right_angle_tolerance_radians {
                    return Err(ChunkValidationError {
                        reason: ChunkValidationReason::ConcaveTrap,
                        detail: format!(
                            "Concave trap candidate at walls {}-{} (dist={:.2}, angle={:.2} rad)",
                            i, j, dist, angle_diff
                        ),
                    });
                }
            }

            if dist <= policy.exit_angle_check_distance
                && angle_diff > 0.0
                && angle_diff < policy.min_exit_angle_radians
            {
                return Err(ChunkValidationError {
                    reason: ChunkValidationReason::ExitAngleFail,
                    detail: format!(
                        "Exit angle fail at walls {}-{} (angle={:.2} rad, min={:.2} rad, dist={:.2})",
                        i, j, angle_diff, policy.min_exit_angle_radians, dist
                    ),
                });
            }
        }
    }

    Ok(())
}

pub fn validate_chunk_schema(
    chunk: &ChunkSchema,
    required_profile: &[bool; PROFILE_RESOLUTION],
    policy: &ProcgenValidationPolicy,
) -> Result<(), ChunkValidationError> {
    validate_edge_match_detailed(&chunk.top_profile, required_profile)?;
    validate_softlock_constraints_detailed(&chunk.walls, policy)?;
    Ok(())
}

pub fn select_chunk_validated<'a>(
    library: &'a ChunkLibrary,
    current_profile: &[bool; PROFILE_RESOLUTION],
    target_pacing: ChunkPacing,
    policy: &ProcgenValidationPolicy,
) -> (Option<&'a ChunkSchema>, ValidationCounters) {
    let mut counters = ValidationCounters::default();
    let mut preferred = Vec::new();
    let mut fallback = Vec::new();

    for chunk in &library.chunks {
        match validate_chunk_schema(chunk, current_profile, policy) {
            Ok(()) => {
                if chunk.pacing == target_pacing {
                    preferred.push(chunk);
                } else {
                    fallback.push(chunk);
                }
            }
            Err(err) => counters.record(err.reason),
        }
    }

    let selected = if !preferred.is_empty() {
        pick_weighted_chunk(&preferred)
    } else {
        pick_weighted_chunk(&fallback)
    };

    (selected, counters)
}

pub fn run_preflight_validation(
    library: &ChunkLibrary,
    policy: &ProcgenValidationPolicy,
) -> ProcgenPreflightSummary {
    let mut counters = ValidationCounters::default();
    let mut valid = 0usize;
    let mut invalid = 0usize;

    for chunk in &library.chunks {
        match validate_chunk_schema(chunk, &chunk.top_profile, policy) {
            Ok(()) => valid += 1,
            Err(err) => {
                invalid += 1;
                counters.record(err.reason);
            }
        }
    }

    ProcgenPreflightSummary {
        total_chunks: library.chunks.len(),
        valid_chunks: valid,
        invalid_chunks: invalid,
        counters,
    }
}

pub fn format_preflight_summary(summary: &ProcgenPreflightSummary) -> String {
    format!(
        "Procgen Preflight Summary\n\
total_chunks: {}\n\
valid_chunks: {}\n\
invalid_chunks: {}\n\
rejections.profile_mismatch: {}\n\
rejections.concave_trap: {}\n\
rejections.exit_angle_fail: {}\n",
        summary.total_chunks,
        summary.valid_chunks,
        summary.invalid_chunks,
        summary.counters.profile_mismatch,
        summary.counters.concave_trap,
        summary.counters.exit_angle_fail,
    )
}

pub fn write_preflight_summary_artifact(
    path: &Path,
    summary: &ProcgenPreflightSummary,
) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, format_preflight_summary(summary))
}

pub fn validate_softlock_constraints(walls: &[WallDef]) -> ValidationResult {
    match validate_softlock_constraints_detailed(walls, &ProcgenValidationPolicy::default()) {
        Ok(()) => ValidationResult::Pass,
        Err(err) => ValidationResult::Fail(err.detail),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn make_chunk(name: &str, pacing: ChunkPacing, weight: f32) -> ChunkSchema {
        ChunkSchema {
            name: name.to_string(),
            pacing,
            weight,
            top_profile: [false; PROFILE_RESOLUTION],
            bottom_profile: [false; PROFILE_RESOLUTION],
            ..ChunkSchema::default()
        }
    }

    #[test]
    fn test_edge_matching_pass() {
        let top = [false; PROFILE_RESOLUTION];
        let bottom = [false; PROFILE_RESOLUTION];
        match validate_edge_match(&top, &bottom) {
            ValidationResult::Pass => assert!(true),
            ValidationResult::Fail(_) => assert!(false, "Should have passed"),
        }
    }

    #[test]
    fn test_edge_matching_fail() {
        let mut top = [false; PROFILE_RESOLUTION];
        top[0] = true;
        let bottom = [false; PROFILE_RESOLUTION];
        match validate_edge_match(&top, &bottom) {
            ValidationResult::Pass => assert!(false, "Should have failed"),
            ValidationResult::Fail(msg) => assert!(msg.contains("Profile mismatch at index 0")),
        }
    }

    #[test]
    fn test_valid_concave_geometry_passes() {
        let walls = vec![
            WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(120.0, 16.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(180.0, 0.0),
                size: Vec2::new(120.0, 16.0),
                rotation: std::f32::consts::FRAC_PI_2,
            },
        ];
        match validate_softlock_constraints(&walls) {
            ValidationResult::Pass => assert!(true),
            ValidationResult::Fail(msg) => assert!(false, "Unexpected fail: {}", msg),
        }
    }

    #[test]
    fn test_concave_trap_rejection() {
        let policy = ProcgenValidationPolicy::default();
        let walls = vec![
            WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(120.0, 20.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(60.0, 45.0),
                size: Vec2::new(120.0, 20.0),
                rotation: std::f32::consts::FRAC_PI_2,
            },
        ];
        match validate_softlock_constraints_detailed(&walls, &policy) {
            Ok(()) => assert!(false, "Should have failed due to concave trap"),
            Err(err) => assert_eq!(err.reason, ChunkValidationReason::ConcaveTrap),
        }
    }

    #[test]
    fn test_softlock_validation_fail_angle() {
        let walls = vec![
            WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(50.0, 10.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(90.0, 10.0), // >60.0 and <150.0 to exercise angle path
                size: Vec2::new(50.0, 10.0),
                rotation: 0.3, // ~17 degrees, sharp
            },
        ];
        match validate_softlock_constraints(&walls) {
            ValidationResult::Pass => assert!(false, "Should have failed due to sharp angle"),
            ValidationResult::Fail(msg) => assert!(msg.contains("Exit angle fail")),
        }
    }

    #[test]
    fn test_profile_mismatch_rejection() {
        let policy = ProcgenValidationPolicy::default();
        let chunk = ChunkSchema {
            name: "Mismatch".to_string(),
            top_profile: [false; PROFILE_RESOLUTION],
            ..ChunkSchema::default()
        };
        let mut required = [false; PROFILE_RESOLUTION];
        required[2] = true;

        match validate_chunk_schema(&chunk, &required, &policy) {
            Ok(()) => assert!(false, "Should have failed due to profile mismatch"),
            Err(err) => assert_eq!(err.reason, ChunkValidationReason::ProfileMismatch),
        }
    }

    #[test]
    fn test_select_chunk_with_pacing() {
        let chunk_open = ChunkSchema {
            name: "Open1".to_string(),
            pacing: ChunkPacing::Open,
            top_profile: [false; PROFILE_RESOLUTION],
            walls: vec![WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(100.0, 12.0),
                rotation: 0.0,
            }],
            ..ChunkSchema::default()
        };
        let chunk_dense = ChunkSchema {
            name: "Dense1".to_string(),
            pacing: ChunkPacing::Dense,
            top_profile: [false; PROFILE_RESOLUTION],
            walls: vec![WallDef {
                position: Vec2::new(220.0, 0.0),
                size: Vec2::new(100.0, 12.0),
                rotation: std::f32::consts::FRAC_PI_2,
            }],
            ..ChunkSchema::default()
        };

        let library = ChunkLibrary {
            chunks: vec![chunk_open, chunk_dense],
        };

        let current_profile = [false; PROFILE_RESOLUTION];

        let selected = select_chunk(&library, &current_profile, ChunkPacing::Open).unwrap();
        assert_eq!(selected.pacing, ChunkPacing::Open);
    }

    #[test]
    fn test_preflight_report_generation_path() {
        let policy = ProcgenValidationPolicy::default();
        let valid_chunk = ChunkSchema {
            name: "Valid".to_string(),
            walls: vec![WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(100.0, 20.0),
                rotation: 0.0,
            }],
            ..ChunkSchema::default()
        };
        let invalid_chunk = ChunkSchema {
            name: "Invalid".to_string(),
            walls: vec![
                WallDef {
                    position: Vec2::new(0.0, 0.0),
                    size: Vec2::new(100.0, 20.0),
                    rotation: 0.0,
                },
                WallDef {
                    position: Vec2::new(40.0, 35.0),
                    size: Vec2::new(100.0, 20.0),
                    rotation: std::f32::consts::FRAC_PI_2,
                },
            ],
            ..ChunkSchema::default()
        };
        let library = ChunkLibrary {
            chunks: vec![valid_chunk, invalid_chunk],
        };

        let summary = run_preflight_validation(&library, &policy);
        assert_eq!(summary.total_chunks, 2);
        assert_eq!(summary.invalid_chunks, 1);
        assert_eq!(summary.counters.concave_trap, 1);

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output_path =
            std::env::temp_dir().join(format!("nb_cx_006_preflight_summary_{}.txt", nanos));
        write_preflight_summary_artifact(&output_path, &summary).unwrap();

        let content = std::fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("Procgen Preflight Summary"));
        assert!(content.contains("rejections.concave_trap: 1"));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn test_soak_same_seed_produces_identical_sequence() {
        let library = ChunkLibrary {
            chunks: vec![
                make_chunk("open_a", ChunkPacing::Open, 1.0),
                make_chunk("open_b", ChunkPacing::Open, 1.0),
                make_chunk("open_c", ChunkPacing::Open, 1.0),
            ],
        };

        let config = ProcgenSoakConfig {
            seed: 1337,
            steps: 40,
            ..Default::default()
        };

        let run_a = simulate_procgen_soak(&library, &config);
        let run_b = simulate_procgen_soak(&library, &config);

        assert_eq!(run_a.chunk_sequence, run_b.chunk_sequence);
        assert_eq!(run_a.pacing_sequence, run_b.pacing_sequence);
        assert_eq!(run_a.rejection_counts, run_b.rejection_counts);
    }

    #[test]
    fn test_soak_different_seed_produces_divergent_sequence() {
        let library = ChunkLibrary {
            chunks: vec![
                make_chunk("open_a", ChunkPacing::Open, 1.0),
                make_chunk("open_b", ChunkPacing::Open, 1.0),
                make_chunk("open_c", ChunkPacing::Open, 1.0),
            ],
        };

        let config_a = ProcgenSoakConfig {
            seed: 7,
            steps: 48,
            ..Default::default()
        };
        let config_b = ProcgenSoakConfig {
            seed: 987654,
            steps: 48,
            ..Default::default()
        };

        let run_a = simulate_procgen_soak(&library, &config_a);
        let run_b = simulate_procgen_soak(&library, &config_b);

        assert_ne!(run_a.chunk_sequence, run_b.chunk_sequence);
    }

    #[test]
    fn test_soak_pacing_progression_constraints_hold() {
        let library = ChunkLibrary {
            chunks: vec![
                make_chunk("open", ChunkPacing::Open, 1.0),
                make_chunk("transition", ChunkPacing::Transition, 1.0),
                make_chunk("dense", ChunkPacing::Dense, 1.0),
            ],
        };

        let summary = simulate_procgen_soak(
            &library,
            &ProcgenSoakConfig {
                seed: 11,
                steps: 9,
                ..Default::default()
            },
        );

        let expected = vec![
            ChunkPacing::Open,
            ChunkPacing::Open,
            ChunkPacing::Transition,
            ChunkPacing::Dense,
            ChunkPacing::Transition,
            ChunkPacing::Open,
            ChunkPacing::Open,
            ChunkPacing::Transition,
            ChunkPacing::Dense,
        ];

        assert_eq!(summary.steps_completed, expected.len());
        assert_eq!(summary.pacing_sequence, expected);
        assert_eq!(summary.longest_same_pacing_streak, 2);
        assert_eq!(summary.pacing_fallback_count, 0);
    }

    #[test]
    fn test_rebalance_output_is_normalized_and_bounded() {
        let library = ChunkLibrary {
            chunks: vec![
                make_chunk("open_a", ChunkPacing::Open, 0.6),
                make_chunk("open_b", ChunkPacing::Open, 0.3),
                make_chunk("open_c", ChunkPacing::Open, 0.1),
            ],
        };

        let mut observed = BTreeMap::new();
        observed.insert("open_a".to_string(), 10usize);
        observed.insert("open_b".to_string(), 50usize);
        observed.insert("open_c".to_string(), 40usize);

        let rebalance = rebalance_chunk_weights(&library, &observed, 0.1, 0.7);

        let sum = rebalance.values().copied().sum::<f32>();
        assert!((sum - 1.0).abs() < 1e-4, "sum={sum}");
        for value in rebalance.values() {
            assert!((*value >= 0.1 - 1e-6) && (*value <= 0.7 + 1e-6));
        }
        assert!(rebalance["open_a"] > rebalance["open_b"]);
    }
}

impl Default for ChunkSchema {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            height: 800.0,
            walls: Vec::new(),
            spawns: Vec::new(),
            weight: 1.0,
            pacing: ChunkPacing::Open,
            top_profile: [false; PROFILE_RESOLUTION],
            bottom_profile: [false; PROFILE_RESOLUTION],
        }
    }
}
