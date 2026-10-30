use std::marker::PhantomData;

use beacon_core::{progression::Progression, state::GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use serde::{Deserialize, de::DeserializeOwned};

use super::{
    asset::{Routes, RoutesHandle, RoutesLoader},
    talk::on_talked_to,
};

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Route {
    pub file: String,
    pub node: String,
}

pub trait NpcRoutes: DeserializeOwned + TypePath + Send + Sync + 'static {
    const PATH: &'static str;

    fn routes(&self) -> impl Iterator<Item = &Route>;

    fn pick(&self, progression: &Progression, last: Option<&Route>) -> Option<&Route>;
}

#[derive(Component, Default, Debug)]
pub struct LastRoute(pub Option<Route>);

pub trait AddNpcDialogue {
    fn add_npc_dialogue<N: Component, R: NpcRoutes>(&mut self) -> &mut Self;
}

impl AddNpcDialogue for App {
    fn add_npc_dialogue<N: Component, R: NpcRoutes>(&mut self) -> &mut Self {
        self.init_asset::<Routes<R>>()
            .register_asset_loader(RoutesLoader::<R>(PhantomData))
            .register_required_components::<N, LastRoute>()
            .configure_loading_state(
                LoadingStateConfig::new(GameState::Loading).load_collection::<RoutesHandle<R>>(),
            )
            .add_observer(on_talked_to::<N, R>)
    }
}
