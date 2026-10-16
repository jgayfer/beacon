use beacon_core::item::{GroundItem, ItemStack};
use bevy::prelude::*;

pub fn ground_item_scene(stack: ItemStack) -> impl Scene {
    bsn! {
        GroundItem({stack})
        Sprite {
            image: {stack.id.def().sprite},
        }
    }
}
