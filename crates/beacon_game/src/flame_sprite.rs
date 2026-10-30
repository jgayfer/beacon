use beacon_core::{dialogue::TalkedTo, interaction::Interaction, npc::FlameSprite};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct FlameSpritePlugin;

impl Plugin for FlameSpritePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_scene("flame_sprite", flame_sprite_scene)
            .add_observer(on_interaction);
    }
}

fn on_interaction(
    event: On<Interaction>,
    flame_sprites: Query<(), With<FlameSprite>>,
    mut commands: Commands,
) {
    if flame_sprites.contains(event.interactable) {
        commands.trigger(TalkedTo {
            npc: event.interactable,
        });
    }
}

fn flame_sprite_scene() -> impl Scene {
    bsn! {
        FlameSprite
        Sprite {
            image: "characters/flame_sprite.aseprite",
        }
    }
}
