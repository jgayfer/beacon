use bevy::prelude::*;

use crate::item::ItemStack;

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    slots: Vec<Option<ItemStack>>,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
        }
    }

    /// The slots contained by this [`Inventory`].
    pub fn slots(&self) -> &[Option<ItemStack>] {
        &self.slots
    }

    /// Insert an [`ItemStack`] into the [`Inventory`].
    ///
    /// Returns whatever didn't fit, or [`None`] if everything was inserted.
    pub fn insert(&mut self, mut item_stack: ItemStack) -> Option<ItemStack> {
        for slot in self.slots.iter_mut().flatten() {
            item_stack = match slot.merge(item_stack) {
                Ok(None) => return None,
                Ok(Some(rest)) | Err(rest) => rest,
            }
        }

        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(item_stack);
            return None;
        }

        Some(item_stack)
    }

    /// Get the [`ItemStack`] at the given index, if one exists.
    pub fn get(&self, index: usize) -> Option<&ItemStack> {
        self.slots.get(index)?.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::ItemId;

    fn wood(count: u16) -> ItemStack {
        ItemStack {
            id: ItemId::Wood,
            count,
        }
    }

    fn wood_max_stack() -> u16 {
        ItemId::Wood.def().max_stack
    }

    #[test]
    fn insert_uses_first_free_slot() {
        let mut inventory = Inventory::new(1);

        let leftover = inventory.insert(wood(1));

        assert_eq!(leftover, None);
        assert_eq!(inventory.get(0), Some(&wood(1)));
    }

    #[test]
    fn insert_tops_up_existing_stack_before_using_new_slot() {
        let mut inventory = Inventory::new(3);
        let initial_count = wood_max_stack() - 10;

        inventory.insert(wood(initial_count));
        let leftover = inventory.insert(wood(15));

        assert_eq!(leftover, None);
        assert_eq!(inventory.get(0), Some(&wood(wood_max_stack())));
        assert_eq!(inventory.get(1), Some(&wood(5)));
    }

    #[test]
    fn insert_returns_leftover_when_full() {
        let mut inventory = Inventory::new(1);
        let first = wood_max_stack() - 10;

        inventory.insert(wood(first));
        let leftover = inventory.insert(wood(15));

        assert_eq!(leftover, Some(wood(5)));
        assert_eq!(inventory.get(0), Some(&wood(wood_max_stack())));
    }
}
