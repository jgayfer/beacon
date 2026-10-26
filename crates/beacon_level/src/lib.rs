mod door;

use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use beacon_core::{
    TILE,
    item::ItemSpawn,
    tile::{self, Solid},
};
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
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .insert_resource(LevelSelection::Identifier("clearing".to_owned()))
            .add_systems(Update, (add_tile_flag_markers, add_colliders).chain())
            .register_ldtk_entity_scene("player", worldly_player_scene)
            .register_ldtk_entity_scene("tree", tree_scene)
            .register_ldtk_entity_scene("item_spawn", item_spawn)
            .register_ldtk_entity_scene("flame_sprite", flame_sprite_scene)
            .register_ldtk_entity_scene("balefire", balefire_scene);
    }
}

fn add_tile_flag_markers(
    mut commands: Commands,
    enums: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>,
) {
    for (entity, enum_tags) in enums {
        let mut entity_commands = commands.entity(entity);

        for tag in &enum_tags.tags {
            match tag.as_str() {
                "solid" => {
                    entity_commands.insert(Solid);
                }
                other => warn!("Unsupported tile flag: {other}"),
            }
        }
    }
}

fn add_colliders(mut commands: Commands, solids: Query<Entity, Added<tile::Solid>>) {
    for entity in solids {
        commands
            .entity(entity)
            .insert((Collider::rectangle(TILE, TILE), RigidBody::Static));
    }
}

fn worldly_player_scene() -> impl Scene {
    bsn! {
        @player_scene()
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

pub fn map_scene() -> impl Scene {
    bsn! {
        LdtkProjectHandle { handle: "map.ldtk" }
    }
}
