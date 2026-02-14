use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::states::{Era, MiniGameId};

/// Plugin for save/load and progression tracking.
pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerProgress>()
            .add_systems(Startup, load_progress);
    }
}

// ─── Resources ─────────────────────────────────────────────────────

/// Tracks the player's overall progress across all eras and mini-games.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProgress {
    /// Total tokens earned across all games
    pub tokens: u64,
    /// Per-game high scores, keyed by (era_index, game_index)
    pub high_scores: HashMap<(u8, u8), u64>,
    /// Which eras are unlocked (era_index → unlocked)
    pub eras_unlocked: HashMap<u8, bool>,
    /// Which mini-games are unlocked (era_index, game_index → unlocked)
    pub games_unlocked: HashMap<(u8, u8), bool>,
    /// Which mini-games have been completed (reached score threshold)
    pub games_completed: HashMap<(u8, u8), bool>,
}

impl Default for PlayerProgress {
    fn default() -> Self {
        let mut eras_unlocked = HashMap::new();
        eras_unlocked.insert(0, true); // 80s always unlocked

        let mut games_unlocked = HashMap::new();
        games_unlocked.insert((0, 0), true); // Tunnel Miner always unlocked

        Self {
            tokens: 0,
            high_scores: HashMap::new(),
            eras_unlocked,
            games_unlocked,
            games_completed: HashMap::new(),
        }
    }
}

impl PlayerProgress {
    /// Check if an era is unlocked.
    pub fn is_era_unlocked(&self, era: Era) -> bool {
        let idx = era_to_index(era);
        *self.eras_unlocked.get(&idx).unwrap_or(&false)
    }

    /// Check if a specific mini-game is unlocked.
    pub fn is_game_unlocked(&self, game: MiniGameId) -> bool {
        let key = game_to_key(game);
        *self.games_unlocked.get(&key).unwrap_or(&false)
    }

    /// Check if a specific mini-game has been completed.
    pub fn is_game_completed(&self, game: MiniGameId) -> bool {
        let key = game_to_key(game);
        *self.games_completed.get(&key).unwrap_or(&false)
    }

    /// Get high score for a mini-game.
    pub fn high_score(&self, game: MiniGameId) -> u64 {
        let key = game_to_key(game);
        *self.high_scores.get(&key).unwrap_or(&0)
    }

    /// Record a game result. Returns tokens earned.
    pub fn record_result(&mut self, game: MiniGameId, score: u64, threshold: u64) -> u64 {
        let key = game_to_key(game);

        // Update high score
        let prev_high = *self.high_scores.get(&key).unwrap_or(&0);
        if score > prev_high {
            self.high_scores.insert(key, score);
        }

        // Check completion
        let newly_completed = score >= threshold && !self.is_game_completed(game);
        if score >= threshold {
            self.games_completed.insert(key, true);
        }

        // Calculate tokens
        let tokens = (score / 100).max(1);
        let bonus = if newly_completed { 50 } else { 0 };
        let total_tokens = tokens + bonus;
        self.tokens += total_tokens;

        // Unlock next game/era if newly completed
        if newly_completed {
            self.unlock_next(game);
        }

        total_tokens
    }

    /// Unlock the next game in sequence, or the next era.
    fn unlock_next(&mut self, completed: MiniGameId) {
        let era_idx = era_to_index(completed.era);
        let game_idx = completed.index;

        // Try to unlock next game in same era
        let games_in_era = games_per_era(completed.era);
        if game_idx + 1 < games_in_era {
            self.games_unlocked.insert((era_idx, game_idx + 1), true);
        }

        // Check if all games in era are completed → unlock next era
        let all_done = (0..games_in_era).all(|i| {
            *self.games_completed.get(&(era_idx, i)).unwrap_or(&false)
        });

        if all_done {
            let next_era = era_idx + 1;
            self.eras_unlocked.insert(next_era, true);
            self.games_unlocked.insert((next_era, 0), true);
        }
    }

    /// Save progress to disk.
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let path = save_file_path();
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&path, json);
            info!("Progress saved to {:?}", path);
        }
    }

    /// Load progress from disk.
    pub fn load() -> Option<Self> {
        let path = save_file_path();
        if path.exists() {
            if let Ok(json) = std::fs::read_to_string(&path) {
                if let Ok(progress) = serde_json::from_str(&json) {
                    info!("Progress loaded from {:?}", path);
                    return Some(progress);
                }
            }
        }
        None
    }
}

// ─── Helper functions ──────────────────────────────────────────────

fn era_to_index(era: Era) -> u8 {
    match era {
        Era::The80s => 0,
        Era::The90s => 1,
        Era::The2000s => 2,
        Era::The2010s => 3,
    }
}

fn game_to_key(game: MiniGameId) -> (u8, u8) {
    (era_to_index(game.era), game.index)
}

fn games_per_era(era: Era) -> u8 {
    match era {
        Era::The80s => 3,
        Era::The90s => 3,
        Era::The2000s => 0,
        Era::The2010s => 0,
    }
}

fn save_file_path() -> std::path::PathBuf {
    // Use the platform-appropriate data directory
    if let Some(proj_dirs) = dirs_next_or_fallback() {
        proj_dirs.join("retrogamegame").join("save.json")
    } else {
        std::path::PathBuf::from("save.json")
    }
}

fn dirs_next_or_fallback() -> Option<std::path::PathBuf> {
    // Simple fallback: use current directory's "data" subfolder
    std::env::current_dir().ok().map(|p| p.join("data"))
}

// ─── Systems ───────────────────────────────────────────────────────

fn load_progress(mut progress: ResMut<PlayerProgress>) {
    if let Some(loaded) = PlayerProgress::load() {
        *progress = loaded;
    }
}
