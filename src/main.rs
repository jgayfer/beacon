use avian2d::prelude::*;
use bevy::{asset::asset_value, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::woodcutting::WoodcuttingPlugin;

mod woodcutting;

const TILE: f32 = 16.0;
const SPEED: f32 = 100.0;
const PLAYER_RADIUS: f32 = 6.0;

/// Animation tags defined in `Soldier.aseprite`.
const SOLDIER_IDLE: &str = "Idle";
const SOLDIER_WALK: &str = "Walk";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin))
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(WoodcuttingPlugin)
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            ..default()
        })
        .insert_resource(Gravity::ZERO)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_int_cell::<Wall>(1)
        .register_ldtk_entity_scene("player", || bsn! { @Player })
        .register_ldtk_entity_scene("tree", || bsn! { @Tree { health: 3 }})
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, (move_player, animate_player).chain())
        .add_systems(
            PostUpdate,
            (follow_player.before(TransformSystems::Propagate),),
        )
        .run();
}

#[derive(Component, LdtkIntCell)]
#[expect(clippy::duplicated_attributes)]
#[require(RigidBody::Static, Collider::rectangle(TILE, TILE))]
struct Wall {}

#[derive(SceneComponent, Clone, Default)]
#[require(
    RigidBody::Dynamic,
    Collider::circle(PLAYER_RADIUS),
    LockedAxes::ROTATION_LOCKED,
    LinearVelocity
)]
struct Player;

impl Player {
    fn scene() -> impl Scene {
        bsn! {
            // The render target. `AseAnimation` fills in the image and atlas.
            Sprite
            AseAnimation {
                aseprite: "Soldier.aseprite",
                animation: Animation::tag(SOLDIER_IDLE),
            }
        }
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

#[derive(SceneComponent, FromTemplate)]
#[require(RigidBody::Static, Collider::circle(TILE / 2.0) )]
struct Tree {
    pub health: usize,
}

impl Tree {
    fn scene() -> impl Scene {
        bsn! {
            Sprite {
                image: "arbutus.png",
            }
        }
    }
}

fn move_player(keys: Res<ButtonInput<KeyCode>>, mut q: Query<&mut LinearVelocity, With<Player>>) {
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
    for mut v in &mut q {
        v.0 = dir.normalize_or_zero() * SPEED;
    }
}

/// Plays the walk animation while the player is moving, and faces the direction of travel.
fn animate_player(mut q: Query<(&LinearVelocity, &mut AseAnimation, &mut Sprite), With<Player>>) {
    for (velocity, mut animation, mut sprite) in &mut q {
        let moving = velocity.0 != Vec2::ZERO;

        let target = if moving { SOLDIER_WALK } else { SOLDIER_IDLE };
        if animation.animation.tag.as_deref() != Some(target) {
            animation.animation.play_loop(target);
        }

        if velocity.0.x != 0.0 {
            sprite.flip_x = velocity.0.x < 0.0;
        }
    }
}

fn follow_player(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let (Ok(p), Ok(mut c)) = (player.single(), camera.single_mut()) else {
        return;
    };
    c.translation.x = p.translation.x;
    c.translation.y = p.translation.y;
}
