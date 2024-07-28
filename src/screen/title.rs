//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;

use crate::ui::prelude::*;

use crate::game::{
    animation::BasicAnimation,
    assets::{FontKey, HandleMap, ImageKey},
};
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);
    app.observe(make_title_animation);
    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands, font_handles: Res<HandleMap<FontKey>>) {
    commands.trigger(MakeTitleAnimation);
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children
                .button("Play", &font_handles)
                .insert(TitleAction::Play);
            children
                .button("Credits", &font_handles)
                .insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children
                .button("Exit", &font_handles)
                .insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct TitleAnimation;

#[derive(Event, Debug)]
pub struct MakeTitleAnimation;
fn make_title_animation(
    _trigger: Trigger<MakeTitleAnimation>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create a texture atlas
    let layout =
        TextureAtlasLayout::from_grid(UVec2 { x: 480, y: 270 }, 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let translation = Vec3::new(0.0, 0.0, 0.0);
    let title_animation = BasicAnimation::new(2);

    commands.spawn((
        Name::new("Title Animation"),
        TitleAnimation,
        SpriteBundle {
            texture: image_handles[&ImageKey::TitleImage].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(2.8).extend(1.0))
                .with_translation(translation),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: title_animation.get_atlas_index(),
        },
        StateScoped(Screen::Title),
        title_animation,
    ));
}
