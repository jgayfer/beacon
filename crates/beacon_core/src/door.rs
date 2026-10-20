use avian2d::collision::{collider::Collider, collision_events::CollisionEventsEnabled};
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

use crate::TILE;

#[derive(EntityEvent)]
pub struct DoorEntered {
    #[event_target]
    pub door: Entity,
    pub target: Entity,
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
#[require(CollisionEventsEnabled, Collider = Collider::rectangle(TILE, TILE))]
pub struct Door {
    pub exit: Option<Entity>,
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct DoorExit;
