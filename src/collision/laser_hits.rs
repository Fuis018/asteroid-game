use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::particles;
use crate::GameState;

pub fn player_laser_hits_asteroid(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    laser_query: Query<&PlayerLaser>,
    mut asteroid_query: Query<(&Asteroid, &mut AsteroidHealth, Option<&AsteroidFragment>, &mut Transform)>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.collider1, event.collider2);
        let (laser, asteroid_body) = if laser_query.get(a).is_ok() {
            (a, event.body2)
        } else if laser_query.get(b).is_ok() {
            (b, event.body1)
        } else {
            continue;
        };

        let mut hit = false;
        if let Some(body) = asteroid_body {
            if let Ok((_, mut health, fragment, mut ast_transform)) = asteroid_query.get_mut(body) {
                health.0 -= PLAYER_LASER_DAMAGE;
                if fragment.is_none() {
                    let current = ast_transform.scale.x;
                    let new_scale = current * 0.92;
                    ast_transform.scale = Vec3::splat(new_scale);
                }
                hit = true;
            }
        }
        if hit {
            commands.entity(laser).despawn();
        }
    }
}

pub fn player_laser_hits_enemy(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    laser_query: Query<(&PlayerLaser, &Transform)>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.collider1, event.collider2);
        if let (Ok((_, lt)), Ok(mut health)) = (laser_query.get(a), enemy_query.get_mut(b)) {
            health.0 -= PLAYER_LASER_DAMAGE;
            particles::spawn_explosion_at(&mut commands, &mut meshes, &mut materials, lt.translation, Color::srgb(1.0, 0.6, 0.1), 0.4);
            commands.entity(a).despawn();
        } else if let (Ok((_, lt)), Ok(mut health)) = (laser_query.get(b), enemy_query.get_mut(a)) {
            health.0 -= PLAYER_LASER_DAMAGE;
            particles::spawn_explosion_at(&mut commands, &mut meshes, &mut materials, lt.translation, Color::srgb(1.0, 0.6, 0.1), 0.4);
            commands.entity(b).despawn();
        }
    }
}

pub fn player_laser_hits_ufo(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    laser_query: Query<&Transform, With<PlayerLaser>>,
    mut ufo_query: Query<(&Transform, &mut Health), With<Ufo>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.collider1, event.collider2);
        if let (Ok(lt), Ok((_, mut health))) = (laser_query.get(a), ufo_query.get_mut(b)) {
            health.0 -= PLAYER_LASER_DAMAGE;
            particles::spawn_explosion_at(&mut commands, &mut meshes, &mut materials, lt.translation, Color::srgb(0.4, 0.8, 1.0), 0.5);
            commands.entity(a).despawn();
        } else if let (Ok(lt), Ok((_, mut health))) = (laser_query.get(b), ufo_query.get_mut(a)) {
            health.0 -= PLAYER_LASER_DAMAGE;
            particles::spawn_explosion_at(&mut commands, &mut meshes, &mut materials, lt.translation, Color::srgb(0.4, 0.8, 1.0), 0.5);
            commands.entity(b).despawn();
        }
    }
}

pub fn enemy_laser_hits_player(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    laser_query: Query<&EnemyLaser>,
    mut player_query: Query<(&mut Health, Entity), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.collider1, event.collider2);
        if let (Ok(_), Ok((mut health, _))) = (laser_query.get(a), player_query.get_mut(b)) {
            health.0 -= ENEMY_LASER_DAMAGE;
            commands.entity(a).despawn();
            if health.0 <= 0.0 {
                next_state.set(GameState::Defeat);
            }
        } else if let (Ok(_), Ok((mut health, _))) = (laser_query.get(b), player_query.get_mut(a)) {
            health.0 -= ENEMY_LASER_DAMAGE;
            commands.entity(b).despawn();
            if health.0 <= 0.0 {
                next_state.set(GameState::Defeat);
            }
        }
    }
}

pub fn ufo_laser_hits_player(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    laser_query: Query<&UfoLaser>,
    mut player_query: Query<(&mut Health, Entity), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.collider1, event.collider2);
        if let (Ok(_), Ok((mut health, _))) = (laser_query.get(a), player_query.get_mut(b)) {
            health.0 -= UFO_LASER_DAMAGE;
            commands.entity(a).despawn();
            if health.0 <= 0.0 {
                next_state.set(GameState::Defeat);
            }
        } else if let (Ok(_), Ok((mut health, _))) = (laser_query.get(b), player_query.get_mut(a)) {
            health.0 -= UFO_LASER_DAMAGE;
            commands.entity(b).despawn();
            if health.0 <= 0.0 {
                next_state.set(GameState::Defeat);
            }
        }
    }
}
