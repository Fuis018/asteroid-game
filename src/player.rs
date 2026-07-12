use avian3d::prelude::*;
use bevy::prelude::*;
use crate::assets::GameAssets;
use crate::components::*;
use crate::constants::*;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands.spawn((
        WorldAssetRoot(assets.player_ship.clone()),
        Transform::from_xyz(0.0, 3.0, 30.0),
        Player,
        Health(PLAYER_MAX_HEALTH),
        FireCooldown(0.0),
        Speed(PLAYER_MIN_SPEED),
        RigidBody::Kinematic,
        Collider::cuboid(4.5, 3.0, 10.0),
        CollisionEventsEnabled,
        RotationalVelocity(Vec3::ZERO),
    ));
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Speed, &mut RotationalVelocity), With<Player>>,
) {
    let Ok((mut transform, mut speed, mut rot_vel)) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let mut target_rot = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        target_rot.x -= PLAYER_TURN_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        target_rot.x += PLAYER_TURN_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        target_rot.y += PLAYER_TURN_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        target_rot.y -= PLAYER_TURN_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyQ) {
        target_rot.z += PLAYER_ROLL_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        target_rot.z -= PLAYER_ROLL_SPEED;
    }

    // Smoothly interpolate current rotational velocity to target rotational velocity
    let sensitivity = 8.0;
    rot_vel.0 = rot_vel.0.lerp(target_rot, sensitivity * dt);

    transform.rotate_local(Quat::from_euler(
        EulerRot::XYZ,
        rot_vel.0.x * dt,
        rot_vel.0.y * dt,
        rot_vel.0.z * dt,
    ));

    if keyboard.pressed(KeyCode::ShiftLeft) {
        speed.0 += PLAYER_ACCELERATION * dt;
    } else if keyboard.pressed(KeyCode::ControlLeft) {
        speed.0 -= PLAYER_BRAKE_DECELERATION * dt;
    } else {
        speed.0 -= PLAYER_DECELERATION * dt;
    }

    speed.0 = speed.0.clamp(PLAYER_MIN_SPEED, PLAYER_MAX_SPEED);

    if speed.0 > 0.01 {
        let forward = transform.forward().as_vec3();
        transform.translation += forward * speed.0 * dt;
    }
}

pub fn player_shooting(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Transform, &mut FireCooldown), With<Player>>,
) {
    let Ok((transform, mut cooldown)) = query.single_mut() else {
        return;
    };

    cooldown.0 -= time.delta_secs();

    if keyboard.pressed(KeyCode::Space) && cooldown.0 <= 0.0 {
        let forward = transform.forward().as_vec3();
        let right = transform.right().as_vec3();
        let left_offset = transform.translation + forward * 3.0 - right * 2.0;
        let right_offset = transform.translation + forward * 3.0 + right * 2.0;

        let laser_mesh = meshes.add(Sphere::new(0.4));
        let laser_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            emissive: LinearRgba::new(0.0, 5.0, 0.0, 1.0),
            ..default()
        });

        let left_dir = forward;
        let right_dir = forward;

        commands.spawn((
            Mesh3d(laser_mesh.clone()),
            MeshMaterial3d(laser_material.clone()),
            Transform::from_translation(left_offset),
            PlayerLaser,
            Velocity(left_dir * LASER_SPEED),
            DespawnAfter(LASER_LIFETIME),
            RigidBody::Kinematic,
            Collider::sphere(0.4),
            Sensor,
            CollisionEventsEnabled,
        ));

        commands.spawn((
            Mesh3d(laser_mesh),
            MeshMaterial3d(laser_material),
            Transform::from_translation(right_offset),
            PlayerLaser,
            Velocity(right_dir * LASER_SPEED),
            DespawnAfter(LASER_LIFETIME),
            RigidBody::Kinematic,
            Collider::sphere(0.4),
            Sensor,
            CollisionEventsEnabled,
        ));

        cooldown.0 = PLAYER_FIRE_RATE;
    }
}
