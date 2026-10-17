use std::time::Duration;

use beacon_core::{
    balefire::Balefire,
    interaction::Interaction,
    inventory::{Inventory, SelectedSlot},
    item::ItemId,
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::common_conditions::on_timer;

const MAX_INTENSITY: u8 = 100;

pub struct BalefirePlugin;

impl Plugin for BalefirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, decay.run_if(on_timer(Duration::from_secs(1))))
            .add_observer(on_interaction);
    }
}

fn decay(mut balefires: Query<&mut Balefire>) {
    for mut balefire in &mut balefires {
        balefire.intensity = balefire.intensity.saturating_sub(1);
    }
}

fn on_interaction(
    event: On<Interaction>,
    mut balefires: Query<&mut Balefire>,
    mut players: Query<(&mut Inventory, &SelectedSlot)>,
) {
    if event.item.is_none_or(|stack| stack.id != ItemId::Wood) {
        return;
    }
    let Ok(mut balefire) = balefires.get_mut(event.target) else {
        return;
    };
    let Ok((mut inventory, selected)) = players.get_mut(event.player) else {
        return;
    };
    let Some(stack) = inventory.take(selected.0) else {
        return;
    };

    balefire.intensity = (balefire.intensity as u16 + stack.count).min(MAX_INTENSITY as u16) as u8;
}
