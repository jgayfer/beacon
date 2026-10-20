mod door;

use beacon_core::{item::ItemId, solid::Solid};
use beacon_scene::{
    balefire::balefire_scene, ground_item::ground_item_scene, player::player_scene,
    tree::tree_scene,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use door::DoorPlugin;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LdtkPlugin, DoorPlugin))
            .insert_resource(LdtkSettings {
                int_grid_rendering: IntGridRendering::Invisible,
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .insert_resource(LevelSelection::Identifier("clearing".to_owned()))
            .register_ldtk_int_cell::<SolidCell>(1)
            .register_ldtk_entity_scene("player", worldly_player_scene)
            .register_ldtk_entity_scene("tree", tree_scene)
            .register_ldtk_entity_scene("axe", axe_scene)
            .register_ldtk_entity_scene("balefire", balefire_scene);
    }
}

fn worldly_player_scene() -> impl Scene {
    bsn! {
        player_scene()
        Worldly
    }
}

fn axe_scene() -> impl Scene {
    ground_item_scene(beacon_core::item::ItemStack {
        id: ItemId::Axe,
        count: 1,
    })
}

#[derive(Bundle, LdtkIntCell, Default)]
struct SolidCell {
    solid: Solid,
}

pub fn map_scene() -> impl Scene {
    bsn! {
        template(|ctx| Ok(LdtkProjectHandle::from(ctx.resource::<AssetServer>().load("map.ldtk"))))
        LevelSet
        Transform
        Visibility
    }
}
