use bevy::prelude::*;

/// Screen transition effects (fade, static, etc).
pub struct TransitionsPlugin;

impl Plugin for TransitionsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_event::<ScreenTransitionEvent>()
        //     .add_systems(Update, (start_transition, update_transition));
    }
}

/// Event to trigger a screen transition.
#[derive(Event)]
pub struct ScreenTransitionEvent {
    pub kind: TransitionKind,
    pub duration: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum TransitionKind {
    FadeToBlack,
    FadeFromBlack,
    Static,
}

/// Component on the transition overlay entity.
#[derive(Component)]
struct TransitionOverlay {
    kind: TransitionKind,
    timer: Timer,
}

fn start_transition(
    // mut events: EventReader<ScreenTransitionEvent>, // Error: EventReader not found
) {
    // Placeholder
}

fn update_transition(
    // time: Res<Time>,
    // mut query: Query<(&mut TransitionOverlay, &mut BackgroundColor, Entity)>,
    // mut commands: Commands,
) {
    // Placeholder
}
