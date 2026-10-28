use beacon_assets::ItemRegistry;
use beacon_core::item::{GroundItem, ItemStack};
use bevy::{image::TRANSPARENT_IMAGE_HANDLE, prelude::*};

pub fn ground_item_scene(stack: ItemStack) -> impl Scene {
    let item_id = stack.def.id.clone();
    bsn! {
        GroundItem({stack})
        template(move |ctx| {
            let image = ctx
                .resource::<ItemRegistry>()
                .item_image(&item_id)
                .unwrap_or(TRANSPARENT_IMAGE_HANDLE);
            Ok(Sprite::from_image(image))
        })
    }
}
