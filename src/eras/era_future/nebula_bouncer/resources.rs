use bevy::prelude::*;

use crate::eras::era_future::nebula_bouncer::components::{OrbElement, OrbModifier};

#[derive(Resource, Default)]
pub struct KineticOrbPool {
    pub inactive: Vec<Entity>,
    pub active_count: usize,
    pub capacity: usize,
}

impl KineticOrbPool {
    pub const DEFAULT_CAPACITY: usize = 100;

    pub fn new(capacity: usize) -> Self {
        Self {
            inactive: Vec::with_capacity(capacity),
            active_count: 0,
            capacity,
        }
    }

    pub fn push(&mut self, entity: Entity) {
        if self.inactive.len() < self.capacity {
            self.inactive.push(entity);
            self.active_count = self.active_count.saturating_sub(1);
        } else {
            // If full, maybe just despawn? But here we are pooling.
            // For now, let's keep it simple.
            warn!("Orb pool overflow, discarding entity {:?}", entity);
        }
    }

    pub fn pop(&mut self) -> Option<Entity> {
        let entity = self.inactive.pop();
        if entity.is_some() {
            self.active_count += 1;
        }
        entity
    }
}

#[derive(Resource, Default)]
pub struct HitStop {
    pub timer: f32,
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct ActiveLoadout {
    pub element: OrbElement,
    pub modifier: OrbModifier,
    pub last_telemetry_time: f32,
}

impl Default for ActiveLoadout {
    fn default() -> Self {
        Self {
            element: OrbElement::default(),
            modifier: OrbModifier::default(),
            last_telemetry_time: f32::NEG_INFINITY,
        }
    }
}

impl ActiveLoadout {
    pub fn cycle_element(&mut self) {
        let next = (self.element.index() + 1) % OrbElement::ALL.len();
        self.element = OrbElement::ALL[next];
    }

    pub fn cycle_modifier(&mut self) {
        let next = (self.modifier.index() + 1) % OrbModifier::ALL.len();
        self.modifier = OrbModifier::ALL[next];
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbSynergyProfile {
    pub damage_scale: f32,
    pub speed_scale: f32,
    pub bounce_delta: i32,
    pub radius_scale: f32,
    pub cryo_slow_factor: f32,
    pub cryo_duration_secs: f32,
    pub void_dot_dps: f32,
    pub void_dot_duration_secs: f32,
}

impl OrbSynergyProfile {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        damage_scale: f32,
        speed_scale: f32,
        bounce_delta: i32,
        radius_scale: f32,
        cryo_slow_factor: f32,
        cryo_duration_secs: f32,
        void_dot_dps: f32,
        void_dot_duration_secs: f32,
    ) -> Self {
        Self {
            damage_scale,
            speed_scale,
            bounce_delta,
            radius_scale,
            cryo_slow_factor,
            cryo_duration_secs,
            void_dot_dps,
            void_dot_duration_secs,
        }
    }
}

impl Default for OrbSynergyProfile {
    fn default() -> Self {
        Self::new(1.0, 1.0, 0, 1.0, 1.0, 0.0, 0.0, 0.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbSpawnStats {
    pub damage: f32,
    pub speed: f32,
    pub bounces: u32,
    pub radius: f32,
}

impl OrbSpawnStats {
    pub const fn new(damage: f32, speed: f32, bounces: u32, radius: f32) -> Self {
        Self {
            damage,
            speed,
            bounces,
            radius,
        }
    }
}

pub const DAMAGE_MIN: f32 = 4.0;
pub const DAMAGE_MAX: f32 = 80.0;
pub const SPEED_MIN: f32 = 220.0;
pub const SPEED_MAX: f32 = 1200.0;
pub const BOUNCE_MIN: i32 = 0;
pub const BOUNCE_MAX: i32 = 8;
pub const RADIUS_MIN: f32 = 3.0;
pub const RADIUS_MAX: f32 = 14.0;

pub fn resolve_orb_spawn_stats(base: OrbSpawnStats, profile: OrbSynergyProfile) -> OrbSpawnStats {
    let damage = (base.damage * profile.damage_scale).clamp(DAMAGE_MIN, DAMAGE_MAX);
    let speed = (base.speed * profile.speed_scale).clamp(SPEED_MIN, SPEED_MAX);
    let bounces = ((base.bounces as i32) + profile.bounce_delta).clamp(BOUNCE_MIN, BOUNCE_MAX);
    let radius = (base.radius * profile.radius_scale).clamp(RADIUS_MIN, RADIUS_MAX);

    OrbSpawnStats::new(damage, speed, bounces as u32, radius)
}

#[derive(Resource)]
pub struct OrbSynergyMatrix {
    entries: [[OrbSynergyProfile; 4]; 4],
}

impl OrbSynergyMatrix {
    pub const ENTRY_COUNT: usize = OrbElement::ALL.len() * OrbModifier::ALL.len();

    pub fn get(&self, element: OrbElement, modifier: OrbModifier) -> OrbSynergyProfile {
        self.entries[element.index()][modifier.index()]
    }

    pub fn iter(&self) -> impl Iterator<Item = (OrbElement, OrbModifier, OrbSynergyProfile)> + '_ {
        OrbElement::ALL.into_iter().flat_map(move |element| {
            OrbModifier::ALL
                .into_iter()
                .map(move |modifier| (element, modifier, self.get(element, modifier)))
        })
    }
}

impl Default for OrbSynergyMatrix {
    fn default() -> Self {
        Self {
            entries: [
                // Plasma
                [
                    OrbSynergyProfile::new(1.00, 1.00, 0, 1.00, 1.00, 0.0, 0.0, 0.0), // Elasticity (neutral baseline)
                    OrbSynergyProfile::new(0.90, 1.10, 1, 0.90, 1.00, 0.0, 0.0, 0.0), // Splinter
                    OrbSynergyProfile::new(1.25, 0.86, 1, 1.15, 1.00, 0.0, 0.0, 0.0), // Mass
                    OrbSynergyProfile::new(0.95, 1.25, 0, 0.88, 1.00, 0.0, 0.0, 0.0), // Velocity
                ],
                // Cryo
                [
                    OrbSynergyProfile::new(1.00, 0.96, 1, 1.02, 0.75, 1.6, 0.0, 0.0), // Elasticity
                    OrbSynergyProfile::new(0.92, 1.02, 0, 0.95, 0.70, 1.3, 0.0, 0.0), // Splinter
                    OrbSynergyProfile::new(1.15, 0.84, 1, 1.18, 0.60, 2.0, 0.0, 0.0), // Mass
                    OrbSynergyProfile::new(0.90, 1.20, 0, 0.90, 0.72, 1.1, 0.0, 0.0), // Velocity
                ],
                // Tesla
                [
                    OrbSynergyProfile::new(1.05, 1.04, 0, 1.00, 1.00, 0.0, 0.0, 0.0), // Elasticity
                    OrbSynergyProfile::new(0.96, 1.12, 1, 0.92, 1.00, 0.0, 0.0, 0.0), // Splinter
                    OrbSynergyProfile::new(1.20, 0.90, 1, 1.12, 1.00, 0.0, 0.0, 0.0), // Mass
                    OrbSynergyProfile::new(0.94, 1.30, 0, 0.86, 1.00, 0.0, 0.0, 0.0), // Velocity
                ],
                // Void
                [
                    OrbSynergyProfile::new(1.03, 0.98, 0, 1.04, 1.00, 0.0, 4.0, 1.8), // Elasticity
                    OrbSynergyProfile::new(0.94, 1.06, 1, 0.92, 1.00, 0.0, 3.0, 1.4), // Splinter
                    OrbSynergyProfile::new(1.20, 0.82, 1, 1.20, 1.00, 0.0, 6.0, 2.2), // Mass
                    OrbSynergyProfile::new(0.90, 1.28, 0, 0.86, 1.00, 0.0, 3.5, 1.2), // Velocity
                ],
            ],
        }
    }
}
