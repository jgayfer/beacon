use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::CollisionEventsEnabled,
};
use beacon_engine::manifest::ManifestEntry;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_reflect::prelude::*;
use bevy_transform::prelude::*;
use serde::Deserialize;

#[derive(Component, Clone, Default)]
#[require(Transform, Collider::circle(8.0), Sensor, CollisionEventsEnabled)]
pub struct GroundItem(pub ItemStack);

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Drops(pub Vec<ItemStack>);

#[derive(Deserialize, Clone, Debug, Default, PartialEq, Eq, Hash, Reflect)]
#[serde(transparent)]
pub struct ItemId(String);

impl ItemId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for ItemId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct ItemDef {
    pub id: ItemId,
    pub max_stack: u16,
    pub kind: ItemKind,
}

impl ItemDef {
    pub fn stack(&self, count: u16) -> ItemStack {
        ItemStack {
            def: self.clone(),
            count,
        }
    }
}

impl ManifestEntry for ItemDef {
    const DIR: &'static str = "items";
    type Sprite = Image;

    fn id(&self) -> &str {
        self.id.get()
    }
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
