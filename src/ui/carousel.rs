use crate::core::states::GameState;
use bevy::prelude::*;

pub struct CarouselPlugin;

impl Plugin for CarouselPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CarouselState>()
            .add_systems(OnEnter(GameState::Menu), setup_carousel)
            .add_systems(
                Update,
                (carousel_input, animate_carousel).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_carousel);
    }
}

#[derive(Resource, Default)]
pub struct CarouselState {
    pub current_index: usize,
    pub target_index: usize,
    pub items: Vec<CarouselItemData>,
}

pub struct CarouselItemData {
    pub title: String,
    pub image_path: String,
    pub era: String,
}

#[derive(Component)]
pub struct CarouselItem {
    pub index: usize,
}

#[derive(Component)]
pub struct CarouselRoot;

fn setup_carousel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<CarouselState>,
) {
    // Define items
    state.items = vec![
        // 80s
        CarouselItemData {
            title: "Tunnel Miner".into(),
            image_path: "ui/thumbnails/tunnel_miner.png".into(),
            era: "1980s DOSE".into(),
        },
        CarouselItemData {
            title: "Cosmic Captain".into(),
            image_path: "ui/thumbnails/cosmic_captain.png".into(),
            era: "1980s DOSE".into(),
        },
        CarouselItemData {
            title: "Star Goose".into(),
            image_path: "ui/thumbnails/star_goose.png".into(),
            era: "1980s DOSE".into(),
        },
        // 90s
        CarouselItemData {
            title: "Worm Wars".into(),
            image_path: "ui/thumbnails/worm_wars.png".into(),
            era: "1990s GOLDEN AGE".into(),
        },
        CarouselItemData {
            title: "Ice Blitz".into(),
            image_path: "ui/thumbnails/ice_blitz.png".into(),
            era: "1990s GOLDEN AGE".into(),
        },
        CarouselItemData {
            title: "Depths of Doom".into(),
            image_path: "ui/thumbnails/depths_of_doom.png".into(),
            era: "1990s GOLDEN AGE".into(),
        },
    ];

    state.current_index = 0;
    state.target_index = 0;

    // Root entity for easy cleanup
    commands
        .spawn((
            CarouselRoot,
            Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .with_children(|parent| {
            for (i, item) in state.items.iter().enumerate() {
                parent.spawn((
                    CarouselItem { index: i },
                    Sprite {
                        image: asset_server.load(&item.image_path),
                        custom_size: Some(Vec2::new(256.0, 192.0)), // 4:3 aspect
                        ..default()
                    },
                    Transform::from_scale(Vec3::ZERO), // Start hidden, animate in
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    GlobalTransform::default(),
                ));
            }
        });
}

fn carousel_input(mut state: ResMut<CarouselState>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::ArrowRight) {
        if state.target_index < state.items.len() - 1 {
            state.target_index += 1;
        } else {
            state.target_index = 0; // Loop back
        }
    } else if input.just_pressed(KeyCode::ArrowLeft) {
        if state.target_index > 0 {
            state.target_index -= 1;
        } else {
            state.target_index = state.items.len() - 1; // Loop to end
        }
    }
}

fn animate_carousel(
    mut state: ResMut<CarouselState>,
    mut query: Query<(&CarouselItem, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
) {
    // Smoothly interpolate current index towards target
    let speed = 5.0 * time.delta_secs();
    let _diff = state.target_index as f32 - state.current_index as f32;

    // Simple integer stepping for logic, but we could use float for smoother index if we wanted
    // For now, just snap current_index if close enough, or handle visual interpolation separately
    // Actually, let's keep state.current_index as usize for logic, but use a float for animation
    // To keep it simple for this iteration:

    state.current_index = state.target_index; // Snap for logic

    // Animate items based on distance from target
    for (item, mut transform, mut sprite) in &mut query {
        let distance = (item.index as i32 - state.target_index as i32) as f32;

        // Define spacing and curve
        let spacing = 300.0;
        let x_pos = distance * spacing;
        let mut z_pos = 50.0 - distance.abs() * 10.0; // Keep everything well in front of background (e.g. at -50)
        let scale = 1.0 - (distance.abs() * 0.2); // Center is larger

        // Clamp scale
        let scale = scale.max(0.5);

        // Opacity/Tint
        if distance.abs() < 0.5 {
            sprite.color = Color::WHITE;
            // Keep center item in front without accumulating z every frame.
            z_pos += 10.0;
        } else {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }

        // Target transform
        let target_translation = Vec3::new(x_pos, 0.0, z_pos);
        let target_scale = Vec3::splat(scale);

        // Interpolate
        transform.translation = transform.translation.lerp(target_translation, speed);
        transform.scale = transform.scale.lerp(target_scale, speed);
    }
}

fn cleanup_carousel(mut commands: Commands, query: Query<Entity, With<CarouselRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
