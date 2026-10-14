use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::CollisionEventsEnabled,
};
use bevy::{
    ecs::{component::Component, entity::Entity, event::EntityEvent},
    transform::components::Transform,
};

#[derive(Clone, Default)]
pub enum Item {
    #[default]
    Wood,
}

#[derive(Component, Clone, Default)]
#[require(Transform, Collider::circle(8.0), Sensor, CollisionEventsEnabled)]
pub struct GroundItem(pub Item);

#[derive(EntityEvent)]
pub struct GroundItemTouched(pub Entity);
