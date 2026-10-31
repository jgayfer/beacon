use avian2d::prelude::*;
use beacon_core::{
    TILE,
    dialogue::Dialogue,
    interaction::{Interactable, InteractionZone},
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct FlameSpritePlugin;

impl Plugin for FlameSpritePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_scene("flame_sprite", flame_sprite_scene);
    }
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(Interactable, RigidBody::Static)]
struct FlameSprite;

fn flame_sprite_scene() -> impl Scene {
    bsn! {
        FlameSprite
        Collider::circle(TILE / 2.0)
        Dialogue("npc/flame_sprite/dialogue.rep")
        Sprite {
            image: "npc/flame_sprite/sprite.aseprite",
        }
        Children [
            InteractionZone
            Collider::circle(TILE * 1.5)
        ]
    }
}
