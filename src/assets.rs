use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "spaceship.glb#Scene0")]
    pub player_ship: Handle<WorldAsset>,
    #[asset(path = "Freighter_Arion.glb#Scene0")]
    pub enemy_ship: Handle<WorldAsset>,
}
