use bevy::prelude::*;
use rand::Rng;

use crate::game::assets::{HandleMap, ImageKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_item).register_type::<Item>();
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
