pub mod beacon;
pub mod map;
pub mod player;
pub mod tree;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteUltraPlugin;
use bevy_light_2d::plugin::Light2dPlugin;
use map::MapPlugin;
use player::PlayerScenePlugin;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AsepriteUltraPlugin,
            Light2dPlugin,
            MapPlugin,
            PlayerScenePlugin,
        ));
    }
}
