use bevy_asset::Handle;
use bevy_ecs::{prelude::*, template::FromTemplate};
use bevy_replique::prelude::*;

#[derive(Component, Clone, FromTemplate)]
pub struct Dialogue(pub Handle<RepliqueDialogue>);
