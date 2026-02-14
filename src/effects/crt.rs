use bevy::prelude::*;

/// CRT scanline effect plugin.
/// Currently a placeholder — will use a WGSL post-processing shader.
pub struct CrtPlugin;

impl Plugin for CrtPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrtSettings>();
    }
}

/// Settings for the CRT effect.
#[derive(Resource, Debug)]
pub struct CrtSettings {
    pub enabled: bool,
    pub scanline_intensity: f32,
    pub curvature: f32,
    pub vignette: f32,
    pub brightness: f32,
}

impl Default for CrtSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            scanline_intensity: 0.3,
            curvature: 0.02,
            vignette: 0.2,
            brightness: 1.1,
        }
    }
}
