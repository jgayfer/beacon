use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy_ecs::{lifecycle::HookContext, prelude::*, world::DeferredWorld};
use bevy_math::Vec2;
use bevy_reflect::Reflect;

use crate::{
    TILE,
    interaction::{Interactable, attach_interaction_zone},
};

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(Interactable, RigidBody::Static, Collider::circle(TILE / 2.0))]
#[component(on_add = on_flame_sprite_add)]
pub struct FlameSprite;

fn on_flame_sprite_add(mut world: DeferredWorld, ctx: HookContext) {
    attach_interaction_zone(
        &mut world,
        ctx.entity,
        Collider::circle(TILE * 1.5),
        Vec2::ZERO,
    );
}
