use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct KineticOrb {
    pub active: bool,
    pub damage: f32,
    pub bounces_remaining: u32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerShip;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Wall;

/// Marker for entities that should participate in specific collision layers
#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Projectile,
    Wall,
}

pub mod depth {
    pub const BACKGROUND: f32 = 0.0;
    pub const WALL: f32 = 5.0;
    pub const DEBRIS: f32 = 10.0;
    pub const ENEMY: f32 = 20.0;
    pub const PLAYER: f32 = 30.0;
    pub const PROJECTILE: f32 = 40.0;
    pub const PARTICLES: f32 = 50.0;
    pub const UI: f32 = 100.0;
}

#[derive(Component, Default)]
pub struct ProjectileTrail {
    pub points: Vec<Vec3>,
    pub max_length: usize,
    pub width: f32,
    pub color: Color,
}

#[derive(Component, Default)]
pub struct ScreenShake {
    pub intensity: f32,
    pub decay: f32,
    pub last_offset: Vec2,
}
