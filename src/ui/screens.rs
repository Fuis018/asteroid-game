use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct VictoryText;

pub fn show_game_over(mut commands: Commands) {
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
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        GameOverText,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font_size: FontSize::Px(72.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.2, 0.2)),
        ));
        parent.spawn((
            Text::new("Presiona R para reiniciar"),
            TextFont {
                font_size: FontSize::Px(28.0),
                ..default()
            },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
        ));
    });
}

pub fn show_victory(mut commands: Commands) {
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
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        VictoryText,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("VICTORY!"),
            TextFont {
                font_size: FontSize::Px(72.0),
                ..default()
            },
            TextColor(Color::srgb(0.2, 1.0, 0.2)),
        ));
        parent.spawn((
            Text::new("Presiona R para reiniciar"),
            TextFont {
                font_size: FontSize::Px(28.0),
                ..default()
            },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
        ));
        parent.spawn((
            Text::new("Main Developer: Luis Román"),
            TextFont {
                font_size: FontSize::Px(20.0),
                ..default()
            },
            TextColor(Color::srgba(0.7, 0.7, 0.7, 0.6)),
            Node {
                margin: UiRect::top(Val::Px(40.0)),
                ..default()
            },
        ));
    });
}

pub fn cleanup_hud(
    mut commands: Commands,
    query: Query<Entity, Or<(With<super::hud::HealthBarText>, With<super::hud::EnemyHealthBarText>, With<super::hud::UfoHealthBarText>, With<super::hud::SpeedText>, With<super::hud::ControlsText>, With<GameOverText>, With<VictoryText>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
