use beacon_core::{interaction::Interaction, npc::FlameSprite};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;

pub struct FlameSpritePlugin;

impl Plugin for FlameSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_interaction);
    }
}

fn on_interaction(event: On<Interaction>, flame_sprites: Query<(), With<FlameSprite>>) {
    if !flame_sprites.contains(event.interactable) {
        return;
    }

    info!("Interacted with Flame Sprite");
}
