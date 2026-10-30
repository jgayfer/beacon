use avian2d::prelude::*;
use beacon_core::{
    TILE,
    dialogue::Dialogue,
    interaction::{Interactable, attach_interaction_zone},
};
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_ecs_ldtk::prelude::*;

pub struct FlameSpritePlugin;

impl Plugin for FlameSpritePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_scene("flame_sprite", flame_sprite_scene);
    }
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(Interactable, RigidBody::Static, Collider::circle(TILE / 2.0))]
#[component(on_add = on_flame_sprite_add)]
struct FlameSprite;

fn on_flame_sprite_add(mut world: DeferredWorld, ctx: HookContext) {
    attach_interaction_zone(
        &mut world,
        ctx.entity,
        Collider::circle(TILE * 1.5),
        Vec2::ZERO,
    );
}

fn flame_sprite_scene() -> impl Scene {
    bsn! {
        FlameSprite
        Dialogue("npc/flame_sprite/dialogue.rep")
        Sprite {
            image: "npc/flame_sprite/sprite.aseprite",
        }
    }
}
