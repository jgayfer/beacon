use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy_ecs::component::Component;

use crate::{TILE, interaction::Interactable};

#[derive(Component, Clone, Default)]
#[require(Interactable)]
pub struct Npc;

#[derive(Component, Clone, Default)]
#[require(Interactable, RigidBody::Static, Collider::circle(TILE / 2.0))]
pub struct FlameSprite;
