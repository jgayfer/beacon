use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{LinearVelocity, LockedAxes, RigidBody},
};
use bevy::{
    ecs::template::FromTemplate,
    scene::{Scene, SceneComponent, bsn},
    sprite::Sprite,
};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

pub mod inventory;
pub mod woodcutting;

pub const PLAYER_RADIUS: f32 = 6.0;
pub const TILE: f32 = 16.0;

pub const SOLDIER_IDLE: &str = "Idle";
pub const SOLDIER_WALK: &str = "Walk";

#[derive(SceneComponent, Clone, Default)]
#[require(
    RigidBody::Dynamic,
    Collider::circle(PLAYER_RADIUS),
    LockedAxes::ROTATION_LOCKED,
    LinearVelocity
)]
pub struct Player;

impl Player {
    fn scene() -> impl Scene {
        bsn! {
            Sprite
            AseAnimation {
                aseprite: "Soldier.aseprite",
                animation: Animation::tag(SOLDIER_IDLE),
            }
        }
    }
}

#[derive(SceneComponent, FromTemplate)]
#[require(RigidBody::Static, Collider::circle(TILE / 2.0) )]
pub struct Tree {
    pub health: usize,
}

impl Tree {
    fn scene() -> impl Scene {
        bsn! {
            Sprite {
                image: "arbutus.png",
            }
        }
    }
}
