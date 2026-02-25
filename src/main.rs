use bevy::prelude::*;
use retro_game_game::RetroGameGamePlugin;
use retro_game_game::core;

fn configure_wsl_winit_backend() {
    let is_wsl = cfg!(target_os = "linux") && std::env::var_os("WSL_DISTRO_NAME").is_some();
    let backend = std::env::var("WINIT_UNIX_BACKEND")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase());

    let force_x11 = matches!(backend.as_deref(), None | Some(""));
    let backend_is_x11 = matches!(backend.as_deref(), Some("x11"));

    if is_wsl && (force_x11 || backend_is_x11) {
        // On some WSL sessions, Wayland socket I/O can drop and close the app.
        // Prefer X11 unless the user already chose a backend explicitly.
        unsafe {
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
            std::env::set_var("WAYLAND_DISPLAY", "");
        }
    }
}

fn main() {
    configure_wsl_winit_backend();

    let asset_root = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
    println!("Starting RetroGameGame...");
    println!("Asset root: {}", asset_root);

    let initial_settings = core::settings::load_settings();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RetroGameGame".into(),
                        resolution: initial_settings.resolution.into(),
                        mode: initial_settings.display_mode.to_bevy_mode(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: asset_root,
                    ..default()
                }),
        )
        .add_plugins(RetroGameGamePlugin)
        .run();
}
