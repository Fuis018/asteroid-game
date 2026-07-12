use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;
use crate::assets::GameAssets;
use crate::components::*;
use crate::constants::*;
use crate::ufo::{spawn_ufo_entity, UfoAssets};
use crate::particles::spawn_explosion_at;

const TURRET_OFFSETS: [Vec3; 4] = [
    Vec3::new(-3.0, 6.0, -10.0),
    Vec3::new(3.0, 6.0, -10.0),
    Vec3::new(-3.0, 6.0, 10.0),
    Vec3::new(3.0, 6.0, 10.0),
];

pub fn spawn_enemy(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let turret_mesh = meshes.add(Cuboid::new(0.8, 0.8, 2.0));
    let turret_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        emissive: LinearRgba::new(1.0, 0.2, 0.0, 1.0),
        ..default()
    });

    let turret_offsets = TURRET_OFFSETS;

    let enemy = commands.spawn((
        WorldAssetRoot(assets.enemy_ship.clone()),
        Transform::from_xyz(0.0, 13.0, ARENA_CENTER),
        Enemy,
        Health(ENEMY_MAX_HEALTH),
        FireCooldown(0.0),
        Velocity(Vec3::ZERO),
        PatrolAngle(0.0),
        RigidBody::Kinematic,
        Collider::compound(vec![(
            Vec3::new(0.0, 4.0, 0.0),
            Quat::IDENTITY,
            Collider::cuboid(12.0, 16.0, 60.0),
        )]),
        CollisionEventsEnabled,
        EnemyFlags::default(),
    )).id();

    for offset in turret_offsets {
        commands.spawn((
            Mesh3d(turret_mesh.clone()),
            MeshMaterial3d(turret_mat.clone()),
            Transform::from_translation(offset),
            Turret,
            ChildOf(enemy),
        ));
    }
}

pub fn enemy_patrol(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PatrolAngle, &mut FireCooldown, &Health), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok((mut transform, mut patrol_angle, mut cooldown, health)) = query.single_mut() else {
        return;
    };

    if health.0 <= 0.0 {
        return;
    }

    let dt = time.delta_secs();
    patrol_angle.0 += ENEMY_ORBIT_SPEED * dt;
    let t = patrol_angle.0;

    let center = Vec3::new(0.0, 13.0, ARENA_CENTER);
    let path_x = ENEMY_ORBIT_RADIUS * 0.95 * t.cos();
    let path_z = ENEMY_ORBIT_RADIUS * 0.65 * (2.0 * t).sin();
    let target_pos = center + Vec3::new(path_x, 0.0, path_z);

    let direction = target_pos - transform.translation;
    let dist = direction.length();
    if dist > 0.5 {
        transform.translation += direction.normalize() * ENEMY_CHASE_SPEED * dt;
    }

    let ahead_t = t + 0.1;
    let ahead_path_x = ENEMY_ORBIT_RADIUS * 0.95 * ahead_t.cos();
    let ahead_path_z = ENEMY_ORBIT_RADIUS * 0.65 * (2.0 * ahead_t).sin();
    let ahead_pos = center + Vec3::new(ahead_path_x, 0.0, ahead_path_z);
    let look_dir = (ahead_pos - transform.translation).normalize_or_zero();
    if look_dir.length_squared() > 0.01 {
        let current_forward = transform.rotation * -Vec3::Z;
        let angle = current_forward.angle_between(look_dir);
        let max_angle = 45.0_f32.to_radians();
        let limited_angle = angle.min(max_angle);
        let rotation_axis = current_forward.cross(look_dir).normalize_or_zero();
        let delta_rot = Quat::from_axis_angle(rotation_axis, limited_angle);
        let adjusted_dir = delta_rot * current_forward;
        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, adjusted_dir);
        transform.rotation = transform.rotation.slerp(target_rotation, 1.0);
    }

    let dist_to_player = transform.translation.distance(player_transform.translation);

    if cooldown.0 <= 0.0 && dist_to_player < ENEMY_SHOOT_RANGE {
        let laser_mesh = meshes.add(Sphere::new(0.2));
        let laser_material = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            emissive: LinearRgba::new(3.0, 0.0, 0.0, 1.0),
            ..default()
        });

        let to_player = (player_transform.translation - transform.translation).normalize();
        let enemy_rot = transform.rotation;
        let mut rng = rand::rng();

        for offset in TURRET_OFFSETS {
            let world_pos = transform.translation + enemy_rot * offset;
            let jitter = Vec3::new(
                rng.random_range(-0.08..0.08),
                rng.random_range(-0.08..0.08),
                rng.random_range(-0.08..0.08),
            );
            let fire_dir = (to_player + jitter).normalize();

            commands.spawn((
                Mesh3d(laser_mesh.clone()),
                MeshMaterial3d(laser_material.clone()),
                Transform::from_translation(world_pos),
                EnemyLaser,
                Velocity(fire_dir * LASER_SPEED),
                DespawnAfter(LASER_LIFETIME),
                RigidBody::Kinematic,
                Collider::sphere(0.2),
                CollisionEventsEnabled,
            ));
        }

        cooldown.0 = ENEMY_FIRE_RATE;
    }

    if cooldown.0 > 0.0 {
        cooldown.0 -= dt;
    }
}

pub fn check_enemy_health_flags(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &Health, &mut EnemyFlags), With<Enemy>>,
    ufo_assets: Res<UfoAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok((enemy_entity, transform, health, mut flags)) = enemy_query.single_mut() else {
        return;
    };

    if health.0 <= 0.0 {
        commands.entity(enemy_entity).despawn();
        return;
    }

    let enemy_pos = transform.translation;
    let first_flag_threshold = 2100.0;
    let second_flag_threshold = 1200.0;

    if !flags.flag_2100_triggered && health.0 <= first_flag_threshold {
        flags.flag_2100_triggered = true;
        // Spawn 2 UFOs around the enemy ship in a small ring.
        for i in 0..2 {
            let angle = (i as f32) * std::f32::consts::PI;
            let offset = Vec3::new(angle.cos() * 35.0, 0.0, angle.sin() * 35.0);
            spawn_ufo_entity(&mut commands, &ufo_assets, enemy_pos + offset);
        }
        spawn_explosion_at(
            &mut commands,
            &mut meshes,
            &mut materials,
            enemy_pos,
            Color::srgb(0.8, 0.2, 0.2),
            5.0,
        );
    }

    if !flags.flag_1200_triggered && health.0 <= second_flag_threshold {
        flags.flag_1200_triggered = true;
        // Spawn 6 UFOs in a rectangular formation on both sides of the enemy ship.
        let side_offsets = [
            Vec3::new(-45.0, 0.0, -20.0),
            Vec3::new(-25.0, 0.0, -20.0),
            Vec3::new(-5.0, 0.0, -20.0),
            Vec3::new(45.0, 0.0, 20.0),
            Vec3::new(25.0, 0.0, 20.0),
            Vec3::new(5.0, 0.0, 20.0),
        ];
        for offset in side_offsets {
            spawn_ufo_entity(&mut commands, &ufo_assets, enemy_pos + offset);
        }
        spawn_explosion_at(
            &mut commands,
            &mut meshes,
            &mut materials,
            enemy_pos,
            Color::srgb(0.8, 0.4, 0.2),
            8.0,
        );
    }

}
