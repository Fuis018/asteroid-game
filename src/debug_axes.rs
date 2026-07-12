use bevy::prelude::*;
use crate::components::{Player, Enemy};
use crate::debug_colliders::DebugColliders;

const AXIS_LENGTH: f32 = 5.0;
const ENEMY_AXIS_LENGTH: f32 = 15.0;

pub fn draw_global_axes(
    debug: Res<DebugColliders>,
    mut gizmos: Gizmos,
) {
    if !debug.0 {
        return;
    }
    let origin = Vec3::ZERO;
    gizmos.arrow(origin, origin + Vec3::X * AXIS_LENGTH * 3.0, Color::srgb(1.0, 0.0, 0.0));
    gizmos.arrow(origin, origin + Vec3::Y * AXIS_LENGTH * 3.0, Color::srgb(0.0, 1.0, 0.0));
    gizmos.arrow(origin, origin + Vec3::Z * AXIS_LENGTH * 3.0, Color::srgb(0.0, 0.0, 1.0));
}

pub fn draw_local_axes(
    debug: Res<DebugColliders>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut gizmos: Gizmos,
) {
    if !debug.0 {
        return;
    }

    if let Ok(tf) = player_query.single() {
        let pos = tf.translation;
        let fwd = tf.forward().as_vec3();
        let right = tf.right().as_vec3();
        let up = tf.up().as_vec3();
        gizmos.arrow(pos, pos + right * AXIS_LENGTH, Color::srgb(1.0, 0.3, 0.3));
        gizmos.arrow(pos, pos + up * AXIS_LENGTH, Color::srgb(0.3, 1.0, 0.3));
        gizmos.arrow(pos, pos + fwd * AXIS_LENGTH, Color::srgb(0.3, 0.3, 1.0));
    }

    if let Ok(tf) = enemy_query.single() {
        let pos = tf.translation;
        let fwd = tf.forward().as_vec3();
        let right = tf.right().as_vec3();
        let up = tf.up().as_vec3();
        gizmos.arrow(pos, pos + right * ENEMY_AXIS_LENGTH, Color::srgb(1.0, 0.5, 0.5));
        gizmos.arrow(pos, pos + up * ENEMY_AXIS_LENGTH, Color::srgb(0.5, 1.0, 0.5));
        gizmos.arrow(pos, pos + fwd * ENEMY_AXIS_LENGTH, Color::srgb(0.5, 0.5, 1.0));
    }
}
