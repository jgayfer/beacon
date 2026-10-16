pub mod balefire;
pub mod hotbar;
pub mod map;
pub mod player;
pub mod tree;

use bevy::prelude::*;
use balefire::BalefireScenePlugin;
use bevy_aseprite_ultra::prelude::AsepriteUltraPlugin;
use hotbar::HotbarPlugin;
use map::MapPlugin;
use player::PlayerScenePlugin;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AsepriteUltraPlugin,
            MapPlugin,
            PlayerScenePlugin,
            HotbarPlugin,
            BalefireScenePlugin,
        ));
    }
}
