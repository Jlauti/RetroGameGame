use bevy::prelude::*;

/// Plugin that registers all game states.
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<MenuState>()
            .add_sub_state::<PlayingState>();
    }
}

// ─── Top-level game state ──────────────────────────────────────────

/// The top-level state machine for the entire application.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Initial boot / splash screen
    Boot,
    /// Main menu and its sub-screens
    Menu,
    /// The timeline hub where the player picks an era
    #[default]
    Timeline,
    /// Inside an era — browsing mini-games
    EraSelect,
    /// Actively playing a mini-game
    Playing,
    /// Results / score screen after a mini-game
    Results,
    /// Settings screen
    Settings,
    /// Credits screen
    Credits,
}

// ─── Menu sub-state ────────────────────────────────────────────────

/// Sub-state active only when `GameState::Menu` is set.
#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Menu)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
    Credits,
}

// ─── Playing sub-state ─────────────────────────────────────────────

/// Sub-state active only when `GameState::Playing` is set.
/// Identifies which mini-game is currently loaded.
#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Playing)]
pub enum PlayingState {
    // Era 1: 1980s
    #[default]
    TunnelMiner,
    CosmicCaptain,
    StarGoose,
    // Era 2: 1990s
    WormWars,
    IceBlitz,
    DepthsOfDoom,
    // Era Future
    NebulaBouncer,
}

// ─── Era identification ────────────────────────────────────────────

/// Identifies a gaming era (decade).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Era {
    The80s,
    The90s,
    The2000s,
    The2010s,
    Future,
}

impl Era {
    /// Human-readable display name for the era.
    pub fn display_name(&self) -> &'static str {
        match self {
            Era::The80s => "The 1980s — The DOS Age",
            Era::The90s => "The 1990s — The Golden Age",
            Era::The2000s => "The 2000s",
            Era::The2010s => "The 2010s",
            Era::Future => "Future — Experimental Frontier",
        }
    }

    /// Short label for UI.
    pub fn label(&self) -> &'static str {
        match self {
            Era::The80s => "1980s",
            Era::The90s => "1990s",
            Era::The2000s => "2000s",
            Era::The2010s => "2010s",
            Era::Future => "Future",
        }
    }
}

/// Identifies a specific mini-game within an era.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MiniGameId {
    pub era: Era,
    pub index: u8, // 0-based index within the era
}

impl MiniGameId {
    pub fn display_name(&self) -> &'static str {
        match (self.era, self.index) {
            (Era::The80s, 0) => "Tunnel Miner",
            (Era::The80s, 1) => "Cosmic Captain",
            (Era::The80s, 2) => "Star Goose",
            (Era::The90s, 0) => "Worm Wars",
            (Era::The90s, 1) => "Ice Blitz",
            (Era::The90s, 2) => "Depths of Doom",
            (Era::Future, 0) => "Nebula Bouncer",
            _ => "Unknown",
        }
    }

    pub fn description(&self) -> &'static str {
        match (self.era, self.index) {
            (Era::The80s, 0) => "Dig tunnels, collect emeralds, crush enemies!",
            (Era::The80s, 1) => "Side-scrolling exploration across alien worlds",
            (Era::The80s, 2) => "Vertical-scrolling shooter over alien terrain",
            (Era::The90s, 0) => "Turn-based artillery with destructible terrain",
            (Era::The90s, 1) => "Fast-paced top-down arcade ice hockey",
            (Era::The90s, 2) => "Turn-based roguelike dungeon crawler",
            (Era::Future, 0) => "Ricochet-driven sci-fi shooter with buildcrafting",
            _ => "",
        }
    }

    pub fn playing_state(&self) -> PlayingState {
        match (self.era, self.index) {
            (Era::The80s, 0) => PlayingState::TunnelMiner,
            (Era::The80s, 1) => PlayingState::CosmicCaptain,
            (Era::The80s, 2) => PlayingState::StarGoose,
            (Era::The90s, 0) => PlayingState::WormWars,
            (Era::The90s, 1) => PlayingState::IceBlitz,
            (Era::The90s, 2) => PlayingState::DepthsOfDoom,
            (Era::Future, 0) => PlayingState::NebulaBouncer,
            _ => PlayingState::TunnelMiner,
        }
    }
}
