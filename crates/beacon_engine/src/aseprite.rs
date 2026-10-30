use asefile::AsepriteFile;
use bevy::{
    asset::{AssetLoader, LoadContext, RenderAssetUsages, io::Reader},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

#[derive(Default, TypePath)]
pub struct AsepriteImageLoader;

impl AssetLoader for AsepriteImageLoader {
    type Asset = Image;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &(),
        _: &mut LoadContext<'_>,
    ) -> Result<Image, BevyError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let file = AsepriteFile::read(&bytes[..])?;

        Ok(Image::new(
            Extent3d {
                width: file.width() as u32,
                height: file.height() as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            file.frame(0).image().into_raw(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        ))
    }

    fn extensions(&self) -> &[&str] {
        &["aseprite", "ase"]
    }
}
