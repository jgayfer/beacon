use std::marker::PhantomData;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_replique::prelude::*;

use super::{DIALOGUE_DIR, NpcRoutes};

#[derive(Asset, TypePath)]
pub struct Routes<R: NpcRoutes> {
    pub routes: R,
    #[dependency]
    dialogues: Vec<Handle<RepliqueDialogue>>,
}

#[derive(Resource)]
pub struct RoutesHandle<R: NpcRoutes>(pub Handle<Routes<R>>);

impl<R: NpcRoutes> AssetCollection for RoutesHandle<R> {
    fn create(world: &mut World) -> Self {
        Self(
            world
                .resource::<AssetServer>()
                .load(format!("{DIALOGUE_DIR}/{}", R::PATH)),
        )
    }

    fn load(world: &mut World) -> Vec<UntypedHandle> {
        vec![Self::create(world).0.untyped()]
    }
}

#[derive(TypePath)]
pub struct RoutesLoader<R>(pub PhantomData<fn() -> R>);

impl<R: NpcRoutes> AssetLoader for RoutesLoader<R> {
    type Asset = Routes<R>;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Routes<R>, BevyError> {
        let mut src = String::new();
        reader.read_to_string(&mut src).await?;
        let routes: R = ron::from_str(&src)?;

        let dialogues = routes
            .routes()
            .map(|route| load_context.load(format!("{DIALOGUE_DIR}/{}", route.file)))
            .collect();

        Ok(Routes { routes, dialogues })
    }

    fn extensions(&self) -> &[&str] {
        &[]
    }
}
