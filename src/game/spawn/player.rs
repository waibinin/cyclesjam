//! Spawn the player.

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{Movement, MovementController, WrapWithinWindow},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    player: Player,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    movement_controller: MovementController,
    movement: Movement,
    wrap_within_window: WrapWithinWindow,
    player_animation: PlayerAnimation,
    state_scoped: StateScoped<Screen>,
    rigid_body: RigidBody,
    gravity_scale: GravityScale,
    collider: Collider,
    collider_density: ColliderDensity,
    friction: Friction,
    locked_axes: LockedAxes,
    linear_damping: LinearDamping,
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn(PlayerBundle {
        name: Name::new("Player"),
        player: Player,
        sprite: SpriteBundle {
            texture: image_handles[&ImageKey::Ducky].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(4.0).extend(1.0))
                .with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        },
        texture_atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: player_animation.get_atlas_index(),
        },
        movement_controller: MovementController::default(),
        movement: Movement { speed: 420.0 },
        wrap_within_window: WrapWithinWindow,
        player_animation,
        state_scoped: StateScoped(Screen::Playing),
        rigid_body: RigidBody::Kinematic,
        gravity_scale: GravityScale(0.0),
        collider: Collider::rectangle(20.0, 20.0),
        collider_density: ColliderDensity(1.5),
        friction: Friction::new(0.8),
        locked_axes: LockedAxes::ROTATION_LOCKED.lock_translation_y(),
        linear_damping: LinearDamping(0.8),
        //     Name::new("Player"),
        //     Player,
        //     SpriteBundle {
        //         texture: image_handles[&ImageKey::Ducky].clone_weak(),
        //         transform: Transform::from_scale(Vec2::splat(4.0).extend(1.0))
        //             .with_translation(Vec3::new(0.0, 0.0, 1.0)),
        //         ..Default::default()
        //     },
        //     TextureAtlas {
        //         layout: texture_atlas_layout.clone(),
        //         index: player_animation.get_atlas_index(),
        //     },
        //     MovementController::default(),
        //     Movement { speed: 420.0 },
        //     WrapWithinWindow,
        //     player_animation,
        //     StateScoped(Screen::Playing),
        //     RigidBody::Dynamic,
        //     GravityScale(0.0),
        //     Collider::rectangle(20.0, 20.0),
        //     ColliderDensity(1.5),
        //     Friction::new(0.8),
        //    LockedAxes::ROTATION_LOCKED.lock_translation_y(),
        //     LinearDamping(0.8),
    });
}
