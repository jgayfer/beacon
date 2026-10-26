use avian2d::prelude::PhysicsDebugPlugin;
use beacon_assets::AssetsPlugin;
use beacon_core::state::GameState;
use beacon_engine::EnginePlugin;
use beacon_game::GamePlugin;
use beacon_level::{LevelPlugin, map_scene};
use beacon_scene::{ScenesPlugin, dialogue_box::dialogue_box_scene, hotbar::hotbar_scene};
use bevy::prelude::*;
use inspector::InspectorPlugin;

mod inspector;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(InspectorPlugin)
        .add_plugins((
            AssetsPlugin,
            EnginePlugin,
            GamePlugin,
            ScenesPlugin,
            LevelPlugin,
        ))
        .add_plugins(PhysicsDebugPlugin)
        .add_systems(OnEnter(GameState::Playing), scene.spawn())
        .run();
}

fn scene() -> impl SceneList {
    bsn_list! {
        @camera()
        --
        @map_scene()
        --
        @hotbar_scene()
        --
        @dialogue_box_scene()
    }
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
