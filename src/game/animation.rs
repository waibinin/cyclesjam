//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use std::time::Duration;

use bevy::prelude::*;

use super::{audio::sfx::PlaySfx, movement::MovementController};
use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.register_type::<PlayerAnimation>();
    app.register_type::<BasicAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSet::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sfx,
            )
                .chain()
                .in_set(AppSet::Update),
        ),
    );
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.0.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.0 == Vec2::ZERO {
            PlayerAnimationState::Idling
        } else {
            PlayerAnimationState::Walking
        };
        animation.update_state(animation_state);
    }
}

/// Update the animation timer.
fn update_animation_timer(
    time: Res<Time>,
    mut query0: Query<&mut PlayerAnimation>,
    mut query1: Query<&mut BasicAnimation>,
) {
    for mut player_animation in &mut query0 {
        player_animation.update_timer(time.delta());
    }

    for mut basic_animation in &mut query1 {
        // println!(
        //     "Before update: Timer: {:?}, Frame: {}",
        //     basic_animation.timer.elapsed(),
        //     basic_animation.frame
        // ); // Debugging statement
        basic_animation.update_timer(time.delta());
        // println!(
        //     "After update: Timer: {:?}, Frame: {}",
        //     basic_animation.timer.elapsed(),
        //     basic_animation.frame
        // ); // Debugging statement
    }
}

fn update_animation_atlas(
    mut animations: ParamSet<(
        Query<(&PlayerAnimation, &mut TextureAtlas)>,
        Query<(&BasicAnimation, &mut TextureAtlas)>,
    )>,
) {
    {
        let mut query0 = animations.p0();
        for (animation, mut atlas) in query0.iter_mut() {
            if animation.changed() {
                atlas.index = animation.get_atlas_index();
            }
        }
    }
    {
        let mut query1 = animations.p1();
        for (animation, mut atlas) in query1.iter_mut() {
            if animation.changed() {
                atlas.index = animation.get_atlas_index();
            }
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the animation.
fn trigger_step_sfx(mut commands: Commands, mut step_query: Query<&PlayerAnimation>) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            commands.trigger(PlaySfx::RandomStep);
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BasicAnimation {
    timer: Timer,
    frame: usize,
    num_frames: usize,
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idling,
    Walking,
}
impl BasicAnimation {
    /// The duration of each idle frame.
    const INTERVAL: Duration = Duration::from_millis(500);

    pub fn new(num_frames: usize) -> Self {
        Self::idling(num_frames)
    }

    fn idling(num_frames: usize) -> Self {
        Self {
            timer: Timer::new(Self::INTERVAL, TimerMode::Repeating),
            frame: 0,
            num_frames,
        }
    }
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() {
            self.frame = (self.frame + 1) % self.num_frames;
        }
    }
    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        self.frame
    }
    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }
}

impl PlayerAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 2;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
        }
    }

    /// The number of walking frames.
    const WALKING_FRAMES: usize = 6;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking => Self::WALKING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling => *self = Self::idling(),
                PlayerAnimationState::Walking => *self = Self::walking(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idling => self.frame,
            PlayerAnimationState::Walking => 6 + self.frame,
        }
    }
}
