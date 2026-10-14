use avian2d::prelude::*;
use bevy::prelude::*;

use crate::TILE;

#[derive(Component, Clone, Default)]
#[require(RigidBody::Static, Collider::circle(TILE / 2.0))]
pub struct Tree {
    pub health: usize,
}
