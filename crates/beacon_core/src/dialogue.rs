use bevy_ecs::prelude::*;

#[derive(Message, Clone, Debug)]
pub struct LineShown {
    pub speaker: Option<String>,
    pub text: String,
}

#[derive(Message, Clone, Debug)]
pub struct DialogueEnded;

#[derive(Message, Clone, Debug)]
pub struct DialogueContinued;

#[derive(EntityEvent, Clone, Debug)]
pub struct TalkedTo {
    #[event_target]
    pub npc: Entity,
}
