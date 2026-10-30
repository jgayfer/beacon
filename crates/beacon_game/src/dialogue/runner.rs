use beacon_core::dialogue::DialogueContinued;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_replique::prelude::*;

use super::{DialogueEnded, LineShown};

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliquePlugin)
            .add_message::<LineShown>()
            .add_message::<DialogueEnded>()
            .add_message::<DialogueContinued>()
            .init_resource::<CurrentLine>()
            .add_systems(Update, next_line)
            .add_systems(
                PostUpdate,
                (
                    next_line.before(DialogueSystem::Runner),
                    (show_line, end_finished).after(DialogueSystem::Runner),
                ),
            );
    }
}

#[derive(Resource, Default)]
struct CurrentLine(Option<DialogueToken>);

fn show_line(
    mut lines: MessageReader<DialogueLine>,
    mut current: ResMut<CurrentLine>,
    mut shown: MessageWriter<LineShown>,
) {
    for line in lines.read() {
        current.0 = Some(line.token);
        shown.write(LineShown {
            speaker: line.speaker.clone(),
            text: line.text.clone(),
        });
    }
}

fn next_line(
    mut continued: MessageReader<DialogueContinued>,
    mut current: ResMut<CurrentLine>,
    mut resume: MessageWriter<ResumeDialogue>,
) {
    if continued.read().count() > 0
        && let Some(token) = current.0.take()
    {
        resume.write(ResumeDialogue {
            token,
            input: ResumeInput::Advance,
        });
    }
}

fn end_finished(
    mut finished: MessageReader<DialogueFinished>,
    mut ended: MessageWriter<DialogueEnded>,
    mut commands: Commands,
) {
    for message in finished.read() {
        commands.entity(message.runner).despawn();
        ended.write(DialogueEnded);
    }
}
