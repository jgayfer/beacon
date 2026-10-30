use beacon_core::progression::{MainStory, Progression};
use bevy::prelude::*;
use bevy_replique::prelude::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_dialogue_command_named("update_main_story", update_main_story)
            .add_dialogue_function_named("story_is", story_is);
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
