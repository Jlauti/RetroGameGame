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

/// Hex outline texture path (generated procedural asset).
pub const HEX_OUTLINE_TEXTURE: &str = "sprites/future/nebula_bouncer/hex_outline.png";

/// Tier colors — Tier 0 is skipped (renders nothing).
/// Tiers 1-3 render as subtle hex outlines with increasing intensity.
pub const TIER_COLORS: [Color; 4] = [
    Color::srgba(0.0, 0.0, 0.0, 0.0),     // Tier 0: invisible (skipped)
    Color::srgba(0.61, 0.35, 0.94, 0.10),  // Tier 1: Electric Purple (subtle)
    Color::srgba(0.0, 1.0, 1.0, 0.18),     // Tier 2: Neon Cyan
    Color::srgba(1.0, 0.0, 1.0, 0.28),     // Tier 3: Hot Magenta
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

pub fn spawn_chunk_topography(
    commands: &mut Commands,
    asset_server: &AssetServer,
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
    let hex_texture: Handle<Image> = asset_server.load(HEX_OUTLINE_TEXTURE);
    let _ = hex_texture; // ensure it stays loaded in cache

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

            // Skip Tier 0 — no visual for lowest elevation
            if tier == 0 {
                continue;
            }

            let material = match tier {
                1 => nebula_mats.hex_material_t1.clone(),
                2 => nebula_mats.hex_material_t2.clone(),
                3 => nebula_mats.hex_material_t3.clone(),
                _ => nebula_mats.hex_material_t1.clone(),
            };

            commands.spawn((
                ChunkMember,
                TopographyHex,
                Mesh3d(nebula_mats.quad_mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_xyz(x, y, depth::BACKGROUND + 0.25)
                    .with_scale(Vec3::new(hex_width * 0.88, hex_height * 0.88, 1.0)),
            ));
        }
    }
}
