use avian2d::prelude::*;
use beacon_core::TILE;
use beacon_engine::{camera::CameraFollowPlugin, hitbox::HitboxPlugin};
use beacon_game::{GroundItemPlugin, PlayerPlugin, WoodcuttingPlugin};
use beacon_scene::{
    player::{PlayerScenePlugin, player_scene},
    tree::tree_scene,
};
use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_light_2d::{
    light::{AmbientLight2d, Light2d, PointLight2d},
    plugin::Light2dPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin))
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(WoodcuttingPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GroundItemPlugin)
        .add_plugins(PlayerScenePlugin)
        .add_plugins(Light2dPlugin)
        .add_plugins(CameraFollowPlugin)
        .add_plugins(HitboxPlugin)
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            ..default()
        })
        .insert_resource(Gravity::ZERO)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_int_cell::<Wall>(1)
        .register_ldtk_entity_scene("player", player_scene)
        .register_ldtk_entity_scene("tree", tree_scene)
        .register_ldtk_entity_scene(
            "beacon",
            || bsn! { PointLight2d { radius: 150.0, color: WHITE } },
        )
        .add_systems(Startup, scene.spawn())
        .run();
}

#[derive(Component, LdtkIntCell)]
#[expect(clippy::duplicated_attributes)]
#[require(RigidBody::Static, Collider::rectangle(TILE, TILE))]
struct Wall {}

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

fn scene() -> impl SceneList {
    bsn_list![camera(), map()]
}

fn map() -> impl Scene {
    bsn! {
        template(|ctx| Ok(LdtkProjectHandle::from(ctx.resource::<AssetServer>().load("map.ldtk"))))
        LevelSet
        Transform
        Visibility
    }
}
