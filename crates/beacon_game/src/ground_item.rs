use avian2d::collision::collision_events::CollisionStart;
use beacon_core::{
    inventory::Inventory,
    item::{GroundItem, ItemId, ItemRegistry},
    player::Player,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const GROUND_ITEM_SIZE: f32 = 12.0;

pub struct GroundItemPlugin;

impl Plugin for GroundItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_scene("item_spawn", item_spawn)
            .add_observer(spawn_ground_item)
            .add_observer(show_ground_item)
            .add_observer(detect_ground_item_contact)
            .add_observer(on_ground_item_touched);
    }
}

#[derive(EntityEvent)]
struct GroundItemTouched(Entity);

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(Visibility)]
struct ItemSpawn {
    id: ItemId,
    count: u16,
}

fn spawn_ground_item(
    event: On<Add<ItemSpawn>>,
    mut commands: Commands,
    spawns: Query<&ItemSpawn>,
    items: Res<ItemRegistry>,
) {
    let Ok(spawn) = spawns.get(event.entity) else {
        return;
    };
    let Some(stack) = items.get(&spawn.id).map(|def| def.stack(spawn.count)) else {
        warn!("Unknown item: {:?}", spawn.id);
        return;
    };
    commands.entity(event.entity).with_child(GroundItem(stack));
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

fn show_ground_item(
    event: On<Add<GroundItem>>,
    mut commands: Commands,
    ground_items: Query<&GroundItem>,
) {
    let Ok(ground_item) = ground_items.get(event.entity) else {
        return;
    };
    commands.entity(event.entity).insert(Sprite {
        image: ground_item.0.def.sprite.clone(),
        custom_size: Some(Vec2::splat(GROUND_ITEM_SIZE)),
        ..Default::default()
    });
}

fn item_spawn(ctx: &EntityInstanceContext) -> impl Scene + use<> {
    let id = ItemId::new(
        ctx.entity_instance
            .get_string_field("item_id")
            .expect("item_spawn entity's string 'item_id' field is present."),
    );

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
