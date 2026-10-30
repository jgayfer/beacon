use beacon_core::progression::{MainStory, Progression};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_replique::prelude::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_dialogue_command_named("update_main_story", update_main_story);
    }
}

fn update_main_story(In((Step(step),)): In<(Step,)>, mut progression: ResMut<Progression>) {
    progression.main_story = step;
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
