mod balefire;
mod door;
mod ground_item;
mod inventory;
mod player;
mod woodcutting;

pub use balefire::BalefirePlugin;
use bevy_app::prelude::*;
pub use ground_item::GroundItemPlugin;
pub use inventory::InventoryPlugin;
pub use player::PlayerPlugin;
pub use woodcutting::WoodcuttingPlugin;

use crate::door::DoorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            WoodcuttingPlugin,
            GroundItemPlugin,
            InventoryPlugin,
            BalefirePlugin,
            DoorPlugin,
        ));
    }
}
