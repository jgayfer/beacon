//! Dialogue as the rest of the game sees it, independent of the backend that
//! runs it.

use bevy_ecs::prelude::*;

/// A line of dialogue to show.
#[derive(Message, Clone, Debug)]
pub struct LineShown {
    pub speaker: Option<String>,
    pub text: String,
}

/// The current conversation is over.
#[derive(Message, Clone, Debug)]
pub struct DialogueEnded;

/// Something was talked to. The dialogue system decides whether a
/// conversation actually starts.
#[derive(EntityEvent, Clone, Debug)]
pub struct TalkedTo {
    #[event_target]
    pub npc: Entity,
}
