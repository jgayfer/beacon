use avian2d::collision::collision_events::CollisionStart;
use beacon_core::{
    inventory::Inventory,
    item::{GroundItem, GroundItemTouched},
    player::Player,
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub struct GroundItemPlugin;

impl Plugin for GroundItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(detect_ground_item_contact)
            .add_observer(on_ground_item_touched);
    }
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
