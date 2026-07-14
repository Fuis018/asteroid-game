use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::particles;
use crate::GameState;

pub fn player_hits_asteroid(
    mut collision_events: MessageReader<CollisionStart>,
    player_query: Query<&Player>,
    asteroid_query: Query<&Asteroid>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.collider1, event.collider2);
        let player_side = if player_query.get(a).is_ok() {
            event.body2
        } else if player_query.get(b).is_ok() {
            event.body1
        } else {
            continue;
        };
        if let Some(body) = player_side {
            if asteroid_query.get(body).is_ok() {
                next_state.set(GameState::Defeat);
                return;
            }
        }
    }
}

pub fn player_hits_ufo(
    mut collision_events: MessageReader<CollisionStart>,
    player_query: Query<&Player>,
    ufo_query: Query<&Ufo>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.collider1, event.collider2);
        if player_query.get(a).is_ok() && ufo_query.get(b).is_ok() {
            next_state.set(GameState::Defeat);
            return;
        }
        if player_query.get(b).is_ok() && ufo_query.get(a).is_ok() {
            next_state.set(GameState::Defeat);
            return;
        }
    }
}

pub fn ufo_hits_asteroid(
    mut collision_events: MessageReader<CollisionStart>,
    ufo_query: Query<&Ufo>,
    asteroid_query: Query<&Asteroid>,
    mut ufo_health: Query<(Entity, &Transform, &mut Health, Option<&UfoHitRecover>), With<Ufo>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.collider1, event.collider2);
        let ufo_body = if ufo_query.get(a).is_ok() {
            event.body2
        } else if ufo_query.get(b).is_ok() {
            event.body1
        } else {
            continue;
        };

        if let Some(body) = ufo_body {
            if asteroid_query.get(body).is_ok() {
                if let Ok((entity, ufo_tf, mut health, recover)) = ufo_health.single_mut() {
                    health.0 -= UFO_ASTEROID_DAMAGE;
                    particles::spawn_explosion_at(&mut commands, &mut meshes, &mut materials, ufo_tf.translation, Color::srgb(1.0, 0.6, 0.1), 1.0);
                    if recover.is_none() {
                        commands.entity(entity).insert(UfoHitRecover(Timer::from_seconds(0.3, TimerMode::Once)));
                    }
                }
            }
        }
    }
}

pub fn asteroid_hits_asteroid(
    mut collision_events: MessageReader<CollisionStart>,
    asteroid_query: Query<&Asteroid>,
    mut health_query: Query<(&mut AsteroidHealth, &Transform, Option<&AsteroidFragment>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let mut hits: Vec<(Entity, Entity, Vec3)> = Vec::new();

    for event in collision_events.read() {
        let body_a = match event.body1 {
            Some(e) if asteroid_query.get(e).is_ok() => e,
            _ => continue,
        };
        let body_b = match event.body2 {
            Some(e) if asteroid_query.get(e).is_ok() => e,
            _ => continue,
        };

        if body_a == body_b {
            continue;
        }

        if let (Ok((_, tf_a, _)), Ok((_, tf_b, _))) =
            (health_query.get(body_a), health_query.get(body_b))
        {
            let midpoint = (tf_a.translation + tf_b.translation) * 0.5;
            hits.push((body_a, body_b, midpoint));
        }
    }

    for (body_a, body_b, midpoint) in hits {
        let color;
        let mut dmg_a = ASTEROID_COLLISION_DAMAGE;
        let mut dmg_b = ASTEROID_COLLISION_DAMAGE;

        if let Ok((hp_a, _, frag_a)) = health_query.get(body_a) {
            if let Ok((hp_b, _, frag_b)) = health_query.get(body_b) {
                color = if frag_a.is_some() || frag_b.is_some() {
                    Color::srgb(1.0, 0.5, 0.1)
                } else {
                    Color::srgb(1.0, 0.8, 0.2)
                };
                if hp_a.0 <= ASTEROID_COLLISION_DAMAGE {
                    dmg_a = hp_a.0;
                }
                if hp_b.0 <= ASTEROID_COLLISION_DAMAGE {
                    dmg_b = hp_b.0;
                }
            } else {
                continue;
            }
        } else {
            continue;
        }

        if let Ok((mut hp, _, _)) = health_query.get_mut(body_a) {
            hp.0 -= dmg_a;
        }
        if let Ok((mut hp, _, _)) = health_query.get_mut(body_b) {
            hp.0 -= dmg_b;
        }

        particles::spawn_explosion_at(&mut commands, &mut meshes, &mut materials, midpoint, color, 0.5);
    }
}
