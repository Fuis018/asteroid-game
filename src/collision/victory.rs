use bevy::prelude::*;

use crate::components::*;
use crate::particles;
use crate::GameState;

pub fn check_victory(
    enemy_query: Query<&Enemy>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if enemy_query.is_empty() {
        next_state.set(GameState::Victory);
    }
}

pub fn check_ufo_destroyed(
    mut commands: Commands,
    ufo_query: Query<(Entity, &Transform, &Health), With<Ufo>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, tf, health) in &ufo_query {
        if health.0 <= 0.0 {
            particles::spawn_explosion_at(&mut commands, &mut meshes, &mut materials, tf.translation, Color::srgb(0.3, 0.7, 1.0), 3.0);
            commands.entity(entity).despawn();
        }
    }
}
