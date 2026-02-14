use bevy::prelude::*;
use retro_game_game::RetroGameGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RetroGameGame".into(),
                resolution: (1024, 768).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RetroGameGamePlugin)
        .run();
}
