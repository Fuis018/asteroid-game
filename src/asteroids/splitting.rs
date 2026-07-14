use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;
use rand::seq::IndexedRandom;

use crate::components::*;
use super::colors::ASTEROID_COLORS;
use super::spawn::random_dir;

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
