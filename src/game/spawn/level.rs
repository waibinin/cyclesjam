//! Spawn the main level by triggering other observers.

use avian2d::{math::*, prelude::*};

use bevy::prelude::*;
use std::collections::HashSet;

use crate::screen::Screen;
use crate::ui::prelude::*;

use crate::ui::widgets::VoiceComponent;
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

#[derive(Resource, Default)]
struct TextVoice {
    pub text: String,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Counter(pub f32);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct SpawnControl(pub bool);

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level)
        .insert_resource(TextVoice::default())
        .insert_resource(TextBubbleEntity(Entity::from_raw(0))) // Initialize with a dummy entity
        .observe(spawn_item)
        .observe(despawn_everyone)
        .observe(spawn_popup)
        .observe(destroy_joints)
        .add_systems(
            Update,
            (spawn_logic, create_distance_joint_system, update_voice_text),
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

    let x2 = rng.gen_range(-800.0..800.0); // Adjust the range as needed
    let y2 = rng.gen_range(-800.0..800.0); // Adjust the range as needed
    let translation2 = Vec3::new(x2, y2, 0.0);
    commands.spawn((
        Name::new("Hair"),
        Item,
        SpriteBundle {
            texture: image_handles[&ImageKey::Elements2].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(4.0).extend(1.0))
                .with_translation(translation2),
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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct FacePopUp;

#[derive(Event, Debug)]
pub struct SpawnPopUp;
#[derive(Resource)]
struct TextBubbleEntity(Entity);

fn spawn_popup(
    _trigger: Trigger<SpawnPopUp>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    font_handles: Res<HandleMap<FontKey>>,
    mut text_voice: ResMut<TextVoice>,
    mut text_bubble_entity: ResMut<TextBubbleEntity>,
) {
    // Create a texture atlas
    let layout =
        TextureAtlasLayout::from_grid(UVec2 { x: 240, y: 180 }, 3, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let translation = Vec3::new(0.0, 0.0, 1.0);
    let popup_animation = BasicAnimation::new(3);

    //let popup_entity =
    commands.spawn((
        Name::new("PopUp"),
        FacePopUp,
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
    ));
    //.id();

    // Spawn the UI root and dialogue bubble
    let bubble_entity = commands
        .ui_root()
        .with_children(|parent| {
            // Store the bubble text entity in the resource
            text_voice.text = "Hello, dreamer, again.".to_string();
            parent.dialogue_bubble(text_voice.text.clone(), &font_handles);
        })
        .id();

    // Store the bubble entity in the resource
    text_bubble_entity.0 = bubble_entity;
}

fn update_voice_text(
    mut query: Query<&mut Text, With<VoiceComponent>>,
    text_voice: Res<TextVoice>,
) {
    for mut text in query.iter_mut() {
        //println!("Updating text to: {}", text_voice.text);
        if !text.sections.is_empty() {
            text.sections[0].value = text_voice.text.clone();
        }
    }
}
