use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use serde::Deserialize;

#[derive(Resource, Default, Clone, PartialEq, Eq, Debug, Reflect)]
#[reflect(Resource)]
pub struct Progression {
    pub main_story: MainStory,
    pub sprite_level: SpriteLevel,
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

impl From<Prologue> for MainStory {
    fn from(beat: Prologue) -> Self {
        MainStory::Prologue(beat)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Reflect, Deserialize)]
pub enum Act1 {}

impl From<Act1> for MainStory {
    fn from(beat: Act1) -> Self {
        MainStory::Act1(beat)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Reflect, Deserialize)]
pub struct SpriteLevel(pub u8);

impl Default for SpriteLevel {
    fn default() -> Self {
        SpriteLevel(1)
    }
}
