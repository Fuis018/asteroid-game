use avian3d::prelude::*;
use bevy::prelude::*;
use crate::components::*;

#[derive(Resource)]
pub struct DebugColliders(pub bool);

fn draw_cuboid_wireframe(gizmos: &mut Gizmos, center: Vec3, half: Vec3, rotation: Quat, color: Color) {
    let corners = [
        Vec3::new(-half.x, -half.y, -half.z),
        Vec3::new( half.x, -half.y, -half.z),
        Vec3::new( half.x,  half.y, -half.z),
        Vec3::new(-half.x,  half.y, -half.z),
        Vec3::new(-half.x, -half.y,  half.z),
        Vec3::new( half.x, -half.y,  half.z),
        Vec3::new( half.x,  half.y,  half.z),
        Vec3::new(-half.x,  half.y,  half.z),
    ];

    let rotated: Vec<Vec3> = corners.iter().map(|c| center + rotation * *c).collect();

    let edges = [
        (0,1),(1,2),(2,3),(3,0),
        (4,5),(5,6),(6,7),(7,4),
        (0,4),(1,5),(2,6),(3,7),
    ];

    for (a, b) in edges {
        gizmos.line(rotated[a], rotated[b], color);
    }
}

fn draw_capsule_wireframe(gizmos: &mut Gizmos, center: Vec3, radius: f32, half_height: f32, rotation: Quat, color: Color) {
    let top = center + rotation * Vec3::Y * half_height;
    let bottom = center + rotation * Vec3::Y * -half_height;

    gizmos.sphere(Isometry3d::from_translation(top), radius, color);
    gizmos.sphere(Isometry3d::from_translation(bottom), radius, color);

    let offset = rotation * Vec3::X * radius;
    gizmos.line(top + offset, bottom + offset, color);
    gizmos.line(top - offset, bottom - offset, color);
    let offset_z = rotation * Vec3::Z * radius;
    gizmos.line(top + offset_z, bottom + offset_z, color);
    gizmos.line(top - offset_z, bottom - offset_z, color);
}

pub fn toggle_debug_colliders(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug: ResMut<DebugColliders>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        debug.0 = !debug.0;
    }
}

pub fn draw_colliders(
    debug: Res<DebugColliders>,
    player_q: Query<(&Transform, &Collider), With<Player>>,
    enemy_q: Query<(&Transform, &Collider), With<Enemy>>,
    ufo_q: Query<(&Transform, &Collider), With<Ufo>>,
    asteroid_hit_q: Query<(&Collider, &GlobalTransform, &AsteroidHitArea)>,
    laser_q: Query<(&Transform, &Collider), Or<(With<PlayerLaser>, With<EnemyLaser>)>>,
    mut gizmos: Gizmos,
) {
    if !debug.0 {
        return;
    }

    for (tf, col) in &player_q {
        let local_aabb = col.aabb(Vec3::ZERO, Quat::IDENTITY);
        let half = (local_aabb.max - local_aabb.min) * 0.5;
        let local_center = (local_aabb.max + local_aabb.min) * 0.5;
        let world_center = tf.translation + tf.rotation * local_center;
        draw_cuboid_wireframe(&mut gizmos, world_center, half, tf.rotation, Color::srgb(0.0, 1.0, 0.0));
    }

    for (tf, col) in &enemy_q {
        let local_aabb = col.aabb(Vec3::ZERO, Quat::IDENTITY);
        let half = (local_aabb.max - local_aabb.min) * 0.5;
        let local_center = (local_aabb.max + local_aabb.min) * 0.5;
        let world_center = tf.translation + tf.rotation * local_center;
        draw_cuboid_wireframe(&mut gizmos, world_center, half, tf.rotation, Color::srgb(1.0, 0.0, 0.0));
    }

    for (tf, col) in &ufo_q {
        let local_aabb = col.aabb(Vec3::ZERO, Quat::IDENTITY);
        let half = (local_aabb.max - local_aabb.min) * 0.5;
        let local_center = (local_aabb.max + local_aabb.min) * 0.5;
        let world_center = tf.translation + tf.rotation * local_center;
        draw_cuboid_wireframe(&mut gizmos, world_center, half, tf.rotation, Color::srgb(0.0, 0.7, 1.0));
    }

    for (col, gtf, hit_area) in &asteroid_hit_q {
        let pos = gtf.translation();
        let rotation = gtf.rotation();
        let aabb = col.aabb(Vec3::ZERO, Quat::IDENTITY);
        let size = aabb.max - aabb.min;

        match hit_area {
            AsteroidHitArea::Core => {
                let half_height = (size.y * 0.5 - size.x * 0.5).max(0.0);
                let radius = size.x * 0.5;
                draw_capsule_wireframe(&mut gizmos, pos, radius, half_height, rotation, Color::srgb(1.0, 1.0, 0.0));
            }
            AsteroidHitArea::Medium => {
                let radius = size.x.max(size.y).max(size.z) * 0.5;
                gizmos.sphere(
                    Isometry3d::from_translation(pos),
                    radius,
                    Color::srgb(1.0, 0.5, 0.0),
                );
            }
        }
    }

    for (tf, col) in &laser_q {
        let aabb = col.aabb(tf.translation, tf.rotation);
        let size = aabb.max - aabb.min;
        let radius = size.x.max(size.y).max(size.z) * 0.5;
        gizmos.sphere(
            Isometry3d::from_translation(tf.translation),
            radius,
            Color::srgb(1.0, 0.5, 0.0),
        );
    }
}
