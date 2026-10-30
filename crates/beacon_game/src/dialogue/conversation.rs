use beacon_core::dialogue::ActiveConversation;
use bevy_asset::AssetServer;
use bevy_ecs::{prelude::*, system::SystemParam};
use bevy_replique::prelude::*;

use super::{DIALOGUE_DIR, Route};

#[derive(SystemParam)]
pub struct Conversation<'w, 's> {
    conversations: Query<'w, 's, (), With<ActiveConversation>>,
    asset_server: Res<'w, AssetServer>,
    commands: Commands<'w, 's>,
    start: MessageWriter<'w, StartDialogue>,
}

impl Conversation<'_, '_> {
    pub fn is_active(&self) -> bool {
        !self.conversations.is_empty()
    }

    pub fn start(&mut self, route: &Route) {
        let runner = self
            .commands
            .spawn((
                DialogueRunner::new(
                    self.asset_server
                        .load(format!("{DIALOGUE_DIR}/{}", route.file)),
                ),
                ActiveConversation,
            ))
            .id();

        self.start.write(StartDialogue {
            runner,
            node: route.node.clone(),
        });
    }
}
