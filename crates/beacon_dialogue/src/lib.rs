mod asset;
mod commands;
mod conversation;
mod npc;
mod runner;
mod talk;

use bevy_app::prelude::*;

use commands::CommandsPlugin;
pub use conversation::in_conversation;
pub use npc::{AddNpcDialogue, NpcRoutes, Route};
use runner::RunnerPlugin;

const DIALOGUE_DIR: &str = "dialogue";

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RunnerPlugin, CommandsPlugin));
    }
}
