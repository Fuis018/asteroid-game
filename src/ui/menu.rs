use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

pub fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.05, 1.0)),
        MainMenuRoot,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("ASTEROID SHOOTER"),
            TextFont {
                font_size: FontSize::Px(64.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
        ));

        parent.spawn((
            Text::new("JUGAR"),
            TextFont {
                font_size: FontSize::Px(36.0),
                ..default()
            },
            TextColor(Color::srgb(0.2, 1.0, 0.2)),
            Node {
                margin: UiRect::top(Val::Px(40.0)),
                padding: UiRect::axes(Val::Px(40.0), Val::Px(15.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.3, 0.1, 1.0)),
            Button,
            PlayButton,
        ));

        parent.spawn((
            Text::new("SALIR"),
            TextFont {
                font_size: FontSize::Px(36.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.3, 0.3)),
            Node {
                padding: UiRect::axes(Val::Px(40.0), Val::Px(15.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.3, 0.1, 0.1, 1.0)),
            Button,
            QuitButton,
        ));
    });
}

pub fn menu_interaction(
    interaction_query: Query<(&Interaction, Option<&PlayButton>, Option<&QuitButton>), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, play, quit) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if play.is_some() {
                next_state.set(GameState::Playing);
            }
            if quit.is_some() {
                exit.write(AppExit::Success);
            }
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
