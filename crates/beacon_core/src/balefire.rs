use avian2d::prelude::*;
use bevy_ecs::{lifecycle::HookContext, prelude::*, world::DeferredWorld};
use bevy_math::Vec2;
use bevy_reflect::prelude::*;

use crate::{
    TILE,
    interaction::{Interactable, attach_interaction_zone},
};

pub const BALEFIRE_RADIUS: f32 = TILE;
const BALEFIRE_DROP_ZONE_RADIUS: f32 = BALEFIRE_RADIUS * 1.5;

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(RigidBody::Static, Collider::circle(BALEFIRE_RADIUS), Interactable)]
#[component(on_add = on_balefire_add)]
pub struct Balefire {
    pub intensity: u8,
}

fn on_balefire_add(mut world: DeferredWorld, ctx: HookContext) {
    attach_interaction_zone(
        &mut world,
        ctx.entity,
        Collider::circle(BALEFIRE_DROP_ZONE_RADIUS),
        Vec2::ZERO,
    );
}
