use bevy::prelude::*;

// ─── Core game components shared across mini-games ─────────────────

/// Marks an entity as the player.
#[derive(Component, Debug)]
pub struct Player;

/// Marks an entity as an enemy.
#[derive(Component, Debug)]
pub struct Enemy;

/// Health component with current and max values.
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    pub fn fraction(&self) -> f32 {
        if self.max == 0 {
            0.0
        } else {
            self.current as f32 / self.max as f32
        }
    }
}

/// Score tracking for a mini-game session.
#[derive(Component, Resource, Debug, Clone, Default)]
pub struct Score {
    pub value: u64,
}

impl Score {
    pub fn add(&mut self, points: u64) {
        self.value += points;
    }
}

/// Lives counter.
#[derive(Component, Resource, Debug, Clone)]
pub struct Lives {
    pub count: i32,
}

impl Lives {
    pub fn new(count: i32) -> Self {
        Self { count }
    }

    pub fn lose_one(&mut self) {
        self.count = (self.count - 1).max(0);
    }

    pub fn is_game_over(&self) -> bool {
        self.count <= 0
    }
}

/// Velocity component for simple 2D movement.
#[derive(Component, Debug, Clone, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// Simple AABB collider.
#[derive(Component, Debug, Clone)]
pub struct BoxCollider {
    pub half_width: f32,
    pub half_height: f32,
}

impl BoxCollider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_width: width / 2.0,
            half_height: height / 2.0,
        }
    }
}

/// Grid position for grid-based games (e.g., Tunnel Miner).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Timer-based animation player.
#[derive(Component, Debug)]
pub struct AnimationPlayer {
    pub timer: Timer,
    pub current_frame: usize,
    pub frame_count: usize,
    pub looping: bool,
}

impl AnimationPlayer {
    pub fn new(fps: f32, frame_count: usize, looping: bool) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            current_frame: 0,
            frame_count,
            looping,
        }
    }

    pub fn tick(&mut self, delta: std::time::Duration) -> bool {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            if self.current_frame + 1 < self.frame_count {
                self.current_frame += 1;
                return true;
            } else if self.looping {
                self.current_frame = 0;
                return true;
            }
        }
        false
    }
}
