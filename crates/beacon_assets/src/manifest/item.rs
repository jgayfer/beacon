use beacon_core::item::{ItemDef, ItemId, ItemStack};

use bevy::prelude::*;

use super::{ManifestEntry, Registry};

impl ManifestEntry for ItemDef {
    const DIR: &'static str = "items";

    fn id(&self) -> &str {
        self.id.get()
    }
}

pub type ItemRegistry = Registry<ItemDef>;

impl ItemRegistry {
    pub fn stack(&self, id: &ItemId, count: u16) -> Option<ItemStack> {
        let def = self.get(id.get())?.clone();
        Some(ItemStack { def, count })
    }

    pub fn item_image(&self, id: &ItemId) -> Option<Handle<Image>> {
        self.image(id.get())
    }
}
