use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct DespawnAfter(pub f32);

#[derive(Component)]
pub struct Rotating(pub Vec3);

#[derive(Component)]
pub struct RotationalVelocity(pub Vec3);
