use bevy_ecs::{component::Component, entity::Entity, event::EntityEvent};

use crate::item::ItemStack;

#[derive(EntityEvent)]
pub struct Interaction {
    #[event_target]
    pub target: Entity,
    pub player: Entity,
    pub item: Option<ItemStack>,
}

#[derive(Component, Default, Clone)]
pub struct Interactable;
