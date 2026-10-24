use avian2d::collision::collision_events::CollisionStart;
use beacon_assets::ItemRegistry;
use beacon_core::{
    inventory::Inventory,
    item::{GroundItem, GroundItemTouched, ItemSpawn},
    player::Player,
};
use beacon_scene::ground_item::ground_item_scene;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_scene::prelude::*;

pub struct GroundItemPlugin;

impl Plugin for GroundItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_ground_item)
            .add_observer(detect_ground_item_contact)
            .add_observer(on_ground_item_touched);
    }
}

fn spawn_ground_item(
    event: On<Add, ItemSpawn>,
    mut commands: Commands,
    spawns: Query<&ItemSpawn>,
    items: Res<ItemRegistry>,
) {
    let Ok(spawn) = spawns.get(event.entity) else {
        return;
    };
    let Some(stack) = items.stack(&spawn.id, spawn.count) else {
        warn!("items.ron doesn't define {:?}", spawn.id);
        return;
    };
    commands
        .entity(event.entity)
        .queue_spawn_related_scenes::<Children>(bsn_list![ground_item_scene(stack)]);
}

fn detect_ground_item_contact(
    event: On<CollisionStart>,
    ground_item_query: Query<(), With<GroundItem>>,
    player_query: Query<(), With<Player>>,
    mut commands: Commands,
) {
    let (ground_item, other) = (event.collider1, event.collider2);
    if ground_item_query.contains(ground_item) && player_query.contains(other) {
        commands.trigger(GroundItemTouched(ground_item));
    }
}

fn on_ground_item_touched(
    event: On<GroundItemTouched>,
    mut commands: Commands,
    mut ground_items: Query<&mut GroundItem>,
    mut inventory: Single<&mut Inventory, With<Player>>,
) {
    let Ok(mut ground_item) = ground_items.get_mut(event.0) else {
        return;
    };

    match inventory.insert(ground_item.to_owned().0) {
        None => {
            commands.entity(event.0).despawn();
        }
        Some(leftover) => {
            ground_item.0 = leftover;
        }
    }
}
