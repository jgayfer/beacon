pub mod balefire;
pub mod dialogue_box;
pub mod flame_sprite;
pub mod ground_item;
pub mod hotbar;
pub mod player;
pub mod tree;

use balefire::BalefireScenePlugin;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteUltraPlugin;
use dialogue_box::DialogueBoxPlugin;
use hotbar::HotbarPlugin;
use player::PlayerScenePlugin;
use tree::TreeScenePlugin;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AsepriteUltraPlugin,
            PlayerScenePlugin,
            TreeScenePlugin,
            HotbarPlugin,
            BalefireScenePlugin,
            DialogueBoxPlugin,
        ));
    }
}
