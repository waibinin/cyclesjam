// use avian2d::prelude::*;
use bevy::prelude::*;

// use crate::screen::Screen;
// use rand::Rng;

use crate::game::{
    animation::BasicAnimation,
    assets::{FontKey, HandleMap, ImageKey},
};

use crate::ui::prelude::*;

use crate::ui::widgets::VoiceComponent;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_popup)
        //.register_type::<Popup>()
        .insert_resource(TextVoice::default())
        .insert_resource(TextBubbleEntity(Entity::from_raw(0))) // Initialize with a dummy entity
        .add_systems(Update, update_voice_text);
}

#[derive(Resource, Default)]
pub struct TextVoice {
    pub text: String,
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
