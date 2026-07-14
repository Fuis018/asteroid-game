use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct PatrolAngle(pub f32);

#[derive(Component)]
pub struct Turret;

#[derive(Component, Default)]
pub struct EnemyFlags {
    pub flag_2100_triggered: bool,
    pub flag_1200_triggered: bool,
}
