use bevy_asset::AssetServer;
use bevy_ecs::{prelude::*, system::SystemParam};
use bevy_replique::prelude::*;

use crate::{DIALOGUE_DIR, Route};

pub fn in_conversation(runners: Query<(), With<DialogueRunner>>) -> bool {
    !runners.is_empty()
}

#[derive(SystemParam)]
pub struct Conversation<'w, 's> {
    runners: Query<'w, 's, (), With<DialogueRunner>>,
    asset_server: Res<'w, AssetServer>,
    commands: Commands<'w, 's>,
    start: MessageWriter<'w, StartDialogue>,
}

impl Conversation<'_, '_> {
    pub fn is_active(&self) -> bool {
        !self.runners.is_empty()
    }

    pub fn start(&mut self, route: &Route) {
        let runner = self
            .commands
            .spawn(DialogueRunner::new(
                self.asset_server
                    .load(format!("{DIALOGUE_DIR}/{}", route.file)),
            ))
            .id();

        self.start.write(StartDialogue {
            runner,
            node: route.node.clone(),
        });
    }
}
