mod asset;
mod commands;
mod conversation;
mod dialogue_box;
mod flame_sprite;
mod npc;
mod runner;
mod talk;

use beacon_core::npc::FlameSprite;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use commands::CommandsPlugin;
use dialogue_box::DialogueBoxPlugin;
use flame_sprite::FlameSpriteRoutes;
use npc::{AddNpcDialogue, NpcRoutes, Route};
use runner::RunnerPlugin;

const DIALOGUE_DIR: &str = "dialogue";

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RunnerPlugin, CommandsPlugin, DialogueBoxPlugin))
            .add_npc_dialogue::<FlameSprite, FlameSpriteRoutes>();
    }
}

#[derive(Message, Clone, Debug)]
struct LineShown {
    speaker: Option<String>,
    text: String,
}

#[derive(Message, Clone, Debug)]
struct DialogueEnded;
