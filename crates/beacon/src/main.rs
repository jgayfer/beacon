use avian2d::prelude::PhysicsDebugPlugin;
use beacon_engine::EnginePlugin;
use beacon_game::GamePlugin;
use beacon_scene::{ScenesPlugin, map::map_scene};
use bevy::prelude::*;
use bevy_light_2d::light::{AmbientLight2d, Light2d};

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
        template(|_| Ok(Light2d {
            ambient_light: AmbientLight2d { brightness: 0.2, ..default() },
        }))
        Projection::from(OrthographicProjection {
            scale: 0.33,
            ..OrthographicProjection::default_2d()
        })
    }
}
