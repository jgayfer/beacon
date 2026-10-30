use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_replique::prelude::*;

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliquePlugin)
            .init_resource::<CurrentLine>()
            .add_systems(Update, next_line)
            .add_systems(
                PostUpdate,
                (
                    next_line.before(DialogueSystem::Runner),
                    (track_line, despawn_finished).after(DialogueSystem::Runner),
                ),
            );
    }
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
