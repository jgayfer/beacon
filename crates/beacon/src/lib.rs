use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::{
    ecs::template::FromTemplate,
    scene::{Scene, SceneComponent, bsn},
    sprite::Sprite,
};

pub mod woodcutting;

pub const PLAYER_RADIUS: f32 = 6.0;
pub const TILE: f32 = 16.0;

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
