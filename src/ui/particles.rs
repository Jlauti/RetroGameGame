use crate::core::states::GameState;
use crate::ui::colors;
use bevy::prelude::*;
use rand::prelude::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_particles)
            .add_systems(
                Update,
                (emit_particles, update_particles).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_particles);
    }
}

#[derive(Component)]
struct ParticleEmitter {
    timer: Timer,
}

#[derive(Component)]
struct Particle {
    velocity: Vec3,
    lifetime: Timer,
}

#[derive(Component)]
struct ParticleRoot;

fn setup_particles(mut commands: Commands) {
    commands.spawn((
        ParticleRoot,
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ParticleEmitter {
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        },
    ));
}

fn emit_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut ParticleEmitter, Entity)>,
) {
    let mut rng = rand::rng();

    for (mut emitter, parent_entity) in &mut query {
        emitter.timer.tick(time.delta());

        if emitter.timer.just_finished() {
            // Spawn a new particle
            let x = rng.random_range(-600.0..600.0);
            let y = rng.random_range(-100.0..100.0);
            let size = rng.random_range(2.0..5.0);

            let color = if rng.random_bool(0.5) {
                colors::EGA_BRIGHT_CYAN
            } else {
                colors::EGA_BRIGHT_MAGENTA
            };

            commands.entity(parent_entity).with_children(|parent| {
                parent.spawn((
                    Particle {
                        velocity: Vec3::new(
                            rng.random_range(-20.0..20.0),
                            rng.random_range(10.0..50.0),
                            0.0,
                        ),
                        lifetime: Timer::from_seconds(rng.random_range(2.0..4.0), TimerMode::Once),
                    },
                    Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size)),
                        ..default()
                    },
                    Transform::from_xyz(x, -300.0 + y, 0.0), // In front of BG (-10), behind carousel (>20)
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    GlobalTransform::default(),
                ));
            });
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut transform, mut sprite) in &mut query {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.just_finished() {
            commands.entity(entity).despawn();
        } else {
            // Move
            transform.translation += particle.velocity * time.delta_secs();

            // Fade out
            let alpha = particle.lifetime.fraction_remaining();
            let mut color = sprite.color.to_srgba();
            color.alpha = alpha;
            sprite.color = Color::Srgba(color);
        }
    }
}

fn cleanup_particles(mut commands: Commands, query: Query<Entity, With<ParticleRoot>>) {
    // This will despawn the root and all its children (particles)
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
