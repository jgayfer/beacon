use beacon_core::dialogue::{DialogueEnded, LineShown};
use bevy::prelude::*;

pub struct DialogueBoxPlugin;

impl Plugin for DialogueBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (show_line, hide_on_finish));
    }
}

#[derive(Component, Default, Clone)]
struct DialogueBox;

#[derive(Component, Default, Clone)]
struct DialogueText;

pub fn dialogue_box_scene() -> impl Scene {
    bsn! {
        DialogueBox
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(80.0),
            left: Val::Percent(20.0),
            width: Val::Percent(60.0),
            padding: UiRect::all(Val::Px(12.0)),
            border: UiRect::all(Val::Px(2.0)),
        }
        Visibility::Hidden
        BorderColor::all(Color::WHITE)
        BackgroundColor({ Color::BLACK.with_alpha(0.8) })
        Children [
            DialogueText
            Text
        ]
    }
}

fn show_line(
    mut lines: MessageReader<LineShown>,
    mut visibility: Single<&mut Visibility, With<DialogueBox>>,
    mut text: Single<&mut Text, With<DialogueText>>,
) {
    for line in lines.read() {
        text.0 = match &line.speaker {
            Some(speaker) => format!("{speaker}: {}", line.text),
            None => line.text.clone(),
        };
        **visibility = Visibility::Inherited;
    }
}

fn hide_on_finish(
    mut finished: MessageReader<DialogueEnded>,
    mut visibility: Single<&mut Visibility, With<DialogueBox>>,
) {
    for _ in finished.read() {
        **visibility = Visibility::Hidden;
    }
}
