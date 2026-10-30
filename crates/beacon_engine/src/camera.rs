use bevy::prelude::*;

pub struct CameraFollowPlugin;

impl Plugin for CameraFollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, follow.before(TransformSystems::Propagate));
    }
}

#[derive(Component, Default, Clone)]
pub struct CameraTarget;

fn follow(
    target: Query<&Transform, With<CameraTarget>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<CameraTarget>)>,
) {
    let (Ok(target), Ok(mut camera)) = (target.single(), camera.single_mut()) else {
        return;
    };
    camera.translation.x = target.translation.x;
    camera.translation.y = target.translation.y;
}
