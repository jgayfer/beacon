use beacon_core::{item::Items, state::GameState};
use beacon_engine::hitbox::Hit;
use beacon_scene::ground_item::ground_item_scene;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_scene::prelude::*;
use bevy_state::prelude::*;
use bevy_transform::prelude::*;

use beacon_core::tree::Tree;

pub struct WoodcuttingPlugin;

impl Plugin for WoodcuttingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_tree_health.run_if(in_state(GameState::Playing)),
        )
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
    items: Res<Items>,
    query: Query<(Entity, &Tree, &GlobalTransform), Changed<Tree>>,
) {
    for (entity, tree, tree_transform) in query {
        if tree.health == 0 {
            let wood = items.stack("wood", 48).expect("items.ron defines wood");
            commands.entity(entity).despawn();
            commands.spawn_scene(bsn! {
                ground_item_scene(wood)
                Transform { translation: {tree_transform.translation()} }
            });
        }
    }
}
