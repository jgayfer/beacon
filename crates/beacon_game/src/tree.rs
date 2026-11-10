use avian2d::prelude::*;
use beacon_core::{
    harvest::{Harvestable, Harvested},
    item::{Drops, ItemId, ItemSpec},
};
use beacon_engine::hitbox::Hurtbox;
use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const TREE_STANDING: &str = "Standing";
const TREE_CHOPPED: &str = "Chopped";

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_scene("oak", oak_tree)
            .register_ldtk_entity_scene("arbutus", arbutus_tree)
            .add_observer(show_stump);
    }
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(RigidBody::Static)]
struct Tree;

fn show_stump(event: On<Add<Harvested>>, mut trees: Query<&mut AseAnimation, With<Tree>>) {
    if let Ok(mut animation) = trees.get_mut(event.entity) {
        animation.animation.play_loop(TREE_CHOPPED);
    }
}

fn oak_tree() -> impl Scene {
    bsn! {
        Tree
        Harvestable(5)
        Drops(
            vec![
                ItemSpec { id: ItemId::new("wood"), count: 1 }
            ]
        )
        Sprite
        Anchor::BOTTOM_CENTER
        AseAnimation {
            aseprite: "trees/oak.aseprite",
            animation: Animation::tag(TREE_STANDING)
        }
        Children [
            Hurtbox
            Collider::capsule(3.0, 6.0)
            Transform::from_xyz(0.0, 6.0, 0.0)
            --
            Collider::circle(4.0)
            Transform::from_xyz(0.0, 6.0, 0.0)
        ]
    }
}

fn arbutus_tree() -> impl Scene {
    bsn! {
        Tree
        Harvestable(5)
        Drops(
            vec![
                ItemSpec { id: ItemId::new("wood"), count: 1 }
            ]
        )
        Sprite
        Anchor::BOTTOM_CENTER
        AseAnimation {
            aseprite: "trees/arbutus.aseprite",
            animation: Animation::tag(TREE_STANDING)
        }
        Children [
            Hurtbox
            Collider::capsule(3.0, 3.0)
            Transform::from_xyz(0.0, 6.0, 0.0)
            --
            Collider::circle(4.0)
            Transform::from_xyz(0.0, 6.0, 0.0)
        ]
    }
}
