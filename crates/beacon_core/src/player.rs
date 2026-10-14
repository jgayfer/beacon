use avian2d::prelude::*;
use beacon_engine::camera::CameraTarget;
use bevy::prelude::*;

const PLAYER_RADIUS: f32 = 6.0;

#[derive(Component, Clone, Default)]
#[require(
    RigidBody::Dynamic,
    Collider::circle(PLAYER_RADIUS),
    LockedAxes::ROTATION_LOCKED,
    LinearVelocity,
    Facing,
    Action,
    CameraTarget
)]
pub struct Player;

#[derive(Component, Clone, Default, Copy, PartialEq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Component, Clone, Default, Copy, PartialEq)]
pub enum Action {
    #[default]
    Idle,
    Walking,
}
