mod balefire;
mod dialogue;
mod door;
mod flame_sprite;
mod ground_item;
mod inventory;
mod player;
mod woodcutting;

pub use balefire::BalefirePlugin;
use beacon_core::progression::Progression;
pub use dialogue::DialoguePlugin;
pub use flame_sprite::FlameSpritePlugin;
use bevy_app::prelude::*;
pub use ground_item::GroundItemPlugin;
pub use inventory::InventoryPlugin;
pub use player::PlayerPlugin;
pub use woodcutting::WoodcuttingPlugin;

use crate::door::DoorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Progression>().add_plugins((
            PlayerPlugin,
            WoodcuttingPlugin,
            GroundItemPlugin,
            InventoryPlugin,
            BalefirePlugin,
            DoorPlugin,
            FlameSpritePlugin,
            DialoguePlugin,
        ));
    }
}
