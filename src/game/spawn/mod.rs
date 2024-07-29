//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;

pub mod level;
pub mod npc;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin, npc::plugin));
    app.init_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Intro,
    First,
    Second,
    Third,
    Ending,
}
