use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Dimensions and constants for procgen
pub const CHUNK_WIDTH: f32 = 800.0;
pub const PROFILE_RESOLUTION: usize = 10; // Number of "slots" to check for edge matching
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect, Default)]
pub enum ChunkPacing {
    #[default]
    Open, // Low hazard, high visibility
    Transition, // Moderate hazard, narrow paths
    Dense,      // High hazard, complex bounce geometry
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
