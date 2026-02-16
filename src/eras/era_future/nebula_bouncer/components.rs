use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct KineticOrb {
    pub active: bool,
    pub damage: f32,
    pub bounces_remaining: u32,
    pub element: OrbElement,
    pub modifier: OrbModifier,
    pub radius_scale: f32,
    pub speed_scale: f32,
    pub cryo_slow_factor: f32,
    pub cryo_duration_secs: f32,
    pub void_dot_dps: f32,
    pub void_dot_duration_secs: f32,
}

impl Default for KineticOrb {
    fn default() -> Self {
        Self {
            active: false,
            damage: 0.0,
            bounces_remaining: 0,
            element: OrbElement::default(),
            modifier: OrbModifier::default(),
            radius_scale: 1.0,
            speed_scale: 1.0,
            cryo_slow_factor: 1.0,
            cryo_duration_secs: 0.0,
            void_dot_dps: 0.0,
            void_dot_duration_secs: 0.0,
        }
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
#[reflect(Component)]
pub enum OrbElement {
    #[default]
    Plasma,
    Cryo,
    Tesla,
    Void,
}

impl OrbElement {
    pub const ALL: [Self; 4] = [Self::Plasma, Self::Cryo, Self::Tesla, Self::Void];

    pub const fn index(self) -> usize {
        match self {
            Self::Plasma => 0,
            Self::Cryo => 1,
            Self::Tesla => 2,
            Self::Void => 3,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Plasma => "Plasma",
            Self::Cryo => "Cryo",
            Self::Tesla => "Tesla",
            Self::Void => "Void",
        }
    }
}

#[derive(Component, Reflect, Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
#[reflect(Component)]
pub enum OrbModifier {
    #[default]
    Elasticity,
    Splinter,
    Mass,
    Velocity,
}

impl OrbModifier {
    pub const ALL: [Self; 4] = [Self::Elasticity, Self::Splinter, Self::Mass, Self::Velocity];

    pub const fn index(self) -> usize {
        match self {
            Self::Elasticity => 0,
            Self::Splinter => 1,
            Self::Mass => 2,
            Self::Velocity => 3,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Elasticity => "Elasticity",
            Self::Splinter => "Splinter",
            Self::Mass => "Mass",
            Self::Velocity => "Velocity",
        }
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct EnemyStatusEffects {
    pub cryo_slow_timer: f32,
    pub cryo_slow_factor: f32,
    pub void_dot_timer: f32,
    pub void_dot_dps: f32,
    pub void_dot_tick_timer: f32,
}

impl Default for EnemyStatusEffects {
    fn default() -> Self {
        Self {
            cryo_slow_timer: 0.0,
            cryo_slow_factor: 1.0,
            void_dot_timer: 0.0,
            void_dot_dps: 0.0,
            void_dot_tick_timer: 0.0,
        }
    }
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
