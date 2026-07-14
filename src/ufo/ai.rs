use avian3d::prelude::*;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn ufo_chase(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Ufo>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Ufo>)>,
    asteroid_query: Query<&Transform, (With<Asteroid>, Without<Ufo>)>,
    mut ufo_query: Query<
        (Entity, &mut Transform, &mut Speed, &mut FireCooldown, &mut PatrolAngle, &Health, Option<&mut UfoHitRecover>),
        With<Ufo>,
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(player_transform) = player_query.iter().next() else {
        return;
    };

    let player_pos = player_transform.translation;
    let enemy_pos = if let Some(enemy_tf) = enemy_query.iter().next() {
        enemy_tf.translation
    } else {
        Vec3::new(0.0, 13.0, ARENA_CENTER)
    };

    let dt = time.delta_secs();

    for (ufo_entity, mut transform, mut speed, mut cooldown, mut patrol_angle, health, hit_recover) in &mut ufo_query {
        if health.0 <= 0.0 {
            continue;
        }

        if let Some(mut recover) = hit_recover {
            recover.0.tick(time.delta());
            transform.scale = Vec3::splat(0.8);
            if recover.0.just_finished() {
                transform.scale = Vec3::splat(1.0);
                commands.entity(ufo_entity).remove::<UfoHitRecover>();
            }
            continue;
        }

        let ufo_pos = transform.translation;

        patrol_angle.0 += UFO_ORBIT_SPEED * dt;
        let t = patrol_angle.0;

        let orbit_center = enemy_pos;
        let orbit_target = orbit_center + Vec3::new(
            UFO_ORBIT_RADIUS * t.cos(),
            5.0 * (t * 0.5).sin(),
            UFO_ORBIT_RADIUS * t.sin(),
        );

        let to_player = player_pos - ufo_pos;
        let dist_to_player = to_player.length();

        let facing_player = transform.forward().as_vec3().dot(to_player.normalize_or_zero());
        let fleeing = dist_to_player < UFO_FLEE_RANGE || (dist_to_player < UFO_SAFE_DISTANCE && facing_player < -0.1);

        let mut desired_dir = if fleeing {
            -to_player.normalize_or_zero()
        } else if dist_to_player < UFO_CHASE_RANGE {
            to_player.normalize_or_zero()
        } else {
            (orbit_target - ufo_pos).normalize_or_zero()
        };

        if !fleeing && dist_to_player < UFO_MIN_PLAYER_DISTANCE && dist_to_player > 0.1 {
            let away_from_player = -to_player.normalize();
            let blend = 1.0 - (dist_to_player / UFO_MIN_PLAYER_DISTANCE);
            desired_dir = (desired_dir + away_from_player * blend * 2.0).normalize_or_zero();

            let side_bias = if (patrol_angle.0 * 10.0).sin() >= 0.0 {
                1.0
            } else {
                -1.0
            };
            let lateral = Vec3::new(side_bias * 0.8, 0.0, 0.0);
            desired_dir = (desired_dir + lateral).normalize_or_zero();
        }

        for ast_tf in &asteroid_query {
            let to_asteroid = ast_tf.translation - ufo_pos;
            let dist_to_asteroid = to_asteroid.length();
            if dist_to_asteroid < UFO_DODGE_DISTANCE && dist_to_asteroid > 0.1 {
                let away = -to_asteroid.normalize();
                let urgency = 1.0 - (dist_to_asteroid / UFO_DODGE_DISTANCE);
                desired_dir = (desired_dir + away * urgency * UFO_DODGE_FORCE * dt).normalize_or_zero();
            }
        }

        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, desired_dir);
        transform.rotation = transform.rotation.slerp(target_rotation, UFO_TURN_SPEED * dt);

        let target_speed = if fleeing {
            UFO_FLEE_SPEED
        } else if dist_to_player < UFO_STOP_DISTANCE {
            UFO_SPEED * 0.55
        } else {
            UFO_SPEED
        };

        let speed_change_rate = 40.0;
        if speed.0 < target_speed {
            speed.0 = (speed.0 + speed_change_rate * dt).min(target_speed);
        } else if speed.0 > target_speed {
            speed.0 = (speed.0 - speed_change_rate * dt).max(target_speed);
        }

        if speed.0 > 0.01 {
            let forward = transform.forward().as_vec3();
            transform.translation += forward * speed.0 * dt;
        }

        if cooldown.0 <= 0.0 && dist_to_player < UFO_SHOOT_RANGE {
            let spawn_pos = transform.translation;
            let laser_dir = to_player.normalize();

            let laser_mesh = meshes.add(Sphere::new(0.35));
            let laser_material = materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.2, 1.0),
                emissive: LinearRgba::new(6.0, 0.4, 8.0, 1.0),
                ..default()
            });

            commands.spawn((
                Mesh3d(laser_mesh),
                MeshMaterial3d(laser_material),
                Transform::from_translation(spawn_pos),
                UfoLaser,
                Velocity(laser_dir * LASER_SPEED),
                DespawnAfter(LASER_LIFETIME),
                RigidBody::Kinematic,
                Collider::sphere(0.35),
                CollisionEventsEnabled,
            ));

            cooldown.0 = UFO_FIRE_RATE;
        }

        if cooldown.0 > 0.0 {
            cooldown.0 -= dt;
        }
    }
}
