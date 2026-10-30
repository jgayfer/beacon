use avian2d::prelude::*;
use beacon_core::{
    TILE,
    harvest::{Harvestable, Harvested},
    item::{Drops, ItemDef, ItemId},
    state::GameState,
};
use beacon_engine::manifest::{ManifestEntry, RegisterManifest, Registry};
use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use serde::Deserialize;

const TREE_STANDING: &str = "Standing";
const TREE_CHOPPED: &str = "Chopped";
const TREE_TAG: &str = "tree";

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.register_manifest::<TreeDef>(GameState::Loading)
            .add_observer(show_stump)
            .add_observer(spawn_tree);
    }
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(RigidBody::Static)]
struct Tree;

#[derive(Deserialize, Clone, Debug, Default, PartialEq, Reflect)]
struct TreeDef {
    id: String,
    health: usize,
    collider_radius: f32,
    #[serde(default)]
    drop: Option<ItemId>,
}

impl ManifestEntry for TreeDef {
    const DIR: &'static str = "trees";
    type Sprite = Aseprite;

    fn id(&self) -> &str {
        &self.id
    }
}

type TreeRegistry = Registry<TreeDef>;

fn show_stump(event: On<Add<Harvested>>, mut trees: Query<&mut AseAnimation, With<Tree>>) {
    if let Ok(mut animation) = trees.get_mut(event.entity) {
        animation.animation.play_loop(TREE_CHOPPED);
    }
}

fn tree_scene(def: TreeDef) -> impl Scene {
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
            let items = ctx.resource::<Registry<ItemDef>>();
            Ok(Drops(
                drop.iter()
                    .filter_map(|id| items.get(id.get()))
                    .map(|def| def.stack(1))
                    .collect(),
            ))
        })
    }
}

fn spawn_tree(
    event: On<Add<EntityInstance>>,
    mut commands: Commands,
    instances: Query<&EntityInstance>,
    trees: Res<TreeRegistry>,
) {
    let Ok(instance) = instances.get(event.entity) else {
        return;
    };

    if !instance.tags.iter().any(|tag| tag == TREE_TAG) {
        return;
    }

    let Some(def) = trees.get(&instance.identifier) else {
        warn!(
            "trees/manifest.ron doesn't define {:?}",
            instance.identifier
        );
        return;
    };

    commands
        .entity(event.entity)
        .apply_scene(tree_scene(def.clone()));
}
