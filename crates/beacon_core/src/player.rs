use avian2d::prelude::*;
use beacon_engine::camera::CameraTarget;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

use crate::inventory::{Inventory, SelectedSlot};

const PLAYER_RADIUS: f32 = 6.0;
const INVENTORY_SIZE: usize = 9;

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(
    RigidBody::Dynamic,
    Collider::circle(PLAYER_RADIUS),
    CollidingEntities,
    LockedAxes::ROTATION_LOCKED,
    LinearVelocity,
    Facing,
    Action,
    CameraTarget,
    Inventory = Inventory::new(INVENTORY_SIZE),
    SelectedSlot,
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

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerSpawn;
