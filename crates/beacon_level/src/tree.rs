use beacon_assets::TreeRegistry;
use beacon_scene::tree::tree_scene;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const TREE_TAG: &str = "tree";

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_tree);
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
        warn!("trees.ron doesn't define {:?}", instance.identifier);
        return;
    };

    commands
        .entity(event.entity)
        .apply_scene(tree_scene(def.clone()));
}
