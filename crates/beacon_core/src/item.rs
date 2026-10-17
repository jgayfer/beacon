use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::CollisionEventsEnabled,
};
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_transform::prelude::*;

#[derive(Component, Clone, Default)]
#[require(Transform, Collider::circle(8.0), Sensor, CollisionEventsEnabled)]
pub struct GroundItem(pub ItemStack);

#[derive(EntityEvent)]
pub struct GroundItemTouched(pub Entity);

#[derive(EntityEvent)]
pub struct ItemUsed {
    #[event_target]
    pub player: Entity,
    pub item: ItemStack,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, Reflect)]
pub enum ItemId {
    #[default]
    Wood,
    Axe,
}

impl ItemId {
    pub fn def(self) -> &'static ItemDef {
        match self {
            ItemId::Wood => &WOOD,
            ItemId::Axe => &AXE,
        }
    }
}

static WOOD: ItemDef = ItemDef {
    name: "wood",
    max_stack: 64,
    sprite: "log.png",
    kind: ItemKind::Resource,
};

static AXE: ItemDef = ItemDef {
    name: "axe",
    max_stack: 1,
    sprite: "axe.png",
    kind: ItemKind::Tool,
};

pub struct ItemDef {
    pub name: &'static str,
    pub max_stack: u16,
    pub sprite: &'static str,
    pub kind: ItemKind,
}

pub enum ItemKind {
    Resource,
    Tool,
}

#[derive(Clone, Default, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
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
