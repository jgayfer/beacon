mod balefire;
mod door;
mod flame_sprite;
mod ground_item;
mod health;
mod inventory;
mod player;

pub use balefire::BalefirePlugin;
use beacon_core::progression::Progression;
use beacon_dialogue::DialoguePlugin;
use bevy_app::prelude::*;
pub use flame_sprite::FlameSpritePlugin;
pub use ground_item::GroundItemPlugin;
pub use health::HealthPlugin;
pub use inventory::InventoryPlugin;
pub use player::PlayerPlugin;

use crate::door::DoorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Progression>().add_plugins((
            PlayerPlugin,
            HealthPlugin,
            GroundItemPlugin,
            InventoryPlugin,
            BalefirePlugin,
            DoorPlugin,
            FlameSpritePlugin,
            DialoguePlugin,
        ));
    }
}
