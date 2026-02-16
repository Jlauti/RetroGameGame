use bevy::prelude::*;

/// Plugin for unified input handling across keyboard, gamepad, etc.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameInput>()
            .add_systems(PreUpdate, update_game_input);
    }
}

/// Unified input state — abstracts keyboard and gamepad into game actions.
#[derive(Resource, Debug, Default)]
pub struct GameInput {
    // Directional
    pub move_direction: Vec2,
    pub just_up: bool,
    pub just_down: bool,
    pub just_left: bool,
    pub just_right: bool,

    // Actions
    pub confirm: bool,  // Enter / A button
    pub cancel: bool,   // Escape / B button
    pub action_a: bool, // Space / X button
    pub action_b: bool, // Shift / Y button
    pub pause: bool,    // Escape / Start

    // Mouse (for menus and some games)
    pub mouse_position: Vec2,
    pub mouse_click: bool,
}

fn update_game_input(keyboard: Res<ButtonInput<KeyCode>>, mut input: ResMut<GameInput>) {
    // Directional movement (WASD + Arrow keys)
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    input.move_direction = if dir.length_squared() > 0.0 {
        dir.normalize()
    } else {
        Vec2::ZERO
    };

    // Just pressed directionals
    input.just_up = keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp);
    input.just_down =
        keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown);
    input.just_left =
        keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft);
    input.just_right =
        keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight);

    // Actions
    input.confirm = keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);
    input.cancel = keyboard.just_pressed(KeyCode::Escape);
    input.action_a = keyboard.just_pressed(KeyCode::Space);
    input.action_b =
        keyboard.just_pressed(KeyCode::ShiftLeft) || keyboard.just_pressed(KeyCode::ShiftRight);
    input.pause = keyboard.just_pressed(KeyCode::Escape);
}
