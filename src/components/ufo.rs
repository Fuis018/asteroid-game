use bevy::prelude::*;

#[derive(Component)]
pub struct Ufo;

#[derive(Component)]
pub struct UfoLaser;

#[derive(Component)]
pub struct UfoHitRecover(pub Timer);

#[derive(Component)]
pub struct UfoTurret;
