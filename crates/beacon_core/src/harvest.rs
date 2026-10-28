use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Harvestable(pub usize);

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Harvested;
