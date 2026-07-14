use avian3d::prelude::*;
use bevy::prelude::*;
use crate::assets::GameAssets;
use crate::components::*;
use crate::constants::*;

pub const TURRET_OFFSETS: [Vec3; 4] = [
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
