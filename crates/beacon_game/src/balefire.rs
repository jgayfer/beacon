use std::time::Duration;

use avian2d::prelude::CollidingEntities;
use beacon_core::{
    balefire::Balefire,
    inventory::{Inventory, SelectedSlot},
    item::ItemId,
    player::PlayerInteracted,
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::common_conditions::on_timer;

const MAX_INTENSITY: u8 = 100;

pub struct BalefirePlugin;

impl Plugin for BalefirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, decay.run_if(on_timer(Duration::from_secs(1))))
            .add_observer(on_player_interacted);
    }
}

fn decay(mut balefires: Query<&mut Balefire>) {
    for mut balefire in &mut balefires {
        balefire.intensity = balefire.intensity.saturating_sub(1);
    }
}

fn on_player_interacted(
    event: On<PlayerInteracted>,
    player: Single<(&mut Inventory, &SelectedSlot)>,
    balefire: Single<(&mut Balefire, &CollidingEntities)>,
) {
    let (mut inventory, selected) = player.into_inner();
    let (mut balefire, near) = balefire.into_inner();

    if !near.contains(&event.0) {
        return;
    }

    if inventory
        .get(selected.0)
        .is_none_or(|stack| stack.id != ItemId::Wood)
    {
        return;
    }

    let Some(stack) = inventory.take(selected.0) else {
        return;
    };

    balefire.intensity = (balefire.intensity as u16 + stack.count).min(MAX_INTENSITY as u16) as u8;
}
