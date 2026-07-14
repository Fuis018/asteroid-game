use bevy::prelude::*;
use rand::Rng;

const STAR_COUNT: usize = 1500;
const STAR_FIELD_RADIUS: f32 = 2000.0;
const STAR_MIN_RADIUS: f32 = 0.05;
const STAR_MAX_RADIUS: f32 = 0.2;

#[derive(Component)]
pub struct Star;

pub fn spawn_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();

    let star_mesh = meshes.add(Sphere::new(1.0));
    let star_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::new(1.5, 1.5, 1.5, 1.0),
        unlit: true,
        ..default()
    });

    for _ in 0..STAR_COUNT {
        let theta: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        let phi: f32 = rng.random_range(0.0..std::f32::consts::PI);
        let r: f32 = rng.random_range(STAR_FIELD_RADIUS * 0.3..STAR_FIELD_RADIUS);

        let x = r * phi.sin() * theta.cos();
        let y = r * phi.sin() * theta.sin();
        let z = r * phi.cos();

        let scale = rng.random_range(STAR_MIN_RADIUS..STAR_MAX_RADIUS);

        commands.spawn((
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(star_material.clone()),
            Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
            Star,
        ));
    }
}
