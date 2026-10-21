use beacon_core::progression::{MainStory, Progression};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_replique::prelude::*;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliquePlugin)
            .add_dialogue_function_named("main_story", main_story)
            .add_dialogue_command_named("met_flame_sprite", met_flame_sprite)
            .init_resource::<CurrentLine>()
            .add_systems(Update, (show_line, next_line, despawn_finished));
    }
}

#[derive(Resource, Default)]
struct CurrentLine(Option<DialogueToken>);

fn show_line(mut lines: MessageReader<DialogueLine>, mut current: ResMut<CurrentLine>) {
    for line in lines.read() {
        current.0 = Some(line.token);
    }
}

fn next_line(
    keys: Res<ButtonInput<KeyCode>>,
    mut current: ResMut<CurrentLine>,
    mut resume: MessageWriter<ResumeDialogue>,
) {
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Enter])
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

fn main_story(In(()): In<()>, progression: Res<Progression>) -> String {
    format!("{:?}", progression.main_story)
}

fn met_flame_sprite(In(()): In<()>, mut progression: ResMut<Progression>) {
    progression.main_story = MainStory::MetSprite;
}
