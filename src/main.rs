use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod assets;
mod asteroids;
mod camera;
mod collision;
mod components;
mod constants;
mod debug;
mod enemy;
mod laser;
mod materials;
mod player;
mod particles;
mod starfield;
mod ui;
mod ufo;

use assets::GameAssets;
use components::*;
use GameState::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Restarting,
    Victory,
    Defeat,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Asteroid Shooter".into(),
                    resolution: (1280u32, 720u32).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(avian3d::PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.05)))
        .insert_resource(debug::DebugColliders(false))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::MainMenu)
                .load_collection::<GameAssets>(),
        )
        .add_systems(OnEnter(GameState::MainMenu), ui::spawn_menu)
        .add_systems(OnExit(GameState::MainMenu), ui::cleanup_menu)
        .add_systems(
            Update,
            ui::menu_interaction.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Startup,
            (
                camera::spawn_camera,
                starfield::spawn_starfield,
            ),
        )
        .add_systems(
            OnEnter(Playing),
            (
                ui::spawn_hud,
                player::spawn_player,
                enemy::spawn_enemy,
                ufo::spawn_ufo,
                asteroids::spawn_asteroids,
                asteroids::spawn_initial_fragments,
            ),
        )
        .add_systems(
            Update,
            (
                player::player_movement,
                player::player_shooting,
                enemy::enemy_patrol,
                ufo::ufo_chase,
                asteroids::move_asteroids,
                asteroids::respawn_asteroids,
                laser::move_lasers,
                laser::despawn_lasers,
                asteroids::split_asteroids,
            )
                .chain()
                .run_if(in_state(Playing)),
        )
        .add_systems(
            Update,
            (
                collision::player_laser_hits_asteroid,
                collision::player_laser_hits_enemy,
                collision::player_laser_hits_ufo,
                collision::enemy_laser_hits_player,
                enemy::check_enemy_health_flags,
                collision::ufo_laser_hits_player,
                collision::player_hits_asteroid,
                collision::player_hits_ufo,
                collision::ufo_hits_asteroid,
                collision::asteroid_hits_asteroid,
                collision::check_ufo_destroyed,
                collision::check_victory,
                camera::camera_follow_player,
                ui::update_hud,
            )
                .chain()
                .run_if(in_state(Playing)),
        )
        .add_systems(
            Update,
            (
                debug::draw_global_axes,
                debug::draw_local_axes,
                debug::toggle_debug_colliders,
                debug::draw_colliders,
                materials::apply_player_materials,
                particles::update_particles,
            )
                .run_if(in_state(Playing)),
        )
        .add_systems(OnEnter(Victory), ui::show_victory)
        .add_systems(OnEnter(Defeat), ui::show_game_over)
        .add_systems(
            Update,
            restart_game.run_if(in_state(Victory).or_else(in_state(Defeat))),
        )
        .add_systems(OnEnter(Restarting), restart_after_despawn)
        .add_systems(OnExit(Victory), ui::cleanup_hud)
        .add_systems(OnExit(Defeat), ui::cleanup_hud)
        .run();
}

fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    entities: Query<Entity, Or<(
        With<Player>,
        With<Enemy>,
        With<Asteroid>,
        With<PlayerLaser>,
        With<EnemyLaser>,
        With<Ufo>,
        With<UfoLaser>,
        With<Turret>,
        With<UfoTurret>,
        With<ui::HealthBarText>,
        With<ui::EnemyHealthBarText>,
        With<ui::UfoHealthBarText>,
        With<ui::ControlsText>,
        With<ui::GameOverText>,
        With<ui::VictoryText>,
    )>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        for entity in &entities {
            commands.entity(entity).despawn();
        }
        next_state.set(Restarting);
    }
}

fn restart_after_despawn(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(Playing);
}
