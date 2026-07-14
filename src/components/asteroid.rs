use bevy::prelude::*;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct AsteroidHealth(pub f32);

#[derive(Component)]
pub struct AsteroidPiece;

#[derive(Component)]
pub struct AsteroidFragment;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum AsteroidGroup {
    Core,
    Medium(usize),
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum AsteroidHitArea {
    Core,
    Medium,
}

#[derive(Component)]
pub struct AsteroidRespawn(pub Timer);
