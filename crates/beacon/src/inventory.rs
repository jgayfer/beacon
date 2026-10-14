use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::{CollisionEventsEnabled, CollisionStart},
};
use bevy::prelude::*;

use crate::Player;

const INVENTORY_SIZE: usize = 9;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_item_contact).add_observer(on_pickup);
    }
}

#[derive(EntityEvent)]
pub struct GroundItemTouched(Entity);

fn on_item_contact(
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

fn on_pickup(event: On<GroundItemTouched>, mut commands: Commands) {
    info!("Item touched");
    commands.entity(event.0).despawn();
}

#[derive(Component, Clone, Default)]
#[require(Transform, Collider::circle(8.0), Sensor, CollisionEventsEnabled)]
pub struct GroundItem(pub Item);

#[derive(Clone, Default)]
pub enum Item {
    #[default]
    Wood,
}

#[derive(Component, Clone)]
pub struct Inventory([Item; INVENTORY_SIZE]);
