use avian2d::prelude::PhysicsDebugPlugin;
use beacon_engine::EnginePlugin;
use beacon_game::GamePlugin;
use beacon_scene::{map::map_scene, ScenesPlugin};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((EnginePlugin, GamePlugin, ScenesPlugin))
        .add_plugins(PhysicsDebugPlugin)
        .add_systems(Startup, scene.spawn())
        .run();
}

fn scene() -> impl SceneList {
    bsn_list![camera(), map_scene()]
}

fn camera() -> impl Scene {
    bsn! {
        Camera2d
        Projection::from(OrthographicProjection {
            scale: 0.33,
            ..OrthographicProjection::default_2d()
        })
    }
}
