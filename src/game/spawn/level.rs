//! Spawn the main level by triggering other observers.

//use avian2d::prelude::*;
use avian2d::{math::*, prelude::*};

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
        .observe(spawn_item)
        .observe(despawn_someone)
        // .observe(destroy_joints)
        .add_systems(Update, (spawn_someone, create_distance_joint_system))
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
                commands.trigger(DespawnSomeone);
                // commands.trigger(DestroyJoints);
                commands.trigger(SpawnNPC);
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
                next_state.set(GameState::First);
            }
        }
        GameState::First => {
            if counter.0 > 0.0 && !spawn_control.0 {
                commands.trigger(DespawnSomeone);
                // commands.trigger(DestroyJoints);
                for _ in 1..3 {
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
                for _ in 1..10 {
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
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let npc_animation = BasicAnimation::new();

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-400.0..400.0); // Adjust the range as needed
    let y = rng.gen_range(-400.0..400.0); // Adjust the range as needed
    let translation = Vec3::new(x, y, 1.0);

    // Generate a random number between 1 and 3
    let npc_index = rng.gen_range(1..=5);
    println!("Generated NPC Index: {}", npc_index); // Debugging line

    // Determine the correct texture handle based on the random number
    let npc_texture = match npc_index {
        1 => image_handles[&ImageKey::Npc1].clone_weak(),
        2 => image_handles[&ImageKey::Npc2].clone_weak(),
        3 => image_handles[&ImageKey::Npc3].clone_weak(),
        4 => image_handles[&ImageKey::Npc4].clone_weak(),
        5 => image_handles[&ImageKey::Npc5].clone_weak(),
        _ => image_handles[&ImageKey::Npc1].clone_weak(), // Fallback
    };

    commands.spawn((
        Name::new("NPC"),
        Npc,
        SpriteBundle {
            texture: npc_texture.clone_weak(),
            transform: Transform::from_scale(Vec2::splat(4.0).extend(1.0))
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
        LinearDamping(0.8),
        {
            let locked_axes = LockedAxes::ROTATION_LOCKED;
            locked_axes.lock_translation_y();
            locked_axes
        },
    ));
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

// // A system that creates a distance joint between colliding entities
// // A system that creates a distance joint between colliding entities
fn create_distance_joint_system(
    mut commands: Commands,
    query: Query<(Entity, &CollidingEntities, &Transform), With<Npc>>,
    transform_query: Query<(Entity, &Transform), With<Npc>>,
) {
    for (entity1, colliding_entities, _transform1) in query.iter() {
        for &entity2 in colliding_entities.iter() {
            if let Ok(_transform2) = transform_query.get(entity2) {
                commands.spawn(
                    DistanceJoint::new(entity1, entity2)
                        .with_local_anchor_1(Vector::ZERO)
                        .with_local_anchor_2(Vector::ZERO)
                        .with_rest_length(100.0)
                        .with_linear_velocity_damping(0.0)
                        .with_angular_velocity_damping(0.0)
                        .with_compliance(0.00000001),
                );
            }
        }
    }
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
