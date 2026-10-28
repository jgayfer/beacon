use avian2d::prelude::*;
use beacon_assets::{ItemRegistry, TreeDef, TreeRegistry};
use beacon_core::{
    TILE,
    harvest::{Harvestable, Harvested},
    item::Drops,
    tree::Tree,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::*;

const TREE_STANDING: &str = "Standing";
const TREE_CHOPPED: &str = "Chopped";

pub struct TreeScenePlugin;

impl Plugin for TreeScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(show_stump);
    }
}

fn show_stump(event: On<Add<Harvested>>, mut trees: Query<&mut AseAnimation, With<Tree>>) {
    if let Ok(mut animation) = trees.get_mut(event.entity) {
        animation.animation.play_loop(TREE_CHOPPED);
    }
}

pub fn tree_scene(def: TreeDef) -> impl Scene {
    let id = def.id.clone();
    let health = def.health;
    let collider = Collider::compound(vec![(
        Vec2::Y * TILE / 2.0,
        Rotation::IDENTITY,
        Collider::circle(def.collider_radius),
    )]);
    let drop = def.drop;

    bsn! {
        Tree
        Harvestable(health)
        collider
        Sprite
        Anchor::BOTTOM_CENTER
        template(move |ctx| {
            let aseprite = ctx
                .resource::<TreeRegistry>()
                .sprite(&id)
                .unwrap_or_default();
            Ok(AseAnimation {
                aseprite,
                animation: Animation::tag(TREE_STANDING),
            })
        })
        template(move |ctx| {
            let items = ctx.resource::<ItemRegistry>();
            Ok(Drops(drop.iter().filter_map(|id| items.stack(id, 1)).collect()))
        })
    }
}
