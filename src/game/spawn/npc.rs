use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screen::Screen;
use rand::Rng;

use crate::game::{
    animation::BasicAnimation,
    assets::{HandleMap, ImageKey},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_npc).register_type::<Npc>();
}

#[derive(Event, Debug)]
pub struct SpawnNPC;
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Npc;
#[derive(Component, Default)]
struct DialogueBubble;

fn spawn_npc(
    _trigger: Trigger<SpawnNPC>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let npc_animation = BasicAnimation::new(2);

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
