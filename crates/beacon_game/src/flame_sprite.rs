use beacon_core::{interaction::Interaction, npc::FlameSprite};
use bevy_app::prelude::*;
use bevy_asset::AssetServer;
use bevy_ecs::prelude::*;
use bevy_replique::prelude::*;

pub struct FlameSpritePlugin;

impl Plugin for FlameSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_interaction);
    }
}

fn on_interaction(
    event: On<Interaction>,
    flame_sprites: Query<(), With<FlameSprite>>,
    runners: Query<(), With<DialogueRunner>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut start: MessageWriter<StartDialogue>,
) {
    if !flame_sprites.contains(event.interactable) || !runners.is_empty() {
        return;
    }

    let runner = commands
        .spawn(DialogueRunner::new(
            asset_server.load("dialogue/flame_sprite.rep"),
        ))
        .id();

    start.write(StartDialogue {
        runner,
        node: "flame_sprite".into(),
    });
}
