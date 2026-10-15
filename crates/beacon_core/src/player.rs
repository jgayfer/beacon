use avian2d::prelude::*;
use beacon_engine::camera::CameraTarget;
use bevy::prelude::*;

use crate::inventory::Inventory;

const PLAYER_RADIUS: f32 = 6.0;
const INVENTORY_SIZE: usize = 9;

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(
    RigidBody::Dynamic,
    Collider::circle(PLAYER_RADIUS),
    LockedAxes::ROTATION_LOCKED,
    LinearVelocity,
    Facing,
    Action,
    CameraTarget,
    Inventory = Inventory::new(INVENTORY_SIZE)
)]
pub struct Player;

#[derive(Component, Clone, Default, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Component, Clone, Default, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub enum Action {
    #[default]
    Idle,
    Walking,
}
