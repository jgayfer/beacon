use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::CollisionEventsEnabled,
};
use bevy::{
    ecs::{component::Component, entity::Entity, event::EntityEvent},
    transform::components::Transform,
};

#[derive(Component, Clone, Default)]
#[require(Transform, Collider::circle(8.0), Sensor, CollisionEventsEnabled)]
pub struct GroundItem(pub ItemStack);

#[derive(EntityEvent)]
pub struct GroundItemTouched(pub Entity);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum ItemId {
    #[default]
    Wood,
}

impl ItemId {
    pub fn def(self) -> &'static ItemDef {
        match self {
            ItemId::Wood => &WOOD,
        }
    }
}

static WOOD: ItemDef = ItemDef {
    name: "wood",
    max_stack: 64,
};

pub struct ItemDef {
    pub name: &'static str,
    pub max_stack: u16,
}

#[derive(Clone, Default, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ItemStack {
    pub id: ItemId,
    pub count: u16,
}

impl ItemStack {
    fn remaining_capacity(&self) -> u16 {
        self.id.def().max_stack - self.count
    }

    pub fn merge(&mut self, mut other: ItemStack) -> Result<Option<ItemStack>, ItemStack> {
        if self.id != other.id {
            return Err(other);
        }
        let amount = other.count.min(self.remaining_capacity());
        self.count += amount;
        other.count -= amount;
        Ok((other.count > 0).then_some(other))
    }
}
