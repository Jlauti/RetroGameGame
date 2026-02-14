/// Shared traits and utilities used across all era mini-games.

/// Trait that all mini-games should implement for consistent lifecycle.
pub trait MiniGame {
    /// Human-readable name of the mini-game.
    fn name(&self) -> &'static str;

    /// Score threshold required to "complete" this mini-game.
    fn completion_threshold(&self) -> u64;

    /// Number of starting lives.
    fn starting_lives(&self) -> i32 {
        3
    }
}
