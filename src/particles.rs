use bevy::prelude::*;
use rand::Rng;
use rand::seq::IndexedRandom;

const EXPLOSION_PARTICLE_COUNT: u32 = 20;
const EXPLOSION_LIFETIME: f32 = 0.6;
const EXPLOSION_SPEED: f32 = 15.0;
const EXPLOSION_MIN_RADIUS: f32 = 0.06;
const EXPLOSION_MAX_RADIUS: f32 = 0.18;

#[derive(Component)]
pub struct ExplosionParticle {
    pub life: f32,
    pub max_life: f32,
    pub initial_scale: f32,
}

pub fn spawn_explosion_at(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    base_color: Color,
    scale: f32,
) {
    let mut rng = rand::rng();

    let hot_colors = [
        Color::srgb(1.0, 0.8, 0.2),
        Color::srgb(1.0, 0.5, 0.1),
        Color::srgb(1.0, 0.3, 0.05),
        Color::srgb(1.0, 0.6, 0.15),
        base_color,
    ];

    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(1).unwrap());

    for _ in 0..EXPLOSION_PARTICLE_COUNT {
        let radius = rng.random_range(EXPLOSION_MIN_RADIUS..EXPLOSION_MAX_RADIUS) * scale;
        let color = *hot_colors.choose(&mut rng).unwrap();

        let mat = materials.add(StandardMaterial {
            base_color: color,
            emissive: color.into(),
            unlit: true,
            ..default()
        });

        let dir = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        )
        .normalize_or_zero();

        let speed = rng.random_range(EXPLOSION_SPEED * 0.5..EXPLOSION_SPEED) * scale;
        let life = rng.random_range(EXPLOSION_LIFETIME * 0.6..EXPLOSION_LIFETIME);

        let initial_scale = radius;

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(mat),
            Transform {
                translation: position,
                scale: Vec3::splat(initial_scale),
                ..default()
            },
            ExplosionParticle {
                life,
                max_life: life,
                initial_scale,
            },
            ParticleVelocity(dir * speed),
        ));
    }
}

#[derive(Component)]
pub(crate) struct ParticleVelocity(pub Vec3);

pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut ExplosionParticle, &ParticleVelocity)>,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, mut particle, velocity) in &mut query {
        particle.life -= dt;

        if particle.life <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        transform.translation += velocity.0 * dt;

        let t = particle.life / particle.max_life;
        let scale = particle.initial_scale * t;
        transform.scale = Vec3::splat(scale);
    }
}
