mod aseprite;
pub mod camera;
pub mod hitbox;
pub mod manifest;

use aseprite::AsepriteImageLoader;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteUltraPlugin;
use camera::CameraFollowPlugin;
use hitbox::HitboxPlugin;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            AsepriteUltraPlugin,
            CameraFollowPlugin,
            HitboxPlugin,
        ))
        .register_asset_loader(AsepriteImageLoader)
        .insert_resource(Gravity::ZERO);
    }
}
