use beacon_assets::ItemRegistry;
use beacon_core::item::{GroundItem, ItemStack};
use bevy::{image::TRANSPARENT_IMAGE_HANDLE, prelude::*};

pub struct GroundItemScenePlugin;

impl Plugin for GroundItemScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(insert_ground_item_sprite);
    }
}

pub fn ground_item_scene(stack: ItemStack) -> impl Scene {
    bsn! {
        GroundItem({stack})
    }
}

fn insert_ground_item_sprite(
    event: On<Add, GroundItem>,
    mut commands: Commands,
    ground_items: Query<&GroundItem>,
    items: Res<ItemRegistry>,
) {
    if let Ok(ground_item) = ground_items.get(event.entity) {
        let image = items
            .image(&ground_item.0.def.id)
            .unwrap_or(TRANSPARENT_IMAGE_HANDLE);
        commands
            .entity(event.entity)
            .insert(Sprite::from_image(image));
    }
}
