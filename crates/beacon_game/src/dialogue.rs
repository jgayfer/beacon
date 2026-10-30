use beacon_core::{
    dialogue::Dialogue,
    interaction::Interaction,
    progression::{MainStory, Progression},
    state::GameState,
};
use bevy::prelude::*;
use bevy_replique::prelude::*;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliquePlugin)
            .init_resource::<CurrentLine>()
            .add_dialogue_command_named("update_main_story", update_main_story)
            .add_dialogue_function_named("story_is", story_is)
            .add_dialogue_function_named("story_after", story_after)
            .add_observer(start_dialogue)
            .add_systems(OnEnter(GameState::Playing), dialogue_box_scene.spawn())
            .add_systems(Update, (show_line, hide_on_finish))
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

    let runner = commands
        .spawn(DialogueRunner::new(dialogue.0.clone()))
        .id();
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

struct Step(MainStory);

impl FromValue for Step {
    fn from_value(value: Value) -> Result<Self, ValueError> {
        let text = String::from_value(value)?;
        ron::from_str(&text)
            .map(Step)
            .map_err(|_| ValueError::Unknown {
                expected: "MainStory",
                got: text,
            })
    }
}

fn update_main_story(In((Step(step),)): In<(Step,)>, mut progression: ResMut<Progression>) {
    progression.main_story = step;
}

fn story_is(In((Step(step),)): In<(Step,)>, progression: Res<Progression>) -> bool {
    progression.main_story == step
}

fn story_after(In((Step(step),)): In<(Step,)>, progression: Res<Progression>) -> bool {
    progression.main_story > step
}

#[derive(Component, Default, Clone)]
struct DialogueBox;

#[derive(Component, Default, Clone)]
struct DialogueText;

fn dialogue_box_scene() -> impl Scene {
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
    mut lines: MessageReader<DialogueLine>,
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
    mut finished: MessageReader<DialogueFinished>,
    mut visibility: Single<&mut Visibility, With<DialogueBox>>,
) {
    for _ in finished.read() {
        **visibility = Visibility::Hidden;
    }
}
