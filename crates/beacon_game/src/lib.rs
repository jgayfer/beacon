mod balefire;
mod ground_item;
mod inventory;
mod player;
mod woodcutting;

use bevy::prelude::*;
pub use balefire::BalefirePlugin;
pub use ground_item::GroundItemPlugin;
pub use inventory::InventoryPlugin;
pub use player::PlayerPlugin;
pub use woodcutting::WoodcuttingPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            WoodcuttingPlugin,
            GroundItemPlugin,
            InventoryPlugin,
            BalefirePlugin,
        ));
    }
}
