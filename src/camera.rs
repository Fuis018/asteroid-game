use bevy::prelude::*;

use crate::components::Player;
use crate::constants::*;

#[derive(Component)]
pub struct GameCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        AmbientLight {
            brightness: 500.0,
            ..default()
        },
        GameCamera,
        Transform::from_xyz(0.0, CAMERA_HEIGHT, CAMERA_OFFSET)
            .looking_at(Vec3::new(0.0, 0.0, CAMERA_OFFSET - 30.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 0.3, 0.0)),
    ));
}

pub fn camera_follow_player(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, Entity), (With<GameCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut camera_transform, _) in &mut camera_query {
        let offset = player_transform.rotation * Vec3::new(0.0, CAMERA_HEIGHT, CAMERA_OFFSET);
        let target_position = player_transform.translation + offset;

        camera_transform.translation = camera_transform
            .translation
            .lerp(target_position, CAMERA_SMOOTH_SPEED * time.delta_secs());

        let forward = player_transform.forward().as_vec3();
        let look_target = player_transform.translation + forward * 40.0;
        camera_transform.look_at(look_target, Vec3::Y);
    }
}
