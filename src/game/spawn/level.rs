//! Spawn the main level by triggering other observers.

use avian2d::prelude::*;
use bevy::prelude::*;

use rand::Rng;

use super::{player::SpawnPlayer, GameState};

use crate::game::{
    animation::BasicAnimation,
    assets::{HandleMap, ImageKey},
};
use crate::screen::Screen;
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Counter(pub f32);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct SpawnControl(pub bool);

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level)
        .observe(spawn_npc)
        .observe(despawn_someone)
        .add_systems(Update, spawn_someone)
        .insert_state(GameState::Intro)
        .insert_resource(SpawnControl(false))
        .insert_resource(Counter(0.0))
        .add_plugins(
            // Add physics plugins and specify a units-per-meter scaling factor, 1 meter = 20 pixels.
            // The unit allows the engine to tune its parameters for the scale of the world, improving stability.
            PhysicsPlugins::default().with_length_unit(100.0),
        );
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
}

fn spawn_someone(
    mut counter: ResMut<Counter>,
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut spawn_control: ResMut<SpawnControl>,
) {
    match game_state.get() {
        GameState::Intro => {
            if counter.0 > 0.0 && !spawn_control.0 {
                commands.trigger(SpawnNPC);
                counter.0 = 0.0;
                spawn_control.0 = true;
            } else if counter.0 > 0.0 && spawn_control.0 {
                commands.trigger(DespawnSomeone);
                counter.0 = 0.0;
                spawn_control.0 = false;
                next_state.set(GameState::First);
            }
        }
        GameState::First => {
            if counter.0 > 0.0 && !spawn_control.0 {
                commands.trigger(SpawnNPC);
                commands.trigger(SpawnNPC);
                counter.0 = 0.0;
                spawn_control.0 = true;
            }
        } // GameState::Second => todo!(),
          // GameState::Third => todo!(),
          // GameState::Ending => todo!(),
    }
}
#[derive(Event, Debug)]
pub struct DespawnSomeone;

fn despawn_someone(
    _trigger: Trigger<DespawnSomeone>,
    mut commands: Commands,
    query: Query<Entity, With<Npc>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Event, Debug)]
pub struct SpawnNPC;
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Npc;

fn spawn_npc(
    _trigger: Trigger<SpawnNPC>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let npc_animation = BasicAnimation::new();

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..500.0); // Adjust the range as needed
    let y = rng.gen_range(0.0..500.0); // Adjust the range as needed
    let translation = Vec3::new(x, y, 0.0);

    commands.spawn((
        Name::new("NPC"),
        Npc,
        SpriteBundle {
            texture: image_handles[&ImageKey::Npc].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(2.0).extend(1.0))
                .with_translation(translation),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: npc_animation.get_atlas_index(),
        },
        npc_animation,
        StateScoped(Screen::Playing),
        RigidBody::Dynamic,
        Collider::rectangle(20.0, 20.0),
        GravityScale(0.0),
        Friction::new(0.7),
        {
            let locked_axes = LockedAxes::ROTATION_LOCKED;
            locked_axes.lock_translation_y();
            locked_axes
        },
    ));
}
