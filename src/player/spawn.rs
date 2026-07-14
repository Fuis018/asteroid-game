use avian3d::prelude::*;
use bevy::prelude::*;
use crate::assets::GameAssets;
use crate::components::*;
use crate::constants::*;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands.spawn((
        WorldAssetRoot(assets.player_ship.clone()),
        Transform::from_xyz(0.0, 7.0, 30.0),
        Player,
        Health(PLAYER_MAX_HEALTH),
        FireCooldown(0.0),
        Speed(PLAYER_MIN_SPEED),
        RotationalVelocity(Vec3::ZERO),
        RigidBody::Kinematic,
        Collider::cuboid(4.5, 3.0, 10.0),
        CollisionEventsEnabled,
    ));
}
