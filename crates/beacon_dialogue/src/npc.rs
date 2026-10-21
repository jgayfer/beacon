use std::marker::PhantomData;

use beacon_core::progression::Progression;
use bevy_app::prelude::*;
use bevy_asset::{Asset, AssetApp, AssetServer};
use bevy_ecs::prelude::*;
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    DIALOGUE_DIR,
    asset::{RoutesHandle, RoutesLoader},
    talk::on_talked_to,
};

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Route {
    pub file: String,
    pub node: String,
}

pub trait NpcRoutes: Asset + DeserializeOwned {
    fn pick(&self, progression: &Progression, last: Option<&Route>) -> Option<&Route>;
}

#[derive(Component, Default, Debug)]
pub struct LastRoute(pub Option<Route>);

pub trait AddNpcDialogue {
    fn add_npc_dialogue<N: Component, R: NpcRoutes>(&mut self, path: &'static str) -> &mut Self;
}

impl AddNpcDialogue for App {
    fn add_npc_dialogue<N: Component, R: NpcRoutes>(&mut self, path: &'static str) -> &mut Self {
        self.init_asset::<R>()
            .register_asset_loader(RoutesLoader::<R>(PhantomData))
            .register_required_components::<N, LastRoute>()
            .add_systems(
                Startup,
                move |asset_server: Res<AssetServer>, mut commands: Commands| {
                    commands.insert_resource(RoutesHandle::<R>(
                        asset_server.load(format!("{DIALOGUE_DIR}/{path}")),
                    ));
                },
            )
            .add_observer(on_talked_to::<N, R>)
    }
}
