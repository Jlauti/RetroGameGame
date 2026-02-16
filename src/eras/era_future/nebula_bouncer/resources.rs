use bevy::prelude::*;

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
