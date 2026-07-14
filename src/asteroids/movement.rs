use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub fn move_asteroids(
    time: Res<Time>,
    mut big: Query<(Entity, &mut Transform, &Velocity, &Rotating, Option<&AsteroidRespawn>), (With<Asteroid>, Without<AsteroidFragment>)>,
    mut fragments: Query<(Entity, &mut Transform, &Velocity, &Rotating), (With<Asteroid>, With<AsteroidFragment>)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, velocity, rotating, respawn) in &mut big {
        if respawn.is_some() {
            continue;
        }

        transform.translation += velocity.0 * dt;
        transform.rotate_local(Quat::from_euler(
            EulerRot::XYZ,
            rotating.0.x * dt,
            rotating.0.y * dt,
            rotating.0.z * dt,
        ));

        if transform.translation.distance(Vec3::new(0.0, 0.0, ARENA_CENTER)) > ASTEROID_DESPAWN_DISTANCE {
            transform.translation = Vec3::new(0.0, -9999.0, 0.0);

            commands.entity(entity).insert((
                Visibility::Hidden,
                AsteroidRespawn(Timer::from_seconds(ASTEROID_RESPAWN_TIME, TimerMode::Once)),
            ));
        }
    }

    for (entity, mut transform, velocity, rotating) in &mut fragments {
        transform.translation += velocity.0 * dt;
        transform.rotate_local(Quat::from_euler(
            EulerRot::XYZ,
            rotating.0.x * dt,
            rotating.0.y * dt,
            rotating.0.z * dt,
        ));

        if transform.translation.distance(Vec3::new(0.0, 0.0, ARENA_CENTER)) > ASTEROID_DESPAWN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}

pub fn respawn_asteroids(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Visibility, &mut Velocity, &AsteroidHealth, &mut AsteroidRespawn), (With<Asteroid>, Without<AsteroidFragment>)>,
    mut commands: Commands,
) {
    let arena_center = Vec3::new(0.0, 0.0, ARENA_CENTER);

    for (entity, mut transform, mut visibility, mut velocity, health, mut respawn) in &mut query {
        if health.0 <= 0.0 {
            continue;
        }

        respawn.0.tick(time.delta());

        if respawn.0.just_finished() {
            let mut rng = rand::rng();
            let angle = rng.random_range(0.0..std::f32::consts::TAU);

            let spawn_pos = Vec3::new(
                angle.cos() * ASTEROID_RESPAWN_DISTANCE,
                rng.random_range(-20.0..20.0),
                ARENA_CENTER + angle.sin() * ASTEROID_RESPAWN_DISTANCE,
            );

            let dir_to_arena = (arena_center - spawn_pos).normalize_or_zero();
            let speed = rng.random_range(ASTEROID_MIN_SPEED..ASTEROID_MAX_SPEED);

            transform.translation = spawn_pos;
            velocity.0 = dir_to_arena * speed;
            *visibility = Visibility::Inherited;

            commands.entity(entity).remove::<AsteroidRespawn>();
        }
    }
}
