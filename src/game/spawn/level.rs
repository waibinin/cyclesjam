//! Spawn the main level by triggering other observers.

//use avian2d::prelude::*;
use avian2d::{math::*, prelude::*};

use bevy::prelude::*;
use std::collections::HashSet;

use crate::ui::prelude::*;
use rand::Rng;

use super::{
    npc::{Npc, SpawnNPC},
    player::SpawnPlayer,
    GameState,
};

use crate::game::{
    animation::BasicAnimation,
    assets::{FontKey, HandleMap, ImageKey},
};

// use crate::ui::prelude::*;

// use crate::screen::Screen;
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Counter(pub f32);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct SpawnControl(pub bool);

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level)
        // .observe(spawn_npc)
        .observe(spawn_item)
        .observe(despawn_someone)
        .observe(spawn_popup)
        // .observe(destroy_joints)
        .add_systems(Update, (spawn_logic, create_distance_joint_system))
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
    for _ in 0..30 {
        commands.trigger(SpawnItem);
    }
}

fn spawn_logic(
    mut counter: ResMut<Counter>,
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut spawn_control: ResMut<SpawnControl>,
) {
    match game_state.get() {
        GameState::Intro => {
            if counter.0 > 0.0 && !spawn_control.0 {
                commands.trigger(DespawnSomeone);
                // commands.trigger(DestroyJoints);
                commands.trigger(SpawnNPC);
                for _ in 0..30 {
                    commands.trigger(SpawnItem);
                    commands.trigger(SpawnPopUp);
                }
                counter.0 = 0.0;
                spawn_control.0 = true;
            } else if counter.0 > 0.0 && spawn_control.0 {
                commands.trigger(DespawnSomeone);
                // commands.trigger(DestroyJoints);

                for _ in 0..20 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = false;
                next_state.set(GameState::First);
            }
        }
        GameState::First => {
            if counter.0 > 0.0 && !spawn_control.0 {
                commands.trigger(DespawnSomeone);
                // commands.trigger(DestroyJoints);
                for _ in 1..15 {
                    commands.trigger(SpawnNPC);
                }
                for _ in 0..30 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = true;
            } else if counter.0 > 0.0 && spawn_control.0 {
                commands.trigger(DespawnSomeone);
                // commands.trigger(DestroyJoints);
                for _ in 0..20 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = false;
                next_state.set(GameState::Second);
            }
        }

        GameState::Second => {
            if counter.0 > 0.0 && !spawn_control.0 {
                commands.trigger(DespawnSomeone);
                // commands.trigger(DestroyJoints);
                for _ in 1..100 {
                    commands.trigger(SpawnNPC);
                }
                for _ in 0..30 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = true;
            }
        } // GameState::Third => todo!(),
          // GameState::Ending => todo!(),
    }
}
#[derive(Event, Debug)]
pub struct DespawnSomeone;

fn despawn_someone(
    _trigger: Trigger<DespawnSomeone>,
    mut commands: Commands,
    query: Query<Entity, Or<(With<Npc>, With<Item>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Item;

#[derive(Event, Debug)]
pub struct SpawnItem;

fn spawn_item(
    _trigger: Trigger<SpawnItem>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create a texture atlas
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 3, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-500.0..500.0); // Adjust the range as needed
    let y = rng.gen_range(-500.0..500.0); // Adjust the range as needed
    let translation = Vec3::new(x, y, 0.0);
    //let sprite_index = rng.gen_range(0..9); // Random index from 0 to 8

    commands.spawn((
        Name::new("Item"),
        Item,
        SpriteBundle {
            texture: image_handles[&ImageKey::Elements].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(4.0).extend(1.0))
                .with_translation(translation),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: rng.gen_range(0..9),
        },
    ));
}

// A system that creates a distance joint between colliding entities
fn create_distance_joint_system(
    mut commands: Commands,
    query: Query<(Entity, &CollidingEntities, &Transform), With<Npc>>,
    transform_query: Query<(Entity, &Transform), With<Npc>>,
    mut existing_joints: Local<HashSet<Entity>>, // Track existing joints
) {
    for (entity1, colliding_entities, _transform1) in query.iter() {
        if existing_joints.contains(&entity1) {
            continue;
        }
        for &entity2 in colliding_entities.iter() {
            if existing_joints.contains(&entity2) {
                // Skip if entity2 is already part of a joint
                continue;
            }
            if let Ok(_transform2) = transform_query.get(entity2) {
                commands.spawn(
                    DistanceJoint::new(entity1, entity2)
                        .with_local_anchor_1(Vector::ZERO)
                        .with_local_anchor_2(Vector::ZERO)
                        .with_rest_length(200.0)
                        .with_linear_velocity_damping(0.0)
                        .with_angular_velocity_damping(0.0)
                        .with_compliance(0.00000001),
                );
                // Add both entities to the set of existing joints
                existing_joints.insert(entity1);
                existing_joints.insert(entity2);
            }
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct FacePopUp;

#[derive(Event, Debug)]
pub struct SpawnPopUp;

fn spawn_popup(
    _trigger: Trigger<SpawnPopUp>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    font_handles: Res<HandleMap<FontKey>>,
) {
    // Create a texture atlas
    let layout =
        TextureAtlasLayout::from_grid(UVec2 { x: 240, y: 180 }, 3, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let translation = Vec3::new(0.0, 0.0, 2.0);
    let popup_animation = BasicAnimation::new(3);

    let popup_entity = commands
        .spawn((
            Name::new("PopUp"),
            Item,
            SpriteBundle {
                texture: image_handles[&ImageKey::PopUp].clone_weak(),
                transform: Transform::from_scale(Vec2::splat(4.0).extend(1.0))
                    .with_translation(translation),
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: popup_animation.get_atlas_index(),
            },
            popup_animation,
        ))
        .id();

    // Spawn the UI root and dialogue bubble
    let _bubble_entity = commands
        .ui_root()
        .with_children(|parent| {
            parent.dialogue_bubble("Hello, Player!", &font_handles);
            println!(
                "Added dialogue bubble to NPC with entity ID: {:?}",
                popup_entity
            );
        })
        .id();
}

// #[derive(Event, Debug)]
// pub struct DestroyJoints;

// fn destroy_joints(
//     _trigger: Trigger<DestroyJoints>,
//     mut commands: Commands,
//     query: Query<(Entity, &DistanceJoint)>,
// ) {
//     for (entity, distance_joint) in query.iter() {
//         commands.entity(entity).despawn();
//         println!("Entity: {:?}, DistanceJoint: {:?}", entity, distance_joint);
//     }
// }
