use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct FireCooldown(pub f32);
