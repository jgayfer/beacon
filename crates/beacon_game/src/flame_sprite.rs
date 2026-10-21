use std::collections::BTreeMap;

use beacon_core::{
    dialogue::TalkedTo,
    interaction::Interaction,
    npc::FlameSprite,
    progression::{MainStory, Progression, SpriteLevel},
};
use beacon_dialogue::{AddNpcDialogue, NpcRoutes, Route};
use bevy_app::prelude::*;
use bevy_asset::Asset;
use bevy_ecs::prelude::*;
use bevy_reflect::TypePath;
use serde::Deserialize;

pub struct FlameSpritePlugin;

impl Plugin for FlameSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_npc_dialogue::<FlameSprite, FlameSpriteRoutes>("flame_sprite/routes.ron")
            .add_observer(on_interaction);
    }
}

#[derive(Asset, TypePath, Deserialize, Debug)]
struct FlameSpriteRoutes {
    story: BTreeMap<MainStory, Route>,
    chatter: BTreeMap<SpriteLevel, Route>,
}

impl NpcRoutes for FlameSpriteRoutes {
    fn pick(&self, progression: &Progression) -> Option<&Route> {
        self.story
            .get(&progression.main_story)
            .or_else(|| self.chatter.get(&progression.sprite_level))
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
