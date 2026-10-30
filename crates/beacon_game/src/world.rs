use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use beacon_core::{TILE, state::GameState};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::Identifier("clearing".to_owned()))
            .add_systems(OnEnter(GameState::Playing), map_scene.spawn())
            .add_systems(Update, (add_tile_flag_markers, add_colliders).chain());
    }
}

/// A tile that has been marked as "solid".
#[derive(Component, Default, Clone)]
struct Solid;

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

fn add_colliders(mut commands: Commands, solids: Query<Entity, Added<Solid>>) {
    for entity in solids {
        commands
            .entity(entity)
            .insert((Collider::rectangle(TILE, TILE), RigidBody::Static));
    }
}

fn map_scene() -> impl Scene {
    bsn! {
        LdtkProjectHandle { handle: "world/map.ldtk" }
    }
}
