use bevy::prelude::*;
use retro_game_game::RetroGameGamePlugin;

fn main() {
    let asset_root = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
    println!("Starting RetroGameGame...");
    println!("Asset root: {}", asset_root);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RetroGameGame".into(),
                resolution: (1024, 768).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }).set(AssetPlugin {
            file_path: asset_root,
            ..default()
        }))
        .add_plugins(RetroGameGamePlugin)
        .run();
}
