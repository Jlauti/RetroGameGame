use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, VideoModeSelection, WindowMode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// The global runtime settings definition.
#[derive(Resource, Debug, Serialize, Deserialize, Clone)]
pub struct GameSettings {
    pub resolution: (u32, u32),
    pub display_mode: DisplayMode,
    pub music_volume: f32,
    pub quit_behavior: QuitBehavior,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Windowed,
    Borderless,
    Fullscreen,
}

impl DisplayMode {
    pub fn to_bevy_mode(&self) -> WindowMode {
        match self {
            DisplayMode::Windowed => WindowMode::Windowed,
            DisplayMode::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            DisplayMode::Fullscreen => {
                WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum QuitBehavior {
    ToHub,
    ToDesktop,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            resolution: (1280, 720),
            display_mode: DisplayMode::Windowed,
            music_volume: 0.7,
            quit_behavior: QuitBehavior::ToHub,
        }
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = load_settings();
        app.insert_resource(settings)
            .add_systems(Update, apply_settings_on_change);
    }
}

fn settings_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    let base = std::env::var("APPDATA").map(PathBuf::from).ok();

    #[cfg(not(target_os = "windows"))]
    let base = std::env::var("HOME")
        .map(|h| {
            let mut p = PathBuf::from(h);
            p.push(".config");
            p
        })
        .ok();

    let mut path = if let Some(mut p) = base {
        p.push("RetroGameGame");
        p
    } else {
        PathBuf::from(".")
    };

    let _ = std::fs::create_dir_all(&path);
    path.push("settings.json");
    path
}

pub fn load_settings() -> GameSettings {
    let path = settings_path();
    if let Ok(content) = fs::read_to_string(&path) {
        match serde_json::from_str::<GameSettings>(&content) {
            Ok(settings) => return settings,
            Err(e) => {
                bevy::log::warn!("Failed to parse settings.json: {}. Using defaults.", e);
            }
        }
    }
    GameSettings::default()
}

pub fn save_settings(settings: &GameSettings) {
    let path = settings_path();
    match serde_json::to_string_pretty(settings) {
        Ok(content) => {
            if let Err(e) = fs::write(&path, content) {
                bevy::log::error!("Failed to save settings: {}", e);
            }
        }
        Err(e) => {
            bevy::log::error!("Failed to serialize settings: {}", e);
        }
    }
}

/// Applies window settings whenever the GameSettings resource changes.
/// Uses Bevy change detection instead of a custom event.
fn apply_settings_on_change(
    settings: Res<GameSettings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !settings.is_changed() {
        return;
    }

    if let Ok(mut window) = window_query.single_mut() {
        window
            .resolution
            .set(settings.resolution.0 as f32, settings.resolution.1 as f32);
        window.mode = settings.display_mode.to_bevy_mode();
    }

    // Save to disk whenever settings change
    save_settings(&settings);
}
