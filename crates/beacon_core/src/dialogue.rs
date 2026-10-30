use bevy_ecs::prelude::*;

#[derive(EntityEvent, Clone, Debug)]
pub struct TalkedTo {
    #[event_target]
    pub npc: Entity,
}
