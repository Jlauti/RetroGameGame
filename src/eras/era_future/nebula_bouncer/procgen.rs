use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Dimensions and constants for procgen
pub const CHUNK_WIDTH: f32 = 800.0;
pub const PROFILE_RESOLUTION: usize = 10; // Number of "slots" to check for edge matching

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

/// Validates that two profiles match
pub fn validate_edge_match(
    top: &[bool; PROFILE_RESOLUTION],
    bottom: &[bool; PROFILE_RESOLUTION],
) -> ValidationResult {
    for i in 0..PROFILE_RESOLUTION {
        if top[i] != bottom[i] {
            return ValidationResult::Fail(format!("Profile mismatch at index {}", i));
        }
    }
    ValidationResult::Pass
}

/// Selects a chunk from the library that matches the current edge profile, prioritizing the target pacing.
pub fn select_chunk<'a>(
    library: &'a ChunkLibrary,
    current_profile: &[bool; PROFILE_RESOLUTION],
    target_pacing: ChunkPacing,
) -> Option<&'a ChunkSchema> {
    // Attempt to filter by both profile and pacing
    let mut candidates: Vec<&ChunkSchema> = library
        .chunks
        .iter()
        .filter(|c| c.top_profile == *current_profile && c.pacing == target_pacing)
        .collect();

    // Fallback: If no pacing matches, just match the profile
    if candidates.is_empty() {
        candidates = library
            .chunks
            .iter()
            .filter(|c| c.top_profile == *current_profile)
            .collect();
    }

    if candidates.is_empty() {
        return None;
    }

    let total_weight: f32 = candidates.iter().map(|c| c.weight).sum();
    let mut pick = rand::random::<f32>() * total_weight;

    for chunk in &candidates {
        if pick <= chunk.weight {
            return Some(chunk);
        }
        pick -= chunk.weight;
    }

    candidates.last().copied()
}

/// Validates anti-softlock constraints:
/// 1. No walls too close to each other forming a trap (< 60 pixels)
/// 2. No extremely sharp concave angles between walls if they are close
pub fn validate_softlock_constraints(walls: &[WallDef]) -> ValidationResult {
    for (i, w1) in walls.iter().enumerate() {
        for (j, w2) in walls.iter().enumerate() {
            if i >= j {
                continue;
            }

            let dist = w1.position.distance(w2.position);
            // Minimum safety distance for parallel or near-parallel walls
            if dist < 60.0 {
                // Check if they form a tight gap
                // Simplified: if they are very close, warn about proximity
                return ValidationResult::Fail(format!(
                    "Walls {} and {} are too close (dist: {:.2} < 60.0)",
                    i, j, dist
                ));
            }

            // Angle check for concave intersections
            // If walls are somewhat close (e.g., < 150.0), check angles
            if dist < 150.0 {
                let angle_diff = (w1.rotation - w2.rotation).abs();
                // Normalize angle diff to [0, PI]
                let angle_diff = if angle_diff > std::f32::consts::PI {
                    (2.0 * std::f32::consts::PI - angle_diff).abs()
                } else {
                    angle_diff
                };

                // Acute angles that catch balls:
                // Very sharp (< 15 degrees ~ 0.26 rad) or
                // "Sort of" sharp (< 45 degrees ~ 0.78 rad)?
                // Ball radius is 15.0 in systems.rs, orb is 5.0.
                // A 15 degree wedge is practically a softlock for bouncing unless perfectly aimed.
                if angle_diff > 0.1 && angle_diff < 0.5 {
                    // ~5.7 to ~28 degrees
                    return ValidationResult::Fail(format!(
                        "Walls {} and {} form a sharp angle ({:.2} rad) which may softlock.",
                        i, j, angle_diff
                    ));
                }
            }
        }
    }
    ValidationResult::Pass
}

#[cfg(test)]
mod tests {
    use super::*;

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
            ValidationResult::Fail(msg) => assert_eq!(msg, "Profile mismatch at index 0"),
        }
    }

    #[test]
    fn test_softlock_validation_pass() {
        let walls = vec![
            WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(10.0, 10.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(200.0, 0.0), // Far away
                size: Vec2::new(10.0, 10.0),
                rotation: 0.0,
            },
        ];
        match validate_softlock_constraints(&walls) {
            ValidationResult::Pass => assert!(true),
            ValidationResult::Fail(msg) => assert!(false, "Unexpected fail: {}", msg),
        }
    }

    #[test]
    fn test_softlock_validation_fail_proximity() {
        let walls = vec![
            WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(10.0, 10.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(10.0, 0.0), // Too close (10.0 < 60.0)
                size: Vec2::new(10.0, 10.0),
                rotation: 0.0,
            },
        ];
        match validate_softlock_constraints(&walls) {
            ValidationResult::Pass => assert!(false, "Should have failed due to proximity"),
            ValidationResult::Fail(msg) => assert!(msg.contains("too close")),
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
                position: Vec2::new(20.0, 10.0), // Close enough to check angle
                size: Vec2::new(50.0, 10.0),
                rotation: 0.3, // ~17 degrees, sharp
            },
        ];
        match validate_softlock_constraints(&walls) {
            ValidationResult::Pass => assert!(false, "Should have failed due to sharp angle"),
            ValidationResult::Fail(msg) => assert!(msg.contains("sharp angle")),
        }
    }

    #[test]
    fn test_select_chunk_with_pacing() {
        let chunk_open = ChunkSchema {
            name: "Open1".to_string(),
            pacing: ChunkPacing::Open,
            top_profile: [false; PROFILE_RESOLUTION],
            ..ChunkSchema::default()
        };
        let chunk_dense = ChunkSchema {
            name: "Dense1".to_string(),
            pacing: ChunkPacing::Dense,
            top_profile: [false; PROFILE_RESOLUTION],
            ..ChunkSchema::default()
        };

        let library = ChunkLibrary {
            chunks: vec![chunk_open, chunk_dense],
        };

        let current_profile = [false; PROFILE_RESOLUTION];

        // Should return Open when target is Open
        let selected = select_chunk(&library, &current_profile, ChunkPacing::Open).unwrap();
        assert_eq!(selected.pacing, ChunkPacing::Open);

        // Should return Dense when target is Dense
        let selected = select_chunk(&library, &current_profile, ChunkPacing::Dense).unwrap();
        assert_eq!(selected.pacing, ChunkPacing::Dense);
    }

    #[test]
    fn test_select_chunk_pacing_fallback() {
        let chunk_open = ChunkSchema {
            name: "Open1".to_string(),
            pacing: ChunkPacing::Open,
            top_profile: [false; PROFILE_RESOLUTION],
            ..ChunkSchema::default()
        };

        let library = ChunkLibrary {
            chunks: vec![chunk_open],
        };

        let current_profile = [false; PROFILE_RESOLUTION];

        // Should fallback to Open even if target is Dense
        let selected = select_chunk(&library, &current_profile, ChunkPacing::Dense).unwrap();
        assert_eq!(selected.pacing, ChunkPacing::Open);
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
