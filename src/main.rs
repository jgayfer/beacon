use avian2d::prelude::*;
use bevy::{asset::asset_value, prelude::*};
use bevy_ecs_ldtk::prelude::*;

const TILE: f32 = 16.0;
const SPEED: f32 = 100.0;
const PLAYER_RADIUS: f32 = 6.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            ..default()
        })
        .insert_resource(Gravity::ZERO)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_int_cell::<Wall>(1)
        .register_ldtk_entity::<PlayerSpawn>("player")
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, move_player)
        .add_systems(
            PostUpdate,
            (
                follow_player.before(TransformSystems::Propagate),
                spawn_player
                    .after(TransformSystems::Propagate)
                    .run_if(not(any_with_component::<Player>)),
            ),
        )
        .run();
}

#[derive(Component, LdtkIntCell)]
#[expect(clippy::duplicated_attributes)]
#[require(RigidBody::Static, Collider::rectangle(TILE, TILE))]
struct Wall {}

#[derive(Component, LdtkEntity)]
struct PlayerSpawn {}

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
            Mesh2d(asset_value(Circle::new(PLAYER_RADIUS)))
            MeshMaterial2d::<ColorMaterial>(asset_value(Color::srgb(1.0, 0.0, 0.0)))
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

fn spawn_player(
    mut commands: Commands,
    spawns: Query<&GlobalTransform, With<PlayerSpawn>>,
    player: Option<Single<(), With<Player>>>,
) {
    let (None, Some(spawn)) = (player, spawns.iter().next()) else {
        return;
    };
    commands.spawn_scene(bsn! { @Player Transform { translation: { spawn.translation() } } });
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
