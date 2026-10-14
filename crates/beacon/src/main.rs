use avian2d::prelude::*;
use beacon_engine::{camera::CameraFollowPlugin, hitbox::HitboxPlugin};
use beacon_game::{GroundItemPlugin, PlayerPlugin, WoodcuttingPlugin};
use beacon_scene::{
    map::{MapPlugin, map_scene},
    player::PlayerScenePlugin,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::{
    light::{AmbientLight2d, Light2d},
    plugin::Light2dPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin))
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(Light2dPlugin)
        .add_plugins((CameraFollowPlugin, HitboxPlugin))
        .add_plugins((PlayerPlugin, WoodcuttingPlugin, GroundItemPlugin))
        .add_plugins((MapPlugin, PlayerScenePlugin))
        .insert_resource(Gravity::ZERO)
        .add_systems(Startup, scene.spawn())
        .run();
}

fn scene() -> impl SceneList {
    bsn_list![camera(), map_scene()]
}

fn camera() -> impl Scene {
    bsn! {
        Camera2d
        template(|_| Ok(Light2d {
            ambient_light: AmbientLight2d { brightness: 0.2, ..default() },
        }))
        Projection::from(OrthographicProjection {
            scale: 0.33,
            ..OrthographicProjection::default_2d()
        })
    }
}
