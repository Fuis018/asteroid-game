use avian3d::prelude::*;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

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
