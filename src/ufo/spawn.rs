use avian3d::prelude::*;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

#[derive(Resource, Clone)]
pub struct UfoAssets {
    pub base_mesh: Handle<Mesh>,
    pub body_mesh: Handle<Mesh>,
    pub cockpit_mesh: Handle<Mesh>,
    pub ring_mesh: Handle<Mesh>,
    pub light_mesh: Handle<Mesh>,
    pub turret_mesh: Handle<Mesh>,
    pub base_mat: Handle<StandardMaterial>,
    pub body_mat: Handle<StandardMaterial>,
    pub cockpit_mat: Handle<StandardMaterial>,
    pub ring_mat: Handle<StandardMaterial>,
    pub light_mat: Handle<StandardMaterial>,
    pub turret_mat: Handle<StandardMaterial>,
}

pub fn spawn_ufo_entity(
    commands: &mut Commands,
    assets: &UfoAssets,
    position: Vec3,
) -> Entity {
    let mut transform = Transform::from_translation(position + Vec3::new(0.0, 2.0, 0.0));
    transform.scale = Vec3::splat(0.75);

    let ufo = commands.spawn((
        Mesh3d(assets.base_mesh.clone()),
        MeshMaterial3d(assets.base_mat.clone()),
        transform,
        Ufo,
        Health(UFO_MAX_HEALTH),
        Speed(0.0),
        FireCooldown(0.0),
        PatrolAngle(0.0),
        RigidBody::Kinematic,
        Collider::cuboid(14.0, 12.0, 14.0),
        CollisionEventsEnabled,
    )).id();

    commands.spawn((
        Mesh3d(assets.body_mesh.clone()),
        MeshMaterial3d(assets.body_mat.clone()),
        Transform::from_xyz(0.0, 2.2, 0.0),
        ChildOf(ufo),
    ));

    commands.spawn((
        Mesh3d(assets.cockpit_mesh.clone()),
        MeshMaterial3d(assets.cockpit_mat.clone()),
        Transform::from_xyz(0.0, 5.0, 0.0),
        ChildOf(ufo),
    ));

    commands.spawn((
        Mesh3d(assets.ring_mesh.clone()),
        MeshMaterial3d(assets.ring_mat.clone()),
        Transform::from_xyz(0.0, 2.2, 0.0),
        ChildOf(ufo),
    ));

    let light_offsets = [
        Vec3::new(5.5, 0.9, 0.0),
        Vec3::new(-5.5, 0.9, 0.0),
        Vec3::new(0.0, 0.9, 5.5),
        Vec3::new(0.0, 0.9, -5.5),
    ];
    for offset in light_offsets {
        commands.spawn((
            Mesh3d(assets.light_mesh.clone()),
            MeshMaterial3d(assets.light_mat.clone()),
            Transform::from_translation(offset),
            ChildOf(ufo),
        ));
    }

    let turret_offsets = [
        Vec3::new(-6.5, 1.5, 0.0),
        Vec3::new(6.5, 1.5, 0.0),
    ];
    for offset in turret_offsets {
        commands.spawn((
            Mesh3d(assets.turret_mesh.clone()),
            MeshMaterial3d(assets.turret_mat.clone()),
            Transform::from_translation(offset),
            UfoTurret,
            ChildOf(ufo),
        ));
    }

    ufo
}

pub fn spawn_ufo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_mesh = meshes.add(Cuboid::new(12.0, 1.6, 12.0));
    let body_mesh = meshes.add(Cuboid::new(10.0, 3.0, 10.0));
    let cockpit_mesh = meshes.add(Sphere::new(3.0));
    let ring_mesh = meshes.add(Torus::new(5.5, 0.35));
    let light_mesh = meshes.add(Sphere::new(0.3));
    let turret_mesh = meshes.add(Cuboid::new(0.6, 0.6, 2.5));

    let base_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.27, 0.3),
        metallic: 0.9,
        reflectance: 0.8,
        ..default()
    });
    let body_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.38, 0.42),
        metallic: 0.85,
        reflectance: 0.7,
        ..default()
    });
    let cockpit_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.55, 0.82, 0.95, 0.7),
        metallic: 0.1,
        reflectance: 0.95,
        ..default()
    });
    let ring_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.22, 0.25),
        metallic: 0.95,
        reflectance: 0.85,
        ..default()
    });
    let light_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.9, 1.0),
        emissive: LinearRgba::new(0.2, 0.8, 1.2, 1.0),
        ..default()
    });
    let turret_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.32, 0.35),
        metallic: 0.9,
        reflectance: 0.75,
        ..default()
    });

    let assets = UfoAssets {
        base_mesh,
        body_mesh,
        cockpit_mesh,
        ring_mesh,
        light_mesh,
        turret_mesh,
        base_mat,
        body_mat,
        cockpit_mat,
        ring_mat,
        light_mat,
        turret_mat,
    };

    commands.insert_resource(assets);
}
