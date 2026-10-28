use avian2d::prelude::*;
use beacon_assets::{ItemRegistry, TreeDef, TreeRegistry};
use beacon_core::{health::Health, item::Drops, tree::Tree};
use bevy::{image::TRANSPARENT_IMAGE_HANDLE, prelude::*};

pub fn tree_scene(def: TreeDef) -> impl Scene {
    let id = def.id.clone();
    let health = def.health;
    let collider = Collider::circle(def.collider_radius);
    let drop = def.drop;

    bsn! {
        Tree
        Health(health)
        collider
        template(move |ctx| {
            let image = ctx
                .resource::<TreeRegistry>()
                .image(&id)
                .unwrap_or(TRANSPARENT_IMAGE_HANDLE);
            Ok(Sprite::from_image(image))
        })
        template(move |ctx| {
            let items = ctx.resource::<ItemRegistry>();
            Ok(Drops(drop.iter().filter_map(|id| items.stack(id, 1)).collect()))
        })
    }
}
