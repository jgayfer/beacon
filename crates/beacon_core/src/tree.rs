use avian2d::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(RigidBody::Static)]
pub struct Tree;
