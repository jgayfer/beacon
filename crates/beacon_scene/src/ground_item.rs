use beacon_assets::ItemRegistry;
use beacon_core::item::{GroundItem, ItemStack};
use bevy::{image::TRANSPARENT_IMAGE_HANDLE, prelude::*};

static GROUND_ITEM_SIZE: f32 = 12.0;

pub fn ground_item_scene(stack: ItemStack) -> impl Scene {
    let item_id = stack.def.id.clone();
    bsn! {
        GroundItem({stack})
        template(move |ctx| {
            let image = ctx
                .resource::<ItemRegistry>()
                .item_image(&item_id)
                .unwrap_or(TRANSPARENT_IMAGE_HANDLE);
            Ok(Sprite {
                image,
                custom_size: Some(Vec2::splat(GROUND_ITEM_SIZE)),
                ..Default::default()
            })
        })
    }
}
