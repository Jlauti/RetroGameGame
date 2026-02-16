use bevy::prelude::*;

use crate::eras::era_future::nebula_bouncer::components::{OrbElement, OrbModifier};
use crate::eras::era_future::nebula_bouncer::procgen::{
    ProcgenPreflightSummary, ValidationCounters,
};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeedbackProfile {
    Safe,
    Normal,
    Intense,
}

impl FeedbackProfile {
    pub const ALL: [Self; 3] = [Self::Safe, Self::Normal, Self::Intense];

    pub const fn next(self) -> Self {
        match self {
            Self::Safe => Self::Normal,
            Self::Normal => Self::Intense,
            Self::Intense => Self::Safe,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Normal => "normal",
            Self::Intense => "intense",
        }
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

#[derive(Resource, Default)]
pub struct ProcgenValidatorTelemetry {
    pub profile_mismatch_rejections: u64,
    pub concave_trap_rejections: u64,
    pub exit_angle_fail_rejections: u64,
    pub selected_chunks: u64,
    pub preflight_total_chunks: usize,
    pub preflight_invalid_chunks: usize,
    pub preflight_summary_path: Option<String>,
}

impl ProcgenValidatorTelemetry {
    pub fn record_runtime_rejections(&mut self, counters: &ValidationCounters) {
        self.profile_mismatch_rejections += counters.profile_mismatch;
        self.concave_trap_rejections += counters.concave_trap;
        self.exit_angle_fail_rejections += counters.exit_angle_fail;
    }

    pub fn record_selection(&mut self) {
        self.selected_chunks += 1;
    }

    pub fn set_preflight(&mut self, summary: &ProcgenPreflightSummary, artifact_path: String) {
        self.preflight_total_chunks = summary.total_chunks;
        self.preflight_invalid_chunks = summary.invalid_chunks;
        self.preflight_summary_path = Some(artifact_path);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FeedbackTuning {
    pub shake_damage_threshold: f32,
    pub shake_damage_scale: f32,
    pub wall_damage_factor: f32,
    pub shake_cap: f32,
    pub shake_decay: f32,
    pub hit_stop_damage_threshold: f32,
    pub hit_stop_damage_scale: f32,
    pub hit_stop_min: f32,
    pub hit_stop_max: f32,
}

pub const fn feedback_tuning(profile: FeedbackProfile) -> FeedbackTuning {
    match profile {
        FeedbackProfile::Safe => FeedbackTuning {
            shake_damage_threshold: 14.0,
            shake_damage_scale: 0.12,
            wall_damage_factor: 0.25,
            shake_cap: 10.0,
            shake_decay: 13.0,
            hit_stop_damage_threshold: 16.0,
            hit_stop_damage_scale: 0.003,
            hit_stop_min: 0.03,
            hit_stop_max: 0.10,
        },
        FeedbackProfile::Normal => FeedbackTuning {
            shake_damage_threshold: 11.0,
            shake_damage_scale: 0.18,
            wall_damage_factor: 0.30,
            shake_cap: 16.0,
            shake_decay: 10.0,
            hit_stop_damage_threshold: 12.0,
            hit_stop_damage_scale: 0.0045,
            hit_stop_min: 0.04,
            hit_stop_max: 0.16,
        },
        FeedbackProfile::Intense => FeedbackTuning {
            shake_damage_threshold: 8.0,
            shake_damage_scale: 0.24,
            wall_damage_factor: 0.40,
            shake_cap: 24.0,
            shake_decay: 7.5,
            hit_stop_damage_threshold: 8.0,
            hit_stop_damage_scale: 0.006,
            hit_stop_min: 0.05,
            hit_stop_max: 0.22,
        },
    }
}

#[derive(Resource)]
pub struct CameraFeedbackSettings {
    pub profile: FeedbackProfile,
    pub shake_enabled: bool,
}

impl Default for CameraFeedbackSettings {
    fn default() -> Self {
        Self {
            profile: FeedbackProfile::Normal,
            shake_enabled: true,
        }
    }
}

pub fn compute_shake_increment(
    impact_damage: f32,
    is_wall_contact: bool,
    profile: FeedbackProfile,
) -> f32 {
    let tuning = feedback_tuning(profile);
    let effective_damage = if is_wall_contact {
        impact_damage * tuning.wall_damage_factor
    } else {
        impact_damage
    };

    if effective_damage <= tuning.shake_damage_threshold {
        0.0
    } else {
        (effective_damage - tuning.shake_damage_threshold) * tuning.shake_damage_scale
    }
}

pub fn next_shake_intensity(
    current_intensity: f32,
    impact_damage: f32,
    is_wall_contact: bool,
    shake_enabled: bool,
    profile: FeedbackProfile,
) -> f32 {
    if !shake_enabled {
        return current_intensity;
    }

    let tuning = feedback_tuning(profile);
    let added = compute_shake_increment(impact_damage, is_wall_contact, profile);
    (current_intensity + added).clamp(0.0, tuning.shake_cap)
}

pub fn compute_hit_stop_duration(impact_damage: f32, profile: FeedbackProfile) -> f32 {
    let tuning = feedback_tuning(profile);
    if impact_damage <= tuning.hit_stop_damage_threshold {
        0.0
    } else {
        ((impact_damage - tuning.hit_stop_damage_threshold) * tuning.hit_stop_damage_scale)
            .clamp(tuning.hit_stop_min, tuning.hit_stop_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < 0.0001, "{a} != {b}");
    }

    #[test]
    fn low_damage_produces_no_shake() {
        let shake = compute_shake_increment(10.0, false, FeedbackProfile::Normal);
        approx_eq(shake, 0.0);
    }

    #[test]
    fn high_damage_produces_non_zero_shake() {
        let shake = compute_shake_increment(36.0, false, FeedbackProfile::Normal);
        assert!(shake > 0.0);
    }

    #[test]
    fn shake_is_capped() {
        let intensity = next_shake_intensity(15.5, 999.0, false, true, FeedbackProfile::Normal);
        approx_eq(
            intensity,
            feedback_tuning(FeedbackProfile::Normal).shake_cap,
        );
    }

    #[test]
    fn hit_stop_respects_threshold_and_cap() {
        approx_eq(
            compute_hit_stop_duration(10.0, FeedbackProfile::Normal),
            0.0,
        );
        approx_eq(
            compute_hit_stop_duration(999.0, FeedbackProfile::Normal),
            feedback_tuning(FeedbackProfile::Normal).hit_stop_max,
        );
    }

    #[test]
    fn toggle_off_prevents_shake_growth() {
        let current = 1.75;
        let intensity =
            next_shake_intensity(current, 200.0, false, false, FeedbackProfile::Intense);
        approx_eq(intensity, current);
    }

    #[test]
    fn preset_mapping_is_deterministic() {
        let safe = feedback_tuning(FeedbackProfile::Safe);
        let normal = feedback_tuning(FeedbackProfile::Normal);
        let intense = feedback_tuning(FeedbackProfile::Intense);

        approx_eq(safe.shake_damage_threshold, 14.0);
        approx_eq(normal.shake_damage_threshold, 11.0);
        approx_eq(intense.shake_damage_threshold, 8.0);

        approx_eq(safe.shake_cap, 10.0);
        approx_eq(normal.shake_cap, 16.0);
        approx_eq(intense.shake_cap, 24.0);

        assert_eq!(FeedbackProfile::Safe.next(), FeedbackProfile::Normal);
        assert_eq!(FeedbackProfile::Normal.next(), FeedbackProfile::Intense);
        assert_eq!(FeedbackProfile::Intense.next(), FeedbackProfile::Safe);
    }
}
