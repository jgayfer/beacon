mod door;

use beacon_core::{item::ItemSpawn, solid::Solid};
use beacon_scene::{
    balefire::balefire_scene, flame_sprite::flame_sprite_scene, player::player_scene,
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
            .register_ldtk_entity_scene("item_spawn", item_spawn)
            .register_ldtk_entity_scene("flame_sprite", flame_sprite_scene)
            .register_ldtk_entity_scene("balefire", balefire_scene);
    }
}

fn worldly_player_scene() -> impl Scene {
    bsn! {
        player_scene()
        Worldly
    }
}

fn item_spawn(ctx: &EntityInstanceContext) -> impl Scene + use<> {
    let id = ctx
        .entity_instance
        .get_string_field("item_id")
        .expect("item_spawn entity's string 'item_id' field is present.")
        .clone();

    let count: u16 = (*ctx
        .entity_instance
        .get_int_field("count")
        .expect("item_spawn entity's int 'count' field is present."))
    .try_into()
    .expect("count is a valid u16");

    bsn! {
        ItemSpawn {
            id,
            count,
        }
    }
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
