use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use serde::Deserialize;

#[derive(Resource, Default, Clone, PartialEq, Eq, Debug, Reflect)]
#[reflect(Resource)]
pub struct Progression {
    pub main_story: MainStory,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Reflect, Deserialize)]
pub enum MainStory {
    Prologue(Prologue),
    Act1(Act1),
}

impl Default for MainStory {
    fn default() -> Self {
        MainStory::Prologue(Prologue::Start)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Reflect, Deserialize)]
pub enum Prologue {
    Start,
    MetSprite,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Reflect, Deserialize)]
pub enum Act1 {
    Start,
}
