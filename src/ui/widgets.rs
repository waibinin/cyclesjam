//! Helper traits for creating common widgets.

// use bevy::ecs::system::SystemParam;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

use super::{interaction::InteractionPalette, palette::*};

use crate::game::assets::{FontKey, HandleMap};

// #[derive(SystemParam)]
// pub struct WidgetParams<'w, 's> {
//     commands: Commands<'w, 's>,
//     font_handles: Res<'w, HandleMap<FontKey>>,
// }

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(
        &mut self,
        text: impl Into<String>,
        font_handles: &Res<HandleMap<FontKey>>,
    ) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(
        &mut self,
        text: impl Into<String>,
        font_handles: &Res<HandleMap<FontKey>>,
    ) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(
        &mut self,
        text: impl Into<String>,
        font_handles: &Res<HandleMap<FontKey>>,
    ) -> EntityCommands;
    /// Spawn a dialogue bubble above an NPC.
    fn dialogue_bubble(
        &mut self,
        text: impl Into<String>,
        font_handles: &Res<HandleMap<FontKey>>,
    ) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    //impl<'w, 's> Widgets for WidgetParams<'w, 's> {
    fn button(
        &mut self,
        text: impl Into<String>,
        font_handles: &Res<HandleMap<FontKey>>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(200.0),
                    height: Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font_handles[&FontKey::UiFont].clone_weak(),
                        font_size: 40.0,
                        color: BUTTON_TEXT,
                    },
                ),
            ));
        });
        entity
    }

    fn header(
        &mut self,
        text: impl Into<String>,
        font_handles: &Res<HandleMap<FontKey>>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Header"),
            NodeBundle {
                style: Style {
                    width: Px(500.0),
                    height: Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Header Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font_handles[&FontKey::UiFont].clone_weak(),
                        font_size: 40.0,
                        color: HEADER_TEXT,
                    },
                ),
            ));
        });
        entity
    }

    fn label(
        &mut self,
        text: impl Into<String>,
        font_handles: &Res<HandleMap<FontKey>>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            NodeBundle {
                style: Style {
                    width: Px(500.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Label Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font_handles[&FontKey::UiFont].clone_weak(),
                        font_size: 24.0,
                        color: LABEL_TEXT,
                    },
                ),
            ));
        });
        entity
    }
    fn dialogue_bubble(
        &mut self,
        text: impl Into<String>,
        font_handles: &Res<HandleMap<FontKey>>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Bubble"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(50.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                transform: Transform::from_xyz(0.0, 50.0, 0.0), // Adjust Y offset
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Bubble Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font_handles[&FontKey::UiFont].clone_weak(),
                        font_size: 48.0,

                        color: LABEL_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.0),
                    padding: UiRect::px(10.0, 10.0, 10.0, 10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
