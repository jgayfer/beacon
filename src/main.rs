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
        .register_ldtk_entity::<TreeSpawn>("tree")
        .add_systems(Startup, scene.spawn())
        .add_systems(
            Update,
            (move_player, spawn_player, spawn_trees, update_tree_count),
        )
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

#[derive(Component, LdtkEntity)]
struct PlayerSpawn {}

#[derive(SceneComponent, Clone, Default)]
#[require(
    RigidBody::Dynamic,
    Collider::circle(PLAYER_RADIUS),
    LockedAxes::ROTATION_LOCKED,
    LinearVelocity
)]
struct Player {
    pub trees: usize,
}

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
    bsn_list![camera(), map(), @TreeCount]
}

fn on_player_collision(
    event: On<CollisionStart>,
    mut players: Query<&mut Player>,
    mut commands: Commands,
) {
    if let Ok(mut player) = players.get_mut(event.collider2) {
        player.trees += 1;
        commands.entity(event.collider1).despawn();
    }
}

fn map() -> impl Scene {
    bsn! {
        template(|ctx| Ok(LdtkProjectHandle::from(ctx.resource::<AssetServer>().load("map.ldtk"))))
        LevelSet
        Transform
        Visibility
    }
}

#[derive(SceneComponent, Default, Clone)]
struct TreeCount;

impl TreeCount {
    fn scene() -> impl Scene {
        bsn! {
            Text("Trees: 0")
            Node {
                position_type: PositionType::Absolute,
                top: px(8),
                left: px(8)
            }
        }
    }
}

fn update_tree_count(
    player: Query<&Player, Changed<Player>>,
    mut text: Query<&mut Text, With<TreeCount>>,
) {
    let (Ok(player), Ok(mut text)) = (player.single(), text.single_mut()) else {
        return;
    };

    text.0 = format!("Trees: {}", player.trees);
}

#[derive(Component, LdtkEntity)]
struct TreeSpawn {}

#[derive(SceneComponent, FromTemplate)]
#[require(RigidBody::Static, Collider::circle(TILE / 2.0), CollisionEventsEnabled)]
struct Tree;

impl Tree {
    fn scene() -> impl Scene {
        bsn! {
            Mesh2d(asset_value(Circle::new(4.0)))
            MeshMaterial2d::<ColorMaterial>(asset_value(Color::srgb(0.0, 1.0, 0.0)))
            on(on_player_collision)
        }
    }
}

fn spawn_trees(mut commands: Commands, tree_spawns: Query<&GlobalTransform, Added<TreeSpawn>>) {
    for transform in tree_spawns {
        commands.spawn_scene(bsn! { @Tree Transform { translation: { transform.translation() } } });
    }
}

fn spawn_player(
    mut commands: Commands,
    new_player_spawns: Query<&GlobalTransform, Added<PlayerSpawn>>,
) {
    for player_spawn in new_player_spawns {
        commands.spawn_scene(
            bsn! { @Player Transform { translation: { player_spawn.translation() } } },
        );
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
