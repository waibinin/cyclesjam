//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
//use rand;

use super::player::SpawnPlayer;

use crate::game::{
    animation::BasicAnimation , 
    assets::{HandleMap, ImageKey}};
use crate::Counter;
use crate::screen::Screen;

 

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level)
    .observe(spawn_npc)
    .add_systems(Update, spawn_someone);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands,) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
}

fn spawn_someone(mut counter:ResMut<Counter>,mut commands:Commands)
{
    if counter.0>0.0
    {
        commands.trigger(SpawnNPC);
        counter.0 =0.0;
    }
}


#[derive(Event, Debug)]
pub struct SpawnNPC;
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Npc;



fn spawn_npc(_trigger: Trigger<SpawnNPC>, mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let npc_animation = BasicAnimation::new();

    commands.spawn((
        Name::new("NPC"),
        Npc,
        SpriteBundle {
            texture: image_handles[&ImageKey::Npc].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(2.0).extend(1.0))
            .with_translation(Vec3::new(100.0, 100.0, 0.0)),
            ..Default::default()
        }
        ,
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: npc_animation.get_atlas_index(),
        },
        npc_animation,
        StateScoped(Screen::Playing)
    ));
}