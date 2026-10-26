use bevy_ecs::component::Component;

/// A tile that has been marked as "solid".
#[derive(Component, Default, Clone)]
pub struct Solid;
