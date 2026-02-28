use crate::eras::era_future::nebula_bouncer::components::depth;
use crate::eras::era_future::nebula_bouncer::procgen::{ChunkMember, ChunkTopography};
use crate::eras::era_future::nebula_bouncer::resources::NebulaMaterials;
use bevy::prelude::*;

/// Component for a single hex in the topography grid
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TopographyHex;

pub const HEX_RADIUS: f32 = 48.0;
pub const HEX_WIDTH: f32 = HEX_RADIUS * 1.732; // sqrt(3)
pub const HEX_HEIGHT: f32 = HEX_RADIUS * 2.0;
const CANYON_HALF_WIDTH: f32 = 960.0 * 0.5;

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

pub fn fold_hash(seed: u64, value: u64) -> u64 {
    seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).rotate_left(7) ^ value
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
    let start_x = -960.0 * 0.5;
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
            // Blend tier steps with local smoothing to reduce harsh stair-stepping.
            let elevation = (normalized_height - 0.35) * 24.0;
            // footprint > 1.0 so hexes overlap slightly → continuous terrain with no gaps.
            let footprint = 1.04;
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

            // NB-A2-010 pass6: Tier-dependent height for 3D terrain relief.
            // Each tier sits at a different Z level; Z-scale stretches the prism vertically.
            let tier_height = match tier {
                0 => 0.0,
                1 => 30.0,
                2 => 68.0,
                3 => 118.0,
                _ => 0.0,
            };
            let z_scale = 1.20 + tier as f32 * 0.32; // taller prisms for higher tiers
            let prism_center_z =
                depth::BACKGROUND + tier_height + elevation + canyon_lift - valley_sink - 42.0;
            // Extrusion depth is 50 units around center, so top face sits at +25*scale.
            let cap_z = prism_center_z + (25.0 * z_scale) + 1.2;
            commands.spawn((
                ChunkMember,
                TopographyHex,
                Mesh3d(nebula_mats.hex_mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_xyz(x, y, prism_center_z).with_scale(Vec3::new(
                    hex_width * footprint,
                    hex_height * footprint,
                    z_scale,
                )),
            ));

            // Additive cap overlay gives each top face a neon contour without changing collision.
            commands.spawn((
                ChunkMember,
                TopographyHex,
                Mesh3d(nebula_mats.quad_mesh.clone()),
                MeshMaterial3d(cap_material),
                Transform::from_xyz(x, y, cap_z).with_scale(Vec3::new(
                    hex_width * 1.01,
                    hex_height * 1.01,
                    1.0,
                )),
            ));

            // Sparse chroma accents add extra neon variation while preserving the green base read.
            let accent_selector = fold_hash(((r as u64) << 32) | c as u64, tier as u64) % 19;
            let accent_material = match accent_selector {
                0 => Some(nebula_mats.hex_accent_material_cyan.clone()),
                1 => Some(nebula_mats.hex_accent_material_magenta.clone()),
                2 => Some(nebula_mats.hex_accent_material_amber.clone()),
                _ => None,
            };
            if let Some(material) = accent_material {
                commands.spawn((
                    ChunkMember,
                    TopographyHex,
                    Mesh3d(nebula_mats.quad_mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(x, y, cap_z + 1.4).with_scale(Vec3::new(
                        hex_width * 0.88,
                        hex_height * 0.88,
                        1.0,
                    )),
                ));
            }
        }
    }
}
