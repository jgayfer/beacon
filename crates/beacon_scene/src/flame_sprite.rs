use beacon_core::npc::FlameSprite;
use bevy::{scene::prelude::*, sprite::Sprite};

pub fn flame_sprite_scene() -> impl Scene {
    bsn! {
        FlameSprite
        Sprite {
            image: "characters/flame_sprite.aseprite",
        }
    }
}
