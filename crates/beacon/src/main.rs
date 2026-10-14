use avian2d::prelude::*;
use beacon::{TILE, Tree, woodcutting::WoodcuttingPlugin};
use beacon_core::player::{Action, Facing, Player};
use beacon_engine::{camera::CameraFollowPlugin, hitbox::HitboxPlugin};
use beacon_game::GroundItemPlugin;
use beacon_scene::player::{PlayerScenePlugin, player_scene};
use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_light_2d::{
    light::{AmbientLight2d, Light2d, PointLight2d},
    plugin::Light2dPlugin,
};

const SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin))
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(WoodcuttingPlugin)
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
        .register_ldtk_entity_scene("tree", || bsn! { @Tree { health: 3 }})
        .register_ldtk_entity_scene(
            "beacon",
            || bsn! { PointLight2d { radius: 150.0, color: WHITE } },
        )
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, move_player)
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

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut LinearVelocity, &mut Facing, &mut Action), With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    for (mut velocity, mut facing, mut action) in &mut q {
        velocity.0 = dir.normalize_or_zero() * SPEED;

        action.set_if_neq(if dir == Vec2::ZERO {
            Action::Idle
        } else {
            Action::Walking
        });

        if dir.x < 0.0 {
            facing.set_if_neq(Facing::Left);
        } else if dir.x > 0.0 {
            facing.set_if_neq(Facing::Right);
        }
    }
}
