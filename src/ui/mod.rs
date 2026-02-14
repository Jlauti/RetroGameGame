pub mod era_select;
pub mod menu;
pub mod results;
pub mod timeline;

use bevy::prelude::*;

/// UI plugin: menus, timeline hub, era selection, results.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu::MenuPlugin,
            timeline::TimelinePlugin,
            era_select::EraSelectPlugin,
            results::ResultsPlugin,
        ));
    }
}

// ─── Shared UI constants ───────────────────────────────────────────

/// Retro-inspired color palette
pub mod colors {
    use bevy::prelude::*;

    // CGA-inspired palette for 80s era
    pub const CGA_BLACK: Color = Color::srgb(0.0, 0.0, 0.0);
    pub const CGA_BLUE: Color = Color::srgb(0.0, 0.0, 0.67);
    pub const CGA_GREEN: Color = Color::srgb(0.0, 0.67, 0.0);
    pub const CGA_CYAN: Color = Color::srgb(0.0, 0.67, 0.67);
    pub const CGA_RED: Color = Color::srgb(0.67, 0.0, 0.0);
    pub const CGA_MAGENTA: Color = Color::srgb(0.67, 0.0, 0.67);
    pub const CGA_YELLOW: Color = Color::srgb(0.67, 0.33, 0.0);
    pub const CGA_WHITE: Color = Color::srgb(0.67, 0.67, 0.67);
    pub const CGA_BRIGHT_CYAN: Color = Color::srgb(0.33, 1.0, 1.0);
    pub const CGA_BRIGHT_GREEN: Color = Color::srgb(0.33, 1.0, 0.33);
    pub const CGA_BRIGHT_YELLOW: Color = Color::srgb(1.0, 1.0, 0.33);
    pub const CGA_BRIGHT_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);

    // UI chrome colors
    pub const PANEL_BG: Color = Color::srgba(0.05, 0.05, 0.12, 0.92);
    pub const PANEL_BORDER: Color = Color::srgb(0.25, 0.25, 0.45);
    pub const BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.35);
    pub const BUTTON_HOVER: Color = Color::srgb(0.25, 0.25, 0.55);
    pub const BUTTON_PRESSED: Color = Color::srgb(0.35, 0.20, 0.55);
    pub const BUTTON_LOCKED: Color = Color::srgb(0.1, 0.1, 0.15);
    pub const TEXT_PRIMARY: Color = Color::srgb(0.9, 0.9, 0.95);
    pub const TEXT_SECONDARY: Color = Color::srgb(0.6, 0.6, 0.7);
    pub const TEXT_ACCENT: Color = Color::srgb(0.4, 0.8, 1.0);
    pub const GOLD: Color = Color::srgb(1.0, 0.84, 0.0);
}

/// Marker for UI entities that should be cleaned up on state exit.
#[derive(Component)]
pub struct StateCleanup<S: bevy::state::state::FreelyMutableState>(pub S);
