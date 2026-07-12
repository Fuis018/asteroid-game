use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;
use rand::seq::IndexedRandom;

use crate::components::*;
use crate::constants::*;

pub const ASTEROID_COLORS: [Color; 12] = [
    Color::srgb(0.40, 0.30, 0.22),
    Color::srgb(0.50, 0.38, 0.28),
    Color::srgb(0.30, 0.25, 0.20),
    Color::srgb(0.55, 0.35, 0.25),
    Color::srgb(0.35, 0.28, 0.22),
    Color::srgb(0.45, 0.32, 0.24),
    Color::srgb(0.28, 0.30, 0.28),
    Color::srgb(0.38, 0.25, 0.20),
    Color::srgb(0.48, 0.40, 0.32),
    Color::srgb(0.32, 0.22, 0.18),
    Color::srgb(0.42, 0.34, 0.26),
    Color::srgb(0.52, 0.42, 0.30),
];

pub fn spawn_asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();
    for _ in 0..ASTEROID_COUNT {
        let zone = rng.random_range(0.0..1.0);
        let (x, y, z) = if zone < 0.6 {
            let angle = rng.random_range(0.0..std::f32::consts::TAU);
            let r = rng.random_range(20.0..ARENA_RADIUS * 1.5);
            (
                angle.cos() * r,
                rng.random_range(-25.0..25.0),
                ARENA_CENTER + angle.sin() * r,
            )
        } else {
            (
                rng.random_range(-ASTEROID_FIELD_RADIUS * 0.5..ASTEROID_FIELD_RADIUS * 0.5),
                rng.random_range(-25.0..25.0),
                rng.random_range(ARENA_CENTER - ASTEROID_FIELD_RADIUS * 0.5..40.0),
            )
        };

        let scale = rng.random_range(ASTEROID_MIN_SCALE..ASTEROID_MAX_SCALE);
        let velocity = Vec3::new(
            rng.random_range(-ASTEROID_MAX_SPEED..ASTEROID_MAX_SPEED),
            rng.random_range(-ASTEROID_MAX_SPEED * 0.3..ASTEROID_MAX_SPEED * 0.3),
            rng.random_range(-ASTEROID_MAX_SPEED..ASTEROID_MAX_SPEED),
        );
        let rot_speed = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        );

        let color = *ASTEROID_COLORS.choose(&mut rng).unwrap();

        let parent = commands
            .spawn((
                Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
                Asteroid,
                AsteroidHealth(40.0),
                Velocity(velocity),
                Rotating(rot_speed),
                RigidBody::Kinematic,
                CollisionEventsEnabled,
            ))
            .id();

        build_asteroid_children(&mut commands, &mut meshes, &mut materials, parent, &mut rng, color, 1.0);
    }
}

fn build_asteroid_children(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
    rng: &mut impl Rng,
    color: Color,
    _scale_factor: f32,
) {
    let darker = Color::srgb(
        color.to_linear().red * 0.7,
        color.to_linear().green * 0.7,
        color.to_linear().blue * 0.7,
    );
    let lighter = Color::srgb(
        (color.to_linear().red * 1.2).min(1.0),
        (color.to_linear().green * 1.2).min(1.0),
        (color.to_linear().blue * 1.2).min(1.0),
    );

    let material_main = materials.add(StandardMaterial { base_color: color, ..default() });
    let material_dark = materials.add(StandardMaterial { base_color: darker, ..default() });
    let material_light = materials.add(StandardMaterial { base_color: lighter, ..default() });

    let core_radius = rng.random_range(1.0..1.5);
    let core_stretch = rng.random_range(1.05..1.25);
    let squash = rng.random_range(0.8..0.92);
    let core_scale = if rng.random_bool(0.5) {
        Vec3::new(core_stretch, squash, 1.0)
    } else {
        Vec3::new(1.0, squash, core_stretch)
    };
    let mut max_reach = core_radius * core_stretch;

    let core_mesh = meshes.add(Sphere::new(core_radius).mesh().ico(4).unwrap());
    let core_half_height = (core_radius * core_stretch - core_radius * squash).max(0.0);
    commands.entity(parent).with_children(|p| {
        p.spawn((
            Mesh3d(core_mesh),
            MeshMaterial3d(material_main),
            Transform::from_scale(core_scale),
            AsteroidPiece,
            AsteroidGroup::Core,
            Collider::capsule(core_radius * squash, core_half_height),
            AsteroidHitArea::Core,
        ));
    });

    let medium_radius = core_radius * 0.75;
    let num_medium = rng.random_range(0..=1u32);

    for mi in 0..num_medium {
        let offset_len = rng.random_range(0.5..core_radius * 0.8);
        let offset_dir = random_dir(rng);
        let medium_center = offset_dir * offset_len;

        let reach = offset_len + medium_radius;
        if reach > max_reach {
            max_reach = reach;
        }

        let mesh = meshes.add(Sphere::new(medium_radius).mesh().ico(3).unwrap());
        let mc = medium_center;
        commands.entity(parent).with_children(|p| {
            p.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material_dark.clone()),
                Transform::from_translation(mc),
                AsteroidPiece,
                AsteroidGroup::Medium(mi as usize),
                Collider::sphere(medium_radius),
                AsteroidHitArea::Medium,
            ));
        });

        let num_small_on_medium = rng.random_range(6..=12u32);
        for _ in 0..num_small_on_medium {
            let small_radius = rng.random_range(0.06..0.18);
            let small_offset_len = rng.random_range(0.1..medium_radius * 0.9);
            let small_dir = random_dir(rng);
            let small_pos = medium_center + small_dir * small_offset_len;

            let reach = small_pos.length() + small_radius;
            if reach > max_reach {
                max_reach = reach;
            }

            let mat = if rng.random_bool(0.5) {
                material_light.clone()
            } else {
                material_dark.clone()
            };
            let mesh = meshes.add(Sphere::new(small_radius).mesh().ico(2).unwrap());
            let sp = small_pos;
            commands.entity(parent).with_children(|p| {
                p.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(mat),
                    Transform::from_translation(sp),
                    AsteroidPiece,
                    AsteroidGroup::Medium(mi as usize),
                ));
            });
        }
    }

    let num_small_direct = rng.random_range(12..=24u32);
    for _ in 0..num_small_direct {
        let small_radius = rng.random_range(0.08..0.28);
        let offset_len = rng.random_range(0.4..core_radius * 1.1);
        let offset_dir = random_dir(rng);
        let offset = offset_dir * offset_len;

        let reach = offset_len + small_radius;
        if reach > max_reach {
            max_reach = reach;
        }

        let mat = if rng.random_bool(0.5) {
            material_light.clone()
        } else {
            material_dark.clone()
        };
        let mesh = meshes.add(Sphere::new(small_radius).mesh().ico(2).unwrap());
        let o = offset;
        commands.entity(parent).with_children(|p| {
            p.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(mat),
                Transform::from_translation(o),
                AsteroidPiece,
                AsteroidGroup::Core,
            ));
        });
    }
}

pub fn spawn_initial_fragments(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();
    for _ in 0..ASTEROID_FRAGMENT_INITIAL_COUNT {
        let zone = rng.random_range(0.0..1.0);
        let (x, y, z) = if zone < 0.6 {
            let angle = rng.random_range(0.0..std::f32::consts::TAU);
            let r = rng.random_range(20.0..ARENA_RADIUS * 1.5);
            (
                angle.cos() * r,
                rng.random_range(-25.0..25.0),
                ARENA_CENTER + angle.sin() * r,
            )
        } else {
            (
                rng.random_range(-ASTEROID_FIELD_RADIUS * 0.5..ASTEROID_FIELD_RADIUS * 0.5),
                rng.random_range(-25.0..25.0),
                rng.random_range(ARENA_CENTER - ASTEROID_FIELD_RADIUS * 0.5..40.0),
            )
        };

        let scale = rng.random_range(ASTEROID_MIN_SCALE..ASTEROID_MAX_SCALE);
        let velocity = Vec3::new(
            rng.random_range(-ASTEROID_MAX_SPEED..ASTEROID_MAX_SPEED),
            rng.random_range(-ASTEROID_MAX_SPEED * 0.3..ASTEROID_MAX_SPEED * 0.3),
            rng.random_range(-ASTEROID_MAX_SPEED..ASTEROID_MAX_SPEED),
        );
        let rot_speed = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        );

        let color = *ASTEROID_COLORS.choose(&mut rng).unwrap();

        let parent = commands
            .spawn((
                Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
                Asteroid,
                AsteroidHealth(60.0),
                AsteroidFragment,
                Velocity(velocity),
                Rotating(rot_speed),
                RigidBody::Kinematic,
                CollisionEventsEnabled,
            ))
            .id();

        let darker = Color::srgb(
            color.to_linear().red * 0.7,
            color.to_linear().green * 0.7,
            color.to_linear().blue * 0.7,
        );
        let lighter = Color::srgb(
            (color.to_linear().red * 1.2).min(1.0),
            (color.to_linear().green * 1.2).min(1.0),
            (color.to_linear().blue * 1.2).min(1.0),
        );

        let material_main = materials.add(StandardMaterial { base_color: color, ..default() });
        let material_dark = materials.add(StandardMaterial { base_color: darker, ..default() });
        let material_light = materials.add(StandardMaterial { base_color: lighter, ..default() });

        let core_radius = rng.random_range(0.8..1.2);
        let core_stretch = rng.random_range(1.05..1.25);
        let squash = rng.random_range(0.8..0.92);
        let core_scale = if rng.random_bool(0.5) {
            Vec3::new(core_stretch, squash, 1.0)
        } else {
            Vec3::new(1.0, squash, core_stretch)
        };

        let core_mesh = meshes.add(Sphere::new(core_radius).mesh().ico(4).unwrap());
        let core_half_height = (core_radius * core_stretch - core_radius * squash).max(0.0);
        commands.entity(parent).with_children(|p| {
            p.spawn((
                Mesh3d(core_mesh),
                MeshMaterial3d(material_main),
                Transform::from_scale(core_scale),
                AsteroidPiece,
                AsteroidGroup::Core,
                Collider::capsule(core_radius * squash, core_half_height),
                AsteroidHitArea::Core,
            ));
        });

        let num_small = rng.random_range(6..=14u32);
        for _ in 0..num_small {
            let small_radius = rng.random_range(0.08..0.28);
            let offset_len = rng.random_range(0.4..core_radius * 1.1);
            let offset_dir = random_dir(&mut rng);
            let offset = offset_dir * offset_len;
            let mat = if rng.random_bool(0.5) {
                material_light.clone()
            } else {
                material_dark.clone()
            };
            let mesh = meshes.add(Sphere::new(small_radius).mesh().ico(2).unwrap());
            let o = offset;
            commands.entity(parent).with_children(|p| {
                p.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(mat),
                    Transform::from_translation(o),
                    AsteroidPiece,
                    AsteroidGroup::Core,
                ));
            });
        }
    }
}

pub fn split_asteroids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    dying: Query<(Entity, &Transform, &Velocity, &AsteroidHealth, &Children), (With<Asteroid>, Without<AsteroidFragment>)>,
    dying_fragments: Query<(Entity, &AsteroidHealth, &Transform), (With<Asteroid>, With<AsteroidFragment>)>,
    group_query: Query<&AsteroidGroup>,
) {
    for (entity, health, transform) in &dying_fragments {
        if health.0 <= 0.0 {
            let color = *ASTEROID_COLORS.choose(&mut rand::rng()).unwrap();
            crate::particles::spawn_explosion_at(
                &mut commands, &mut meshes, &mut materials,
                transform.translation, color, transform.scale.x,
            );
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform, velocity, health, children) in &dying {
        if health.0 > 0.0 {
            continue;
        }

        let mut core_children: Vec<Entity> = Vec::new();
        let mut medium_children: Vec<Entity> = Vec::new();

        for child in children.iter() {
            match group_query.get(child) {
                Ok(AsteroidGroup::Core) => core_children.push(child),
                Ok(AsteroidGroup::Medium(_)) => medium_children.push(child),
                _ => {}
            }
        }

        let mut rng = rand::rng();

        if !medium_children.is_empty() {
            let new_scale = transform.scale.x * 0.85;
            let spread_vel = Vec3::new(
                rng.random_range(-3.0..3.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-3.0..3.0),
            );
            let new_parent = commands.spawn((
                Transform {
                    translation: transform.translation,
                    scale: Vec3::splat(new_scale),
                    rotation: transform.rotation,
                },
                Asteroid,
                AsteroidHealth(60.0),
                AsteroidFragment,
                Velocity(velocity.0 + spread_vel),
                Rotating(Vec3::new(
                    rng.random_range(-1.5..1.5),
                    rng.random_range(-1.5..1.5),
                    rng.random_range(-1.5..1.5),
                )),
                RigidBody::Kinematic,
                CollisionEventsEnabled,
            )).id();

            let color = *ASTEROID_COLORS.choose(&mut rng).unwrap();
            let medium_radius = 1.2;
            let material_main = materials.add(StandardMaterial {
                base_color: color,
                ..default()
            });
            let material_dark = materials.add(StandardMaterial {
                base_color: Color::srgb(
                    color.to_linear().red * 0.7,
                    color.to_linear().green * 0.7,
                    color.to_linear().blue * 0.7,
                ),
                ..default()
            });
            let material_light = materials.add(StandardMaterial {
                base_color: Color::srgb(
                    (color.to_linear().red * 1.2).min(1.0),
                    (color.to_linear().green * 1.2).min(1.0),
                    (color.to_linear().blue * 1.2).min(1.0),
                ),
                ..default()
            });

            let core_stretch = rng.random_range(1.05..1.25);
            let squash = rng.random_range(0.8..0.92);
            let core_scale = if rng.random_bool(0.5) {
                Vec3::new(core_stretch, squash, 1.0)
            } else {
                Vec3::new(1.0, squash, core_stretch)
            };
            let core_radius = medium_radius / 0.75;
            let core_mesh = meshes.add(Sphere::new(core_radius).mesh().ico(4).unwrap());
            let core_half_height = (core_radius * core_stretch - core_radius * squash).max(0.0);
            commands.entity(new_parent).with_children(|p| {
                p.spawn((
                    Mesh3d(core_mesh),
                    MeshMaterial3d(material_main),
                    Transform::from_scale(core_scale),
                    AsteroidPiece,
                    AsteroidGroup::Core,
                    Collider::capsule(core_radius * squash, core_half_height),
                    AsteroidHitArea::Core,
                ));
            });

            let mesh = meshes.add(Sphere::new(medium_radius).mesh().ico(3).unwrap());
            let offset_dir = random_dir(&mut rng);
            let mc = offset_dir * rng.random_range(0.5..core_radius * 0.8);
            let mcm = mc;
            commands.entity(new_parent).with_children(|p| {
                p.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(material_dark.clone()),
                    Transform::from_translation(mcm),
                    AsteroidPiece,
                    AsteroidGroup::Medium(0),
                    Collider::sphere(medium_radius),
                    AsteroidHitArea::Medium,
                ));
            });

            let num_small = rng.random_range(8..=16u32);
            for _ in 0..num_small {
                let small_radius = rng.random_range(0.06..0.18);
                let small_offset_len = rng.random_range(0.1..medium_radius * 0.9);
                let small_dir = random_dir(&mut rng);
                let small_pos = mc + small_dir * small_offset_len;
                let mat = if rng.random_bool(0.5) {
                    material_light.clone()
                } else {
                    material_dark.clone()
                };
                let mesh = meshes.add(Sphere::new(small_radius).mesh().ico(2).unwrap());
                let sp = small_pos;
                commands.entity(new_parent).with_children(|p| {
                    p.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(mat),
                        Transform::from_translation(sp),
                        AsteroidPiece,
                        AsteroidGroup::Medium(0),
                    ));
                });
            }

            let num_small_core = rng.random_range(6..=12u32);
            for _ in 0..num_small_core {
                let small_radius = rng.random_range(0.08..0.28);
                let offset_len = rng.random_range(0.4..core_radius * 1.1);
                let offset_dir = random_dir(&mut rng);
                let offset = offset_dir * offset_len;
                let mat = if rng.random_bool(0.5) {
                    material_light.clone()
                } else {
                    material_dark.clone()
                };
                let mesh = meshes.add(Sphere::new(small_radius).mesh().ico(2).unwrap());
                let o = offset;
                commands.entity(new_parent).with_children(|p| {
                    p.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(mat),
                        Transform::from_translation(o),
                        AsteroidPiece,
                        AsteroidGroup::Core,
                    ));
                });
            }

        }

        if !core_children.is_empty() {
            let new_scale = transform.scale.x * 0.8;
            let spread_vel = Vec3::new(
                rng.random_range(-3.0..3.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-3.0..3.0),
            );
            let new_parent = commands.spawn((
                Transform {
                    translation: transform.translation,
                    scale: Vec3::splat(new_scale),
                    rotation: transform.rotation,
                },
                Asteroid,
                AsteroidHealth(60.0),
                AsteroidFragment,
                Velocity(velocity.0 - spread_vel),
                Rotating(Vec3::new(
                    rng.random_range(-1.5..1.5),
                    rng.random_range(-1.5..1.5),
                    rng.random_range(-1.5..1.5),
                )),
                RigidBody::Kinematic,
                CollisionEventsEnabled,
            )).id();

            let color = *ASTEROID_COLORS.choose(&mut rng).unwrap();
            let core_radius = rng.random_range(0.8..1.2);
            let core_stretch = rng.random_range(1.05..1.25);
            let squash = rng.random_range(0.8..0.92);
            let core_scale = if rng.random_bool(0.5) {
                Vec3::new(core_stretch, squash, 1.0)
            } else {
                Vec3::new(1.0, squash, core_stretch)
            };

            let material_main = materials.add(StandardMaterial {
                base_color: color,
                ..default()
            });
            let material_light = materials.add(StandardMaterial {
                base_color: Color::srgb(
                    (color.to_linear().red * 1.2).min(1.0),
                    (color.to_linear().green * 1.2).min(1.0),
                    (color.to_linear().blue * 1.2).min(1.0),
                ),
                ..default()
            });
            let material_dark = materials.add(StandardMaterial {
                base_color: Color::srgb(
                    color.to_linear().red * 0.7,
                    color.to_linear().green * 0.7,
                    color.to_linear().blue * 0.7,
                ),
                ..default()
            });

            let core_mesh = meshes.add(Sphere::new(core_radius).mesh().ico(4).unwrap());
            let core_half_height = (core_radius * core_stretch - core_radius * squash).max(0.0);
            commands.entity(new_parent).with_children(|p| {
                p.spawn((
                    Mesh3d(core_mesh),
                    MeshMaterial3d(material_main),
                    Transform::from_scale(core_scale),
                    AsteroidPiece,
                    AsteroidGroup::Core,
                    Collider::capsule(core_radius * squash, core_half_height),
                    AsteroidHitArea::Core,
                ));
            });

            let num_small = rng.random_range(6..=12u32);
            for _ in 0..num_small {
                let small_radius = rng.random_range(0.08..0.28);
                let offset_len = rng.random_range(0.4..core_radius * 1.1);
                let offset_dir = random_dir(&mut rng);
                let offset = offset_dir * offset_len;
                let mat = if rng.random_bool(0.5) {
                    material_light.clone()
                } else {
                    material_dark.clone()
                };
                let mesh = meshes.add(Sphere::new(small_radius).mesh().ico(2).unwrap());
                let o = offset;
                commands.entity(new_parent).with_children(|p| {
                    p.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(mat),
                        Transform::from_translation(o),
                        AsteroidPiece,
                        AsteroidGroup::Core,
                    ));
                });
            }

        }

        let color = *ASTEROID_COLORS.choose(&mut rng).unwrap();
        crate::particles::spawn_explosion_at(
            &mut commands, &mut meshes, &mut materials,
            transform.translation, color, transform.scale.x,
        );

        commands.entity(entity).despawn();
    }
}

fn random_dir(rng: &mut impl Rng) -> Vec3 {
    let dir = Vec3::new(
        rng.random_range(-1.0..1.0),
        rng.random_range(-1.0..1.0),
        rng.random_range(-1.0..1.0),
    );
    dir.normalize_or_zero()
}

pub fn move_asteroids(
    time: Res<Time>,
    mut big: Query<(Entity, &mut Transform, &Velocity, &Rotating, Option<&AsteroidRespawn>), (With<Asteroid>, Without<AsteroidFragment>)>,
    mut fragments: Query<(Entity, &mut Transform, &Velocity, &Rotating), (With<Asteroid>, With<AsteroidFragment>)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, velocity, rotating, respawn) in &mut big {
        if respawn.is_some() {
            continue;
        }

        transform.translation += velocity.0 * dt;
        transform.rotate_local(Quat::from_euler(
            EulerRot::XYZ,
            rotating.0.x * dt,
            rotating.0.y * dt,
            rotating.0.z * dt,
        ));

        if transform.translation.distance(Vec3::new(0.0, 0.0, ARENA_CENTER)) > ASTEROID_DESPAWN_DISTANCE {
            transform.translation = Vec3::new(0.0, -9999.0, 0.0);

            commands.entity(entity).insert((
                Visibility::Hidden,
                AsteroidRespawn(Timer::from_seconds(ASTEROID_RESPAWN_TIME, TimerMode::Once)),
            ));
        }
    }

    for (entity, mut transform, velocity, rotating) in &mut fragments {
        transform.translation += velocity.0 * dt;
        transform.rotate_local(Quat::from_euler(
            EulerRot::XYZ,
            rotating.0.x * dt,
            rotating.0.y * dt,
            rotating.0.z * dt,
        ));

        if transform.translation.distance(Vec3::new(0.0, 0.0, ARENA_CENTER)) > ASTEROID_DESPAWN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}

pub fn respawn_asteroids(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Visibility, &mut Velocity, &AsteroidHealth, &mut AsteroidRespawn), (With<Asteroid>, Without<AsteroidFragment>)>,
    mut commands: Commands,
) {
    let arena_center = Vec3::new(0.0, 0.0, ARENA_CENTER);

    for (entity, mut transform, mut visibility, mut velocity, health, mut respawn) in &mut query {
        if health.0 <= 0.0 {
            continue;
        }

        respawn.0.tick(time.delta());

        if respawn.0.just_finished() {
            let mut rng = rand::rng();
            let angle = rng.random_range(0.0..std::f32::consts::TAU);

            let spawn_pos = Vec3::new(
                angle.cos() * ASTEROID_RESPAWN_DISTANCE,
                rng.random_range(-20.0..20.0),
                ARENA_CENTER + angle.sin() * ASTEROID_RESPAWN_DISTANCE,
            );

            let dir_to_arena = (arena_center - spawn_pos).normalize_or_zero();
            let speed = rng.random_range(ASTEROID_MIN_SPEED..ASTEROID_MAX_SPEED);

            transform.translation = spawn_pos;
            velocity.0 = dir_to_arena * speed;
            *visibility = Visibility::Inherited;

            commands.entity(entity).remove::<AsteroidRespawn>();
        }
    }
}
