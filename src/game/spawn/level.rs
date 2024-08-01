//! Spawn the main level by triggering other observers.

use avian2d::{math::*, prelude::*};

use bevy::prelude::*;
use std::collections::HashSet;

use crate::screen::Screen;
// use crate::ui::prelude::*;

// use crate::ui::widgets::VoiceComponent;
//use rand::Rng;

use super::{
    bigface::{FacePopUp, SpawnPopUp, TextVoice},
    npc::{Npc, SpawnNPC},
    player::SpawnPlayer,
    tiles::{Item, SpawnItem},
    GameState,
};

// use crate::game::{
//     animation::BasicAnimation,
//     assets::{FontKey, HandleMap, ImageKey},
// };

// #[derive(Resource, Default)]
// struct TextVoice {
//     pub text: String,
// }

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Counter(pub f32);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct SpawnControl(pub bool);

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level)
        .observe(despawn_everyone)
        .observe(destroy_joints)
        .add_systems(
            Update,
            (spawn_logic, create_distance_joint_system), //, update_voice_text),
        )
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
    mut text_voice: ResMut<TextVoice>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    match game_state.get() {
        GameState::Intro => {
            if counter.0 > 1.0 && !spawn_control.0 {
                commands.trigger(DespawnEveryone);
                // commands.trigger(DestroyJoints);
                commands.trigger(SpawnPopUp);
                for _ in 0..30 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = true;
                text_voice.text = "You should help me find my way".to_string();
            } else if counter.0 > 1.0 && spawn_control.0 {
                commands.trigger(DespawnEveryone);
                // commands.trigger(DestroyJoints);
                text_voice.text = "Hey! Don't Leave".to_string();
                for _ in 0..20 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = false;
                next_state.set(GameState::First);
            }
        }
        GameState::First => {
            if counter.0 > 2.0 && !spawn_control.0 {
                commands.trigger(DespawnEveryone);
                text_voice.text = "You won't get far...".to_string();
                // commands.trigger(DestroyJoints);
                for _ in 1..15 {
                    commands.trigger(SpawnNPC);
                }
                for _ in 0..30 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = true;
            } else if counter.0 > 2.0 && spawn_control.0 {
                //commands.trigger(DespawnEveryone);
                text_voice.text = "You are trapped here.".to_string();
                commands.trigger(DespawnEveryone);
                for _ in 0..20 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = false;
                next_state.set(GameState::Second);
            }
        }

        GameState::Second => {
            if counter.0 > 1.0 && !spawn_control.0 {
                commands.trigger(DespawnEveryone);
                text_voice.text = "As do i".to_string();
                // commands.trigger(DestroyJoints);
                for _ in 1..100 {
                    commands.trigger(SpawnNPC);
                }
                for _ in 0..30 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = true;
            } else if counter.0 > 1.0 && spawn_control.0 {
                commands.trigger(DespawnEveryone);
                text_voice.text = "You can't escape.".to_string();
                //commands.trigger(DestroyJoints);
                for _ in 0..20 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = false;
                next_state.set(GameState::Third);
            }
        }
        GameState::Third => {
            if counter.0 > 0.0 && !spawn_control.0 {
                commands.trigger(DespawnEveryone);
                for _ in 1..200 {
                    commands.trigger(SpawnNPC);
                }
                commands.trigger(SpawnPopUp);
                for _ in 0..30 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = true;
            } else if counter.0 > 2.0 && spawn_control.0 {
                commands.trigger(DespawnEveryone);
                // commands.trigger(DestroyJoints);
                text_voice.text = "All the little spirits walk with you.".to_string();
                //commands.entity(Player).insert(Npc);

                for _ in 0..20 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = false;
                next_state.set(GameState::Ending);
            }
        }
        GameState::Ending => {
            if counter.0 > 0.0 && !spawn_control.0 {
                commands.trigger(DespawnEveryone);
                text_voice.text = "Until you wake up".to_string();
                // commands.trigger(DestroyJoints);
                for _ in 1..15 {
                    commands.trigger(SpawnNPC);
                }
                for _ in 0..30 {
                    commands.trigger(SpawnItem);
                }
                counter.0 = 0.0;
                spawn_control.0 = true;
            } else if counter.0 > 20.0 && spawn_control.0 {
                commands.trigger(DespawnEveryone);
                counter.0 = 0.0;
                spawn_control.0 = false;
                next_state.set(GameState::Intro);
                next_screen.set(Screen::Splash);
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct DespawnEveryone;

fn despawn_everyone(
    _trigger: Trigger<DespawnEveryone>,
    mut commands: Commands,
    query: Query<Entity, Or<(With<Npc>, With<Item>, With<FacePopUp>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// A system that creates a distance joint between colliding entities
fn create_distance_joint_system(
    mut commands: Commands,
    query: Query<(Entity, &CollidingEntities, &Transform)>, // With<Npc>>,
    transform_query: Query<(Entity, &Transform)>,           //, With<Npc>>,
    mut existing_joints: Local<HashSet<Entity>>,            // Track existing joints
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

#[derive(Event, Debug)]
pub struct DestroyJoints;

fn destroy_joints(
    _trigger: Trigger<DestroyJoints>,
    mut commands: Commands,
    query: Query<(Entity, &DistanceJoint)>,
) {
    for (entity, _distance_joint) in query.iter() {
        commands.entity(entity).despawn();
        //println!("Entity: {:?}, DistanceJoint: {:?}", entity, distance_joint);
    }
}
