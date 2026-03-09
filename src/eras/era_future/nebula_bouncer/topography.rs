use crate::eras::era_future::nebula_bouncer::components::{
    BreakableHazard, BreakableRewardRole, GameLayer, HexExtrusion, PlayerSurfaceRole,
    SurfaceArchetype, SurfaceDurability, SurfaceRole, TerrainContourSample, Wall, depth,
};
use crate::eras::era_future::nebula_bouncer::procgen::{
    BREAKABLE_HEAL_AMOUNT, ChunkMember, ChunkTopography,
};
use crate::eras::era_future::nebula_bouncer::resources::NebulaMaterials;
use crate::shared::components::Health;
use avian2d::prelude::*;
use bevy::prelude::*;

/// Component for a single hex in the topography grid
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TopographyHex;

pub const HEX_RADIUS: f32 = 48.0;
pub const HEX_WIDTH: f32 = HEX_RADIUS * 1.732; // sqrt(3)
pub const HEX_HEIGHT: f32 = HEX_RADIUS * 2.0;
pub const TERRAIN_WIDTH: f32 = 1360.0;
const CANYON_HALF_WIDTH: f32 = TERRAIN_WIDTH * 0.5;
pub const SOFT_BOUNDARY_X: f32 = CANYON_HALF_WIDTH - 84.0;
pub const CORE_LANE_HALF_WIDTH: f32 = SOFT_BOUNDARY_X * 0.6;
pub const SHOULDER_WIDTH: f32 = SOFT_BOUNDARY_X - CORE_LANE_HALF_WIDTH;
const EXTRUSION_CENTER_LIMIT_X: f32 = SOFT_BOUNDARY_X - (HEX_WIDTH * 0.64);
const INNER_BANK_MIN_X: f32 = CORE_LANE_HALF_WIDTH - 52.0;
const INNER_BANK_MAX_X: f32 = CORE_LANE_HALF_WIDTH + 132.0;
const OUTER_RIDGE_MIN_X: f32 = CORE_LANE_HALF_WIDTH + 124.0;
const LATE_GATE_MIN_X: f32 = CORE_LANE_HALF_WIDTH * 0.34;
const LATE_GATE_MAX_X: f32 = CORE_LANE_HALF_WIDTH * 0.58;
const MOTIF_BAND_HEIGHT: f32 = HEX_HEIGHT * 2.25;
const BREAKABLE_HAZARD_HP: i32 = 10;
const FLOOR_SURFACE_MIN_Z: f32 = depth::BACKGROUND + 0.28;
const FLOOR_SURFACE_LIFT_SCALE: f32 = 0.16;
const BREAKABLE_MARKER_Z_OFFSET: f32 = 18.0;
const BREAKABLE_MARKER_THICKNESS: f32 = 4.0;
const BREAKABLE_MARKER_BLOCK_SIZE: f32 = 8.0;

/// Hex outline texture path (generated procedural asset).
pub const HEX_OUTLINE_TEXTURE: &str = "sprites/future/nebula_bouncer/hex_outline.png";

/// Tier colors for neon topography. Tier 0 is the lowest basin, Tier 3 the highest mound.
/// NB-A2-010 pass3: Bright saturated neon wireframe edges. Texture provides outline shape.
pub const TIER_COLORS: [Color; 4] = [
    Color::srgba(0.03, 0.28, 0.12, 0.96), // Tier 0: dark emerald
    Color::srgba(0.05, 0.42, 0.18, 0.96), // Tier 1: emerald
    Color::srgba(0.09, 0.60, 0.25, 0.96), // Tier 2: bright emerald
    Color::srgba(0.18, 0.82, 0.34, 0.96), // Tier 3: neon crest
];

/// Topography height quantization tiers
pub fn quantize_height(height: f32) -> usize {
    if height < 0.25 {
        0
    } else if height < 0.50 {
        1
    } else if height < 0.75 {
        2
    } else {
        3
    }
}

/// Generate a pseudo-random hash from a seed and a value (SplitMix64 mix).
pub fn fold_hash(seed: u64, value: u64) -> u64 {
    let mut z = seed.wrapping_add(value).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn classify_extrusion_surface(
    tier: usize,
    abs_x: f32,
    late_gate_band: bool,
    chunk_center_y: f32,
) -> SurfaceRole {
    let hard_blocker = chunk_center_y > 3200.0
        && late_gate_band
        && tier >= 2
        && abs_x >= LATE_GATE_MIN_X
        && abs_x <= LATE_GATE_MAX_X;

    if hard_blocker {
        SurfaceRole {
            player_role: PlayerSurfaceRole::HardCrashBlocker,
            ricochet: false,
            archetype: SurfaceArchetype::HardCrashExtrusion,
            ..SurfaceRole::default()
        }
    } else {
        SurfaceRole {
            player_role: PlayerSurfaceRole::SoftPressureBoundary,
            ricochet: true,
            archetype: SurfaceArchetype::RicochetExtrusion,
            ..SurfaceRole::default()
        }
    }
}

fn motif_band_index(y: f32) -> i32 {
    (y / MOTIF_BAND_HEIGHT).floor() as i32
}

fn structured_extrusion_role(
    tier: usize,
    c: i32,
    r: i32,
    x: f32,
    y: f32,
    chunk_center_y: f32,
) -> Option<SurfaceRole> {
    let abs_x = x.abs();
    if abs_x > EXTRUSION_CENTER_LIMIT_X {
        return None;
    }

    let band = motif_band_index(y);
    let motif_cycle = band.rem_euclid(6);
    let side_sign = if x < 0.0 { -1 } else { 1 };
    let lattice = (c - r).rem_euclid(3);
    let post_lattice = (c + r).rem_euclid(2);

    let preferred_side = match motif_cycle {
        1 | 4 => -1,
        2 | 5 => 1,
        _ => 0,
    };
    let pocket_side = match motif_cycle {
        4 => -1,
        5 => 1,
        _ => 0,
    };
    let late_gate_band = band.rem_euclid(8) == 3;
    let hard_gate =
        late_gate_band && post_lattice == 0 && abs_x >= LATE_GATE_MIN_X && abs_x <= LATE_GATE_MAX_X;
    if hard_gate {
        return Some(classify_extrusion_surface(
            tier.max(1),
            abs_x,
            true,
            chunk_center_y,
        ));
    }

    if tier == 0 {
        return None;
    }

    let chicane_cycle = motif_cycle == 3;
    let favored_side = preferred_side == 0 || preferred_side == side_sign || chicane_cycle;
    let inner_min_x = if chicane_cycle {
        CORE_LANE_HALF_WIDTH - 86.0
    } else if preferred_side == side_sign || preferred_side == 0 {
        INNER_BANK_MIN_X
    } else {
        CORE_LANE_HALF_WIDTH + 18.0
    };
    let inner_max_x = if preferred_side == side_sign {
        INNER_BANK_MAX_X + 18.0
    } else if chicane_cycle {
        CORE_LANE_HALF_WIDTH + 108.0
    } else {
        CORE_LANE_HALF_WIDTH + 84.0
    };

    let inner_bank = abs_x >= inner_min_x && abs_x <= inner_max_x;
    let outer_ridge = abs_x > OUTER_RIDGE_MIN_X && abs_x <= EXTRUSION_CENTER_LIMIT_X;
    let pocket_gap = pocket_side == side_sign && outer_ridge;
    let bank_lattice = if chicane_cycle {
        (band + side_sign).rem_euclid(3)
    } else {
        band.rem_euclid(3)
    };
    let ridge_lattice = if chicane_cycle {
        (band + 2).rem_euclid(3)
    } else {
        (band + 1).rem_euclid(3)
    };
    let pressure_bank = inner_bank && favored_side && lattice == bank_lattice;
    let ridge_post = outer_ridge && !pocket_gap && tier >= 2 && lattice == ridge_lattice;
    let chicane_post = chicane_cycle
        && tier >= 2
        && abs_x >= CORE_LANE_HALF_WIDTH * 0.56
        && abs_x <= CORE_LANE_HALF_WIDTH + 28.0
        && post_lattice == if side_sign < 0 { 0 } else { 1 };

    if pressure_bank || ridge_post || chicane_post {
        Some(classify_extrusion_surface(
            tier,
            abs_x,
            false,
            chunk_center_y,
        ))
    } else {
        None
    }
}

fn tier_at(topography: &ChunkTopography, cols: i32, rows: i32, c: i32, r: i32) -> Option<f32> {
    if c < 0 || c >= cols || r < 0 || r >= rows {
        return None;
    }
    let idx = (r * cols + c) as usize;
    topography
        .tiers
        .get(idx)
        .copied()
        .map(|v| (v.min(3)) as f32)
}

fn smoothed_height(topography: &ChunkTopography, cols: i32, rows: i32, c: i32, r: i32) -> f32 {
    let center = tier_at(topography, cols, rows, c, r).unwrap_or(0.0);
    let mut total = center * 2.0;
    let mut weight = 2.0;

    // Offset-coordinates hex neighbors with row parity.
    let neighbor_deltas_even: [(i32, i32); 6] =
        [(-1, 0), (1, 0), (0, -1), (-1, -1), (0, 1), (-1, 1)];
    let neighbor_deltas_odd: [(i32, i32); 6] = [(-1, 0), (1, 0), (1, -1), (0, -1), (1, 1), (0, 1)];
    let deltas = if r % 2 == 0 {
        &neighbor_deltas_even
    } else {
        &neighbor_deltas_odd
    };

    for (dc, dr) in deltas {
        if let Some(value) = tier_at(topography, cols, rows, c + dc, r + dr) {
            total += value;
            weight += 1.0;
        }
    }

    (total / weight) / 3.0
}

pub fn spawn_chunk_topography(
    commands: &mut Commands,
    _asset_server: &AssetServer,
    nebula_mats: &NebulaMaterials,
    chunk_center_y: f32,
    chunk_height: f32,
    topography: &ChunkTopography,
) {
    let cols = topography.cols.max(0);
    let rows = topography.rows.max(0);
    let hex_width = topography.hex_width.max(1.0);
    let hex_height = topography.hex_height.max(1.0);
    let start_x = -TERRAIN_WIDTH * 0.5;
    let start_y = chunk_center_y - chunk_height * 0.5;
    let _ = nebula_mats.hex_texture.clone(); // keep outline texture alive

    for r in 0..rows {
        for c in 0..cols {
            let offset_x = if r % 2 == 1 { hex_width * 0.5 } else { 0.0 };
            let x = start_x + c as f32 * hex_width + offset_x;
            let y = start_y + r as f32 * (hex_height * 0.75);
            let idx = (r * cols + c) as usize;
            let Some(&tier_u8) = topography.tiers.get(idx) else {
                continue;
            };
            let tier = (tier_u8 as usize).min(TIER_COLORS.len() - 1);
            let normalized_height = smoothed_height(topography, cols, rows, c, r);
            // Blend tier steps with local smoothing to keep coherent terrain contours.
            let elevation = (normalized_height - 0.35) * 24.0;
            let side_ratio = (x.abs() / CANYON_HALF_WIDTH).clamp(0.0, 1.0);
            let side_curve = side_ratio.powf(1.45);
            let canyon_lift = side_curve * 176.0;
            let valley_sink = (1.0 - side_curve) * 34.0;

            let material = match tier {
                0 => nebula_mats.hex_material_t0.clone(),
                1 => nebula_mats.hex_material_t1.clone(),
                2 => nebula_mats.hex_material_t2.clone(),
                3 => nebula_mats.hex_material_t3.clone(),
                _ => nebula_mats.hex_material_t0.clone(),
            };
            let cap_material = match tier {
                0 => nebula_mats.hex_cap_material_t0.clone(),
                1 => nebula_mats.hex_cap_material_t1.clone(),
                2 => nebula_mats.hex_cap_material_t2.clone(),
                3 => nebula_mats.hex_cap_material_t3.clone(),
                _ => nebula_mats.hex_cap_material_t0.clone(),
            };
            let base_outline_z =
                depth::BACKGROUND + 1.0 + (elevation * 0.22) + (canyon_lift * 0.08)
                    - (valley_sink * 0.05);
            let terrain_sample_height = (base_outline_z - depth::BACKGROUND).max(0.0);

            commands.spawn((
                ChunkMember,
                TerrainContourSample {
                    height: terrain_sample_height,
                    motif: topography
                        .surface_roles
                        .get(idx)
                        .copied()
                        .unwrap_or_default()
                        .motif,
                    placement_zone: topography
                        .surface_roles
                        .get(idx)
                        .copied()
                        .unwrap_or_default()
                        .placement_zone,
                    cadence: topography
                        .surface_roles
                        .get(idx)
                        .copied()
                        .unwrap_or_default()
                        .cadence,
                },
                Transform::from_xyz(x, y, base_outline_z),
                GlobalTransform::default(),
            ));

            let valley_tile = tier == 0;
            let rim_scale = if valley_tile { 0.94 } else { 0.96 };
            let body_scale = if valley_tile { 0.84 } else { 0.88 };
            let flat_z_scale = if valley_tile { 0.016 } else { 0.02 };
            let floor_surface_z = base_outline_z
                .max(FLOOR_SURFACE_MIN_Z + (terrain_sample_height * FLOOR_SURFACE_LIFT_SCALE));

            // Render every cell as part of the visible floor so the lane reads as anchored topography.
            commands.spawn((
                ChunkMember,
                TopographyHex,
                Mesh3d(nebula_mats.hex_mesh.clone()),
                MeshMaterial3d(cap_material.clone()),
                Transform::from_xyz(x, y, floor_surface_z).with_scale(Vec3::new(
                    hex_width * rim_scale,
                    hex_height * rim_scale,
                    flat_z_scale,
                )),
            ));

            commands.spawn((
                ChunkMember,
                TopographyHex,
                Mesh3d(nebula_mats.hex_mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(x, y, floor_surface_z + 0.5).with_scale(Vec3::new(
                    hex_width * body_scale,
                    hex_height * body_scale,
                    flat_z_scale,
                )),
            ));

            // Keep tile read clean: no flat accent overlays on non-extruded tiles.
            let accent_selector = ((motif_band_index(y) + c - r).rem_euclid(4)) as usize;
            let surface_role = topography
                .surface_roles
                .get(idx)
                .copied()
                .unwrap_or_default();

            if surface_role.player_role != PlayerSurfaceRole::TraversalSurface {
                let tier_height = match tier {
                    0 => 28.0_f32,
                    1 => 44.0_f32,
                    2 => 62.0_f32,
                    3 => 86.0_f32,
                    _ => 28.0_f32,
                };
                let z_scale = (tier_height / 50.0_f32).max(0.36_f32);
                let prism_center_z = base_outline_z + 8.0 + (25.0 * z_scale) + (side_curve * 8.0);
                let cap_z = prism_center_z + (25.0 * z_scale) + 0.5;
                let extrusion_rim_material = match surface_role.archetype {
                    SurfaceArchetype::HardCrashExtrusion => {
                        nebula_mats.hex_accent_material_amber.clone()
                    }
                    SurfaceArchetype::RicochetExtrusion => match accent_selector % 4 {
                        0 => nebula_mats.hex_accent_material_cyan.clone(),
                        1 => nebula_mats.hex_accent_material_magenta.clone(),
                        2 => nebula_mats.hex_accent_material_blue.clone(),
                        _ => nebula_mats.hex_accent_material_lime.clone(),
                    },
                    SurfaceArchetype::TerrainBoundary => {
                        nebula_mats.hex_accent_material_cyan.clone()
                    }
                };
                let collider_radius =
                    if surface_role.player_role == PlayerSurfaceRole::HardCrashBlocker {
                        (hex_width * 0.31).clamp(14.0, 44.0)
                    } else {
                        (hex_width * 0.24).clamp(12.0, 36.0)
                    };

                if surface_role.durability == SurfaceDurability::Destructible {
                    let heal_amount =
                        if surface_role.breakable_reward == BreakableRewardRole::HealthBearing {
                            BREAKABLE_HEAL_AMOUNT
                        } else {
                            0
                        };
                    let breakable_body_material =
                        if surface_role.breakable_reward == BreakableRewardRole::HealthBearing {
                            nebula_mats.hex_accent_material_lime.clone()
                        } else {
                            nebula_mats.hex_accent_material_amber.clone()
                        };
                    let breakable_cap_material =
                        if surface_role.breakable_reward == BreakableRewardRole::HealthBearing {
                            nebula_mats.hex_accent_material_amber.clone()
                        } else {
                            nebula_mats.hex_accent_material_magenta.clone()
                        };
                    commands
                        .spawn((
                            Wall,
                            HexExtrusion,
                            surface_role,
                            BreakableHazard {
                                family: surface_role.breakable_family,
                                reward: surface_role.breakable_reward,
                                heal_amount,
                            },
                            Health::new(BREAKABLE_HAZARD_HP),
                            ChunkMember,
                            Transform::from_xyz(x, y, depth::WALL),
                            GlobalTransform::default(),
                            RigidBody::Static,
                            Collider::circle(collider_radius),
                            CollisionEventsEnabled,
                            Friction::new(0.0),
                            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
                            CollisionLayers::new(
                                GameLayer::Wall,
                                [GameLayer::Projectile, GameLayer::Player],
                            ),
                        ))
                        .with_children(|parent| {
                            let marker_z = cap_z + BREAKABLE_MARKER_Z_OFFSET - depth::WALL;
                            parent.spawn((
                                ChunkMember,
                                TopographyHex,
                                Mesh3d(nebula_mats.hex_mesh.clone()),
                                MeshMaterial3d(breakable_body_material.clone()),
                                Transform::from_xyz(0.0, 0.0, prism_center_z - depth::WALL)
                                    .with_scale(Vec3::new(
                                        hex_width * rim_scale,
                                        hex_height * rim_scale,
                                        z_scale,
                                    )),
                            ));
                            parent.spawn((
                                ChunkMember,
                                TopographyHex,
                                Mesh3d(nebula_mats.hex_mesh.clone()),
                                MeshMaterial3d(breakable_cap_material.clone()),
                                Transform::from_xyz(0.0, 0.0, cap_z - depth::WALL).with_scale(
                                    Vec3::new(
                                        hex_width * rim_scale,
                                        hex_height * rim_scale,
                                        flat_z_scale,
                                    ),
                                ),
                            ));
                            parent.spawn((
                                ChunkMember,
                                TopographyHex,
                                Mesh3d(nebula_mats.hex_mesh.clone()),
                                MeshMaterial3d(breakable_body_material.clone()),
                                Transform::from_xyz(0.0, 0.0, cap_z + 0.5 - depth::WALL)
                                    .with_scale(Vec3::new(
                                        hex_width * body_scale,
                                        hex_height * body_scale,
                                        flat_z_scale,
                                    )),
                            ));

                            if surface_role.breakable_reward == BreakableRewardRole::HealthBearing {
                                for (x_offset, y_offset) in [
                                    (-10.0, 8.0),
                                    (0.0, 12.0),
                                    (10.0, 8.0),
                                    (-6.0, 0.0),
                                    (6.0, 0.0),
                                    (0.0, -8.0),
                                ] {
                                    parent.spawn((
                                        ChunkMember,
                                        Mesh3d(nebula_mats.lane_mesh.clone()),
                                        MeshMaterial3d(
                                            nebula_mats.hex_accent_material_lime.clone(),
                                        ),
                                        Transform::from_xyz(x_offset, y_offset, marker_z)
                                            .with_scale(Vec3::new(
                                                BREAKABLE_MARKER_BLOCK_SIZE,
                                                BREAKABLE_MARKER_BLOCK_SIZE,
                                                BREAKABLE_MARKER_THICKNESS,
                                            )),
                                    ));
                                }
                                parent.spawn((
                                    ChunkMember,
                                    Mesh3d(nebula_mats.orb_mesh.clone()),
                                    MeshMaterial3d(nebula_mats.hex_accent_material_amber.clone()),
                                    Transform::from_xyz(0.0, 1.0, marker_z + 3.0)
                                        .with_scale(Vec3::splat(BREAKABLE_MARKER_BLOCK_SIZE * 0.9)),
                                ));
                            } else {
                                parent.spawn((
                                    ChunkMember,
                                    Mesh3d(nebula_mats.lane_mesh.clone()),
                                    MeshMaterial3d(nebula_mats.hex_accent_material_amber.clone()),
                                    Transform::from_xyz(0.0, 0.0, marker_z).with_scale(Vec3::new(
                                        BREAKABLE_MARKER_BLOCK_SIZE * 2.6,
                                        BREAKABLE_MARKER_BLOCK_SIZE * 0.75,
                                        BREAKABLE_MARKER_THICKNESS,
                                    )),
                                ));
                                parent.spawn((
                                    ChunkMember,
                                    Mesh3d(nebula_mats.lane_mesh.clone()),
                                    MeshMaterial3d(nebula_mats.hex_accent_material_amber.clone()),
                                    Transform::from_xyz(0.0, 0.0, marker_z).with_scale(Vec3::new(
                                        BREAKABLE_MARKER_BLOCK_SIZE * 0.75,
                                        BREAKABLE_MARKER_BLOCK_SIZE * 2.6,
                                        BREAKABLE_MARKER_THICKNESS,
                                    )),
                                ));
                                parent.spawn((
                                    ChunkMember,
                                    Mesh3d(nebula_mats.orb_mesh.clone()),
                                    MeshMaterial3d(nebula_mats.hex_accent_material_magenta.clone()),
                                    Transform::from_xyz(0.0, 0.0, marker_z + 2.0)
                                        .with_scale(Vec3::splat(BREAKABLE_MARKER_BLOCK_SIZE * 0.7)),
                                ));
                            }
                        });
                } else {
                    // Pillar body
                    commands.spawn((
                        ChunkMember,
                        TopographyHex,
                        Mesh3d(nebula_mats.hex_mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(x, y, prism_center_z).with_scale(Vec3::new(
                            hex_width * rim_scale,
                            hex_height * rim_scale,
                            z_scale,
                        )),
                    ));

                    // Pillar neon cap
                    commands.spawn((
                        ChunkMember,
                        TopographyHex,
                        Mesh3d(nebula_mats.hex_mesh.clone()),
                        MeshMaterial3d(extrusion_rim_material),
                        Transform::from_xyz(x, y, cap_z).with_scale(Vec3::new(
                            hex_width * rim_scale,
                            hex_height * rim_scale,
                            flat_z_scale,
                        )),
                    ));

                    // Pillar cap dark center
                    commands.spawn((
                        ChunkMember,
                        TopographyHex,
                        Mesh3d(nebula_mats.hex_mesh.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(x, y, cap_z + 0.5).with_scale(Vec3::new(
                            hex_width * body_scale,
                            hex_height * body_scale,
                            flat_z_scale,
                        )),
                    ));
                    commands.spawn((
                        Wall,
                        HexExtrusion,
                        surface_role,
                        ChunkMember,
                        Transform::from_xyz(x, y, depth::WALL),
                        GlobalTransform::default(),
                        RigidBody::Static,
                        Collider::circle(collider_radius),
                        CollisionEventsEnabled,
                        Friction::new(0.0),
                        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
                        CollisionLayers::new(
                            GameLayer::Wall,
                            [GameLayer::Projectile, GameLayer::Player],
                        ),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn late_gate_band_produces_hard_blocker_role() {
        let role = classify_extrusion_surface(3, 160.0, true, 4000.0);
        assert_eq!(role.player_role, PlayerSurfaceRole::HardCrashBlocker);
        assert!(!role.ricochet);
        assert_eq!(role.archetype, SurfaceArchetype::HardCrashExtrusion);
    }

    #[test]
    fn flank_extrusions_default_to_ricochet_geometry() {
        let role = classify_extrusion_surface(2, CORE_LANE_HALF_WIDTH + 40.0, false, 4000.0);
        assert_eq!(role.player_role, PlayerSurfaceRole::SoftPressureBoundary);
        assert!(role.ricochet);
        assert_eq!(role.archetype, SurfaceArchetype::RicochetExtrusion);
    }

    #[test]
    fn extrusion_envelope_rejects_out_of_bounds_cells() {
        let role = structured_extrusion_role(3, 0, 0, SOFT_BOUNDARY_X + 12.0, 0.0, 2000.0);
        assert!(role.is_none());
    }

    #[test]
    fn shoulder_bands_emit_structured_ricochet_roles() {
        let role = structured_extrusion_role(
            2,
            4,
            0,
            -(CORE_LANE_HALF_WIDTH + 72.0),
            MOTIF_BAND_HEIGHT,
            1600.0,
        );
        assert!(role.is_some());
        assert_eq!(role.unwrap().archetype, SurfaceArchetype::RicochetExtrusion);
    }
}
