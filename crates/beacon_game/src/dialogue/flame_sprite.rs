use std::collections::BTreeMap;

use beacon_core::progression::{MainStory, Progression, SpriteLevel};
use bevy::prelude::*;
use serde::Deserialize;

use super::{NpcRoutes, Route};

#[derive(TypePath, Deserialize, Debug)]
pub struct FlameSpriteRoutes {
    story: BTreeMap<MainStory, Vec<Route>>,
    chatter: BTreeMap<SpriteLevel, Vec<Route>>,
}

impl NpcRoutes for FlameSpriteRoutes {
    const PATH: &'static str = "flame_sprite/routes.ron";

    fn routes(&self) -> impl Iterator<Item = &Route> {
        self.story.values().chain(self.chatter.values()).flatten()
    }

    fn pick(&self, progression: &Progression, last: Option<&Route>) -> Option<&Route> {
        let routes = self
            .story
            .get(&progression.main_story)
            .or_else(|| self.chatter.get(&progression.sprite_level))?;

        let next = last
            .and_then(|last| routes.iter().position(|route| route == last))
            .map_or(0, |i| (i + 1) % routes.len());

        routes.get(next)
    }
}
