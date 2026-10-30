use avian2d::prelude::PhysicsDebugPlugin;
use beacon_engine::EnginePlugin;
use beacon_game::GamePlugin;
use bevy::prelude::*;
use inspector::InspectorPlugin;

mod inspector;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(InspectorPlugin)
        .add_plugins((EnginePlugin, GamePlugin))
        .add_plugins(PhysicsDebugPlugin)
        .run();
}
