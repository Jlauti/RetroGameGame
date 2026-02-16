use bevy::prelude::*;

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
