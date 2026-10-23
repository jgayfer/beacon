mod aseprite;
pub mod balefire;
pub mod dialogue_box;
pub mod flame_sprite;
pub mod ground_item;
pub mod hotbar;
pub mod player;
pub mod tree;

use aseprite::AsepriteImageLoader;
use balefire::BalefireScenePlugin;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteUltraPlugin;
use dialogue_box::DialogueBoxPlugin;
use ground_item::GroundItemScenePlugin;
use hotbar::HotbarPlugin;
use player::PlayerScenePlugin;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(AsepriteImageLoader).add_plugins((
            AsepriteUltraPlugin,
            PlayerScenePlugin,
            HotbarPlugin,
            GroundItemScenePlugin,
            BalefireScenePlugin,
            DialogueBoxPlugin,
        ));
    }
}
