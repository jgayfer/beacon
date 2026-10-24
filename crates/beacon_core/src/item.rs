use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::CollisionEventsEnabled,
};
use bevy_camera::visibility::Visibility;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_transform::prelude::*;
use serde::Deserialize;

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

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(Visibility)]
pub struct ItemSpawn {
    pub id: String,
    pub count: u16,
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct ItemDef {
    pub id: String,
    pub max_stack: u16,
    pub kind: ItemKind,
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub enum ItemKind {
    #[default]
    Resource,
    Tool,
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Reflect)]
pub struct ItemStack {
    pub def: ItemDef,
    pub count: u16,
}

impl ItemStack {
    fn remaining_capacity(&self) -> u16 {
        self.def.max_stack - self.count
    }

    pub fn merge(&mut self, mut other: ItemStack) -> Result<Option<ItemStack>, ItemStack> {
        if self.def.id != other.def.id {
            return Err(other);
        }
        let amount = other.count.min(self.remaining_capacity());
        self.count += amount;
        other.count -= amount;
        Ok((other.count > 0).then_some(other))
    }
}
