use avian2d::prelude::*;
use bevy::prelude::*;

use crate::TILE;

/// A tile that blocks movement.
#[derive(Component, Default, Clone)]
#[require(RigidBody::Static, Collider = tile_collider())]
pub struct Solid;

fn tile_collider() -> Collider {
    Collider::rectangle(TILE, TILE)
}
