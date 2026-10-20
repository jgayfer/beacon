use avian2d::collision::collider::{Collider, Sensor};
use bevy_ecs::{component::Component, entity::Entity, event::EntityEvent, world::DeferredWorld};
use bevy_math::Vec2;
use bevy_transform::components::Transform;

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

pub fn attach_interaction_zone(
    world: &mut DeferredWorld,
    parent: Entity,
    collider: Collider,
    offset: Vec2,
) {
    world.commands().entity(parent).with_child((
        InteractionZone,
        collider,
        Transform::from_translation(offset.extend(0.0)),
    ));
}
