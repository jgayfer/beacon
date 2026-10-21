use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

#[derive(Resource, Default, Clone, PartialEq, Eq, Debug, Reflect)]
#[reflect(Resource)]
pub struct Progression {
    pub main_story: MainStory,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Reflect)]
pub enum MainStory {
    #[default]
    Start,
    MetSprite,
}
