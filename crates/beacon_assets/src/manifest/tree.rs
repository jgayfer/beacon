use beacon_core::item::ItemId;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use serde::Deserialize;

use super::{ManifestEntry, Registry};

#[derive(Deserialize, Clone, Debug, Default, PartialEq, Reflect)]
pub struct TreeDef {
    pub id: String,
    pub health: usize,
    pub collider_radius: f32,
    #[serde(default)]
    pub drop: Option<ItemId>,
}

impl ManifestEntry for TreeDef {
    const DIR: &'static str = "trees";
    type Sprite = Aseprite;

    fn id(&self) -> &str {
        &self.id
    }
}

pub type TreeRegistry = Registry<TreeDef>;
