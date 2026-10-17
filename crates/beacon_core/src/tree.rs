use avian2d::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

use crate::TILE;

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(RigidBody::Static, Collider::circle(TILE / 2.0))]
pub struct Tree {
    pub health: usize,
}
