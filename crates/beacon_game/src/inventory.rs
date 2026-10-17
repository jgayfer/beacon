use beacon_core::{inventory::SelectedSlot, player::Player};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_slot);
    }
}

fn select_slot(keys: Res<ButtonInput<KeyCode>>, mut selected: Single<&mut SelectedSlot, With<Player>>) {
    use KeyCode::*;
    const DIGITS: [KeyCode; 9] = [
        Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9,
    ];
    if let Some(i) = DIGITS.iter().position(|key| keys.just_pressed(*key)) {
        selected.set_if_neq(SelectedSlot(i));
    }
}
