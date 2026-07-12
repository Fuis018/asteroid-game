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

pub fn ufo_chase(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Ufo>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Ufo>)>,
    asteroid_query: Query<&Transform, (With<Asteroid>, Without<Ufo>)>,
    mut ufo_query: Query<
        (Entity, &mut Transform, &mut Speed, &mut FireCooldown, &mut PatrolAngle, &Health, Option<&mut UfoHitRecover>),
        With<Ufo>,
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(player_transform) = player_query.iter().next() else {
        return;
    };

    let player_pos = player_transform.translation;
    let enemy_pos = if let Some(enemy_tf) = enemy_query.iter().next() {
        enemy_tf.translation
    } else {
        Vec3::new(0.0, 13.0, ARENA_CENTER)
    };

    let dt = time.delta_secs();

    for (ufo_entity, mut transform, mut speed, mut cooldown, mut patrol_angle, health, hit_recover) in &mut ufo_query {
        if health.0 <= 0.0 {
            continue;
        }

        if let Some(mut recover) = hit_recover {
            recover.0.tick(time.delta());
            transform.scale = Vec3::splat(0.8);
            if recover.0.just_finished() {
                transform.scale = Vec3::splat(1.0);
                commands.entity(ufo_entity).remove::<UfoHitRecover>();
            }
            continue;
        }

        let ufo_pos = transform.translation;

        patrol_angle.0 += UFO_ORBIT_SPEED * dt;
        let t = patrol_angle.0;

        let orbit_center = enemy_pos;
        let orbit_target = orbit_center + Vec3::new(
            UFO_ORBIT_RADIUS * t.cos(),
            5.0 * (t * 0.5).sin(),
            UFO_ORBIT_RADIUS * t.sin(),
        );

        let to_player = player_pos - ufo_pos;
        let dist_to_player = to_player.length();

        let mut desired_dir = if dist_to_player < UFO_CHASE_RANGE {
            to_player.normalize_or_zero()
        } else {
            (orbit_target - ufo_pos).normalize_or_zero()
        };

        if dist_to_player < UFO_MIN_PLAYER_DISTANCE && dist_to_player > 0.1 {
            let away_from_player = -to_player.normalize();
            let blend = 1.0 - (dist_to_player / UFO_MIN_PLAYER_DISTANCE);
            desired_dir = (desired_dir + away_from_player * blend * 2.0).normalize_or_zero();

            let side_bias = if (patrol_angle.0 * 10.0).sin() >= 0.0 {
                1.0
            } else {
                -1.0
            };
            let lateral = Vec3::new(side_bias * 0.8, 0.0, 0.0);
            desired_dir = (desired_dir + lateral).normalize_or_zero();
        }

        for ast_tf in &asteroid_query {
            let to_asteroid = ast_tf.translation - ufo_pos;
            let dist_to_asteroid = to_asteroid.length();
            if dist_to_asteroid < UFO_DODGE_DISTANCE && dist_to_asteroid > 0.1 {
                let away = -to_asteroid.normalize();
                let urgency = 1.0 - (dist_to_asteroid / UFO_DODGE_DISTANCE);
                desired_dir = (desired_dir + away * urgency * UFO_DODGE_FORCE * dt).normalize_or_zero();
            }
        }

        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, desired_dir);
        transform.rotation = transform.rotation.slerp(target_rotation, UFO_TURN_SPEED * dt);

        let target_speed = if dist_to_player < UFO_STOP_DISTANCE {
            UFO_SPEED * 0.55
        } else {
            UFO_SPEED
        };

        let speed_change_rate = 40.0;
        if speed.0 < target_speed {
            speed.0 = (speed.0 + speed_change_rate * dt).min(target_speed);
        } else if speed.0 > target_speed {
            speed.0 = (speed.0 - speed_change_rate * dt).max(target_speed);
        }

        if speed.0 > 0.01 {
            let forward = transform.forward().as_vec3();
            transform.translation += forward * speed.0 * dt;
        }

        if cooldown.0 <= 0.0 && dist_to_player < UFO_SHOOT_RANGE {
            let spawn_pos = transform.translation + transform.forward().as_vec3() * 8.0;
            let laser_dir = to_player.normalize();

            let laser_mesh = meshes.add(Sphere::new(0.35));
            let laser_material = materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.2, 1.0),
                emissive: LinearRgba::new(6.0, 0.4, 8.0, 1.0),
                ..default()
            });

            commands.spawn((
                Mesh3d(laser_mesh),
                MeshMaterial3d(laser_material),
                Transform::from_translation(spawn_pos),
                UfoLaser,
                Velocity(laser_dir * LASER_SPEED),
                DespawnAfter(LASER_LIFETIME),
                RigidBody::Kinematic,
                Collider::sphere(0.35),
                CollisionEventsEnabled,
            ));

            cooldown.0 = UFO_FIRE_RATE;
        }

        if cooldown.0 > 0.0 {
            cooldown.0 -= dt;
        }
    }
}
