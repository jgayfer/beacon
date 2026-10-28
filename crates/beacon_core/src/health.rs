use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Health(pub usize);
