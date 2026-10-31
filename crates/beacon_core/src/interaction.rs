use avian2d::collision::collider::{Collider, Sensor};
use bevy_ecs::{component::Component, entity::Entity, event::EntityEvent};

use crate::item::ItemStack;

#[derive(EntityEvent)]
pub struct Interaction {
    #[event_target]
    pub interactable: Entity,
    pub player: Entity,
    pub item: Option<ItemStack>,
}

#[derive(Component, Default, Clone)]
pub struct Interactable;

#[derive(Component, Default, Clone)]
#[require(Collider, Sensor)]
pub struct InteractionZone;
