use bevy::prelude::*;

use crate::components::*;

pub fn move_lasers(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), Or<(With<PlayerLaser>, With<EnemyLaser>, With<UfoLaser>)>>,
) {
    let dt = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * dt;
    }
}

pub fn despawn_lasers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnAfter), Or<(With<PlayerLaser>, With<EnemyLaser>, With<UfoLaser>)>>,
) {
    for (entity, mut timer) in &mut query {
        timer.0 -= time.delta_secs();
        if timer.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
