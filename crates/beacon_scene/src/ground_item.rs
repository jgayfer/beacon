use beacon_core::item::{GroundItem, ItemStack};
use bevy::prelude::*;

pub fn ground_item_scene(stack: ItemStack) -> impl Scene {
    let sprite = stack.def.sprite.clone();
    bsn! {
        GroundItem({stack})
        Sprite {
            image: {sprite},
        }
    }
}
