use avian2d::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

use crate::TILE;

pub const BALEFIRE_RADIUS: f32 = TILE;
const BALEFIRE_DROP_ZONE_RADIUS: f32 = BALEFIRE_RADIUS * 1.5;

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(
    Collider::circle(BALEFIRE_DROP_ZONE_RADIUS),
    Sensor,
    CollisionEventsEnabled,
    CollidingEntities
)]
pub struct Balefire {
    pub intensity: u8,
}
