use beacon_core::item::{GroundItem, ItemId, ItemStack};
use beacon_engine::hitbox::Hit;
use bevy::prelude::*;

use beacon_core::tree::Tree;

pub struct WoodcuttingPlugin;

impl Plugin for WoodcuttingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_tree_health)
            .add_observer(on_tree_hit);
    }
}

fn on_tree_hit(event: On<Hit>, mut trees: Query<&mut Tree>) {
    if let Ok(mut tree) = trees.get_mut(event.target) {
        info!("Tree hit");
        tree.health = tree.health.saturating_sub(1);
    }
}

fn check_tree_health(
    mut commands: Commands,
    query: Query<(Entity, &Tree, &GlobalTransform), Changed<Tree>>,
) {
    for (entity, tree, tree_transform) in query {
        if tree.health == 0 {
            commands.entity(entity).despawn();
            commands.spawn_scene(bsn! {
                GroundItem(ItemStack { id: ItemId::Wood, count: 1 })
                Transform { translation: {tree_transform.translation()} }
                Sprite {
                    image: "log.png",
                }
            });
        }
    }
}
