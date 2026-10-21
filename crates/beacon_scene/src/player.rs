use beacon_core::player::{Action, Facing, Player};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

pub const SOLDIER_IDLE: &str = "Idle";
pub const SOLDIER_WALK: &str = "Walk";

pub fn player_scene() -> impl Scene {
    bsn! {
        #Player
        Player
        Sprite
        AseAnimation {
            aseprite: "soldier.aseprite",
            animation: Animation::tag(SOLDIER_IDLE),
        }
    }
}

pub struct PlayerScenePlugin;

impl Plugin for PlayerScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_player);
    }
}

fn animate_player(
    mut q: Query<
        (&Facing, &Action, &mut AseAnimation, &mut Sprite),
        (With<Player>, Or<(Changed<Facing>, Changed<Action>)>),
    >,
) {
    for (facing, action, mut animation, mut sprite) in &mut q {
        let animation_tag = match action {
            Action::Idle => SOLDIER_IDLE,
            Action::Walking => SOLDIER_WALK,
        };

        if animation.animation.tag.as_deref() != Some(animation_tag) {
            animation.animation.play_loop(animation_tag);
        }

        sprite.flip_x = matches!(facing, Facing::Left);
    }
}
