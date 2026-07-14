use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

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

    let pitch_factor = 0.55;

    if keyboard.pressed(KeyCode::KeyW) {
        target_rot.x -= PLAYER_TURN_SPEED * pitch_factor;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        target_rot.x += PLAYER_TURN_SPEED * pitch_factor;
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
