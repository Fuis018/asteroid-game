use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

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
pub struct PlayerLaser;

#[derive(Component)]
pub struct EnemyLaser;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct FireCooldown(pub f32);

#[derive(Component)]
pub struct DespawnAfter(pub f32);

#[derive(Component)]
pub struct Rotating(pub Vec3);

#[derive(Component)]
pub struct PatrolAngle(pub f32);

#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Ufo;

#[derive(Component)]
pub struct UfoLaser;

#[derive(Component)]
pub struct UfoHitRecover(pub Timer);

#[derive(Component)]
pub struct UfoTurret;

#[derive(Component)]
pub struct AsteroidRespawn(pub Timer);

#[derive(Component)]
pub struct RotationalVelocity(pub Vec3);

#[derive(Component, Default)]
pub struct EnemyFlags {
    pub flag_2100_triggered: bool,
    pub flag_1200_triggered: bool,
}
