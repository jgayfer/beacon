mod commands;
mod dialogue_box;

use beacon_core::{dialogue::Dialogue, interaction::Interaction};
use bevy::prelude::*;
use bevy_replique::prelude::*;
use commands::CommandsPlugin;
use dialogue_box::DialogueBoxPlugin;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RepliquePlugin, CommandsPlugin, DialogueBoxPlugin))
            .init_resource::<CurrentLine>()
            .add_observer(start_dialogue)
            .add_systems(
                PostUpdate,
                (
                    next_line.before(DialogueSystem::Runner),
                    (track_line, despawn_finished).after(DialogueSystem::Runner),
                ),
            );
    }
}

fn start_dialogue(
    event: On<Interaction>,
    dialogues: Query<&Dialogue>,
    runners: Query<(), With<DialogueRunner>>,
    mut commands: Commands,
    mut start: MessageWriter<StartDialogue>,
) {
    let Ok(dialogue) = dialogues.get(event.interactable) else {
        return;
    };
    if !runners.is_empty() {
        return;
    }

    let runner = commands.spawn(DialogueRunner::new(dialogue.0.clone())).id();
    start.write(StartDialogue {
        runner,
        node: "talk".into(),
    });
}

#[derive(Resource, Default)]
struct CurrentLine(Option<DialogueToken>);

fn track_line(mut lines: MessageReader<DialogueLine>, mut current: ResMut<CurrentLine>) {
    for line in lines.read() {
        current.0 = Some(line.token);
    }
}

fn next_line(
    keys: Res<ButtonInput<KeyCode>>,
    mut current: ResMut<CurrentLine>,
    mut resume: MessageWriter<ResumeDialogue>,
) {
    if keys.just_pressed(KeyCode::Space)
        && let Some(token) = current.0.take()
    {
        resume.write(ResumeDialogue {
            token,
            input: ResumeInput::Advance,
        });
    }
}

fn despawn_finished(mut finished: MessageReader<DialogueFinished>, mut commands: Commands) {
    for message in finished.read() {
        commands.entity(message.runner).despawn();
    }
}
