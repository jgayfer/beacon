pub mod balefire;
pub mod ground_item;
pub mod hotbar;
pub mod player;
pub mod tree;

use bevy::prelude::*;
use balefire::BalefireScenePlugin;
use bevy_aseprite_ultra::prelude::AsepriteUltraPlugin;
use hotbar::HotbarPlugin;
use player::PlayerScenePlugin;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AsepriteUltraPlugin,
            PlayerScenePlugin,
            HotbarPlugin,
            BalefireScenePlugin,
        ));
    }
}
