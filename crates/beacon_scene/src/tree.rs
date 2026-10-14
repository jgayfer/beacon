use beacon_core::tree::Tree;
use bevy::prelude::*;

pub fn tree_scene() -> impl Scene {
    bsn! {
        Tree { health: 3 }
        Sprite {
            image: "arbutus.png",
        }
    }
}
