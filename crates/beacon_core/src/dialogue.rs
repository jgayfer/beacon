use bevy_ecs::prelude::*;

#[derive(Message, Clone, Debug)]
pub struct DialogueContinued;

#[derive(EntityEvent, Clone, Debug)]
pub struct TalkedTo {
    #[event_target]
    pub npc: Entity,
}

#[derive(Component, Default, Clone, Debug)]
pub struct ActiveConversation;

pub fn in_conversation(conversations: Query<(), With<ActiveConversation>>) -> bool {
    !conversations.is_empty()
}
