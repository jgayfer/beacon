pub mod camera;
pub mod hitbox;

use avian2d::prelude::*;
use bevy::prelude::*;
use camera::CameraFollowPlugin;
use hitbox::HitboxPlugin;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugins::default(), CameraFollowPlugin, HitboxPlugin))
            .insert_resource(Gravity::ZERO);
    }
}
