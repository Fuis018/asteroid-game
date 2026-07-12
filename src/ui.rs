use bevy::prelude::*;

use crate::components::{Enemy, Health, Player, Speed, Ufo};

#[derive(Component)]
pub struct HealthBarText;

#[derive(Component)]
pub struct EnemyHealthBarText;

#[derive(Component)]
pub struct UfoHealthBarText;

#[derive(Component)]
pub struct SpeedText;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct VictoryText;

#[derive(Component)]
pub struct ControlsText;

pub fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Health: 300"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgba(0.0, 1.0, 0.0, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        HealthBarText,
    ));

    commands.spawn((
        Text::new("Enemy: 1500"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgba(1.0, 0.0, 0.0, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        },
        EnemyHealthBarText,
    ));

    commands.spawn((
        Text::new("UFOs: 1"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgba(0.8, 0.2, 1.0, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(10.0),
            ..default()
        },
        UfoHealthBarText,
    ));

    commands.spawn((
        Text::new("Speed: 0 / 30"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgba(0.5, 0.8, 1.0, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(130.0),
            left: Val::Px(10.0),
            ..default()
        },
        SpeedText,
    ));

    commands.spawn((
        Text::new("WASD: Rotate | Shift: Thrust | Ctrl: Brake | Q/E: Roll | Space: Fire"),
        TextFont {
            font_size: FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ControlsText,
    ));
}

pub fn update_hud(
    player_query: Query<(&Health, &Speed), With<Player>>,
    enemy_query: Query<&Health, With<Enemy>>,
    ufo_query: Query<&Health, With<Ufo>>,
    mut health_text: Query<&mut Text, (With<HealthBarText>, Without<EnemyHealthBarText>, Without<UfoHealthBarText>, Without<SpeedText>)>,
    mut enemy_text: Query<&mut Text, (With<EnemyHealthBarText>, Without<HealthBarText>, Without<UfoHealthBarText>, Without<SpeedText>)>,
    mut ufo_text: Query<&mut Text, (With<UfoHealthBarText>, Without<HealthBarText>, Without<EnemyHealthBarText>, Without<SpeedText>)>,
    mut speed_text: Query<&mut Text, (With<SpeedText>, Without<HealthBarText>, Without<EnemyHealthBarText>, Without<UfoHealthBarText>)>,
) {
    for (health, speed) in &player_query {
        for mut text in &mut health_text {
            **text = format!("Health: {:.0}", health.0);
        }
        for mut text in &mut speed_text {
            **text = format!("Speed: {:.0} / {:.0}", speed.0, crate::constants::PLAYER_MAX_SPEED);
        }
    }
    if let Some(health) = enemy_query.iter().next() {
        for mut text in &mut enemy_text {
            **text = format!("Enemy: {:.0}", health.0);
        }
    } else {
        for mut text in &mut enemy_text {
            **text = "Enemy: DESTROYED".to_string();
        }
    }
    let ufo_count = ufo_query.iter().count();
    for mut text in &mut ufo_text {
        **text = format!("UFOs: {}", ufo_count);
    }
}

pub fn show_game_over(mut commands: Commands) {
    commands.spawn((
        Text::new("GAME OVER\nPress R to restart"),
        TextFont {
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        GameOverText,
    ));
}

pub fn show_victory(mut commands: Commands) {
    commands.spawn((
        Text::new("VICTORY!\nPress R to restart"),
        TextFont {
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        VictoryText,
    ));
}

pub fn cleanup_hud(
    mut commands: Commands,
    query: Query<Entity, Or<(With<HealthBarText>, With<EnemyHealthBarText>, With<UfoHealthBarText>, With<SpeedText>, With<ControlsText>, With<GameOverText>, With<VictoryText>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
