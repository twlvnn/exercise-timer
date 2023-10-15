use crate::exercise_setup::ExerciseSetup;
use json::{self, object};
use relm4::binding::*;
use relm4::gtk::gio::{self, prelude::*};

const APP_ID: &str = "xyz.safeworlds.hiit";

#[derive(Clone, Debug, Default)]
pub struct WindowGeometry {
    pub width: I32Binding,
    pub height: I32Binding,
    pub is_maximized: BoolBinding,
}

impl WindowGeometry {
    pub fn new_from_gsettings() -> Self {
        let settings = gio::Settings::new(APP_ID);
        Self {
            width: I32Binding::new(settings.int("window-width")),
            height: I32Binding::new(settings.int("window-height")),
            is_maximized: BoolBinding::new(settings.boolean("window-is-maximized")),
        }
    }
}

impl Drop for WindowGeometry {
    fn drop(&mut self) {
        let settings = gio::Settings::new(APP_ID);
        settings.delay();
        let _ = settings.set_int("window-width", self.width.get());
        let _ = settings.set_int("window-height", self.height.get());
        let _ = settings.set_boolean("window-is-maximized", self.is_maximized.get());
        settings.apply();
    }
}

fn parse_json_to_exercise_setup(value: &json::JsonValue) -> ExerciseSetup {
    ExerciseSetup {
        name: value["name"]
            .as_str()
            .expect("Cannot find 'name' in settings dictionary")
            .to_string(),
        sets: value["sets"]
            .as_usize()
            .expect("Cannot find 'sets' in settings dictionary"),
        warmup_s: value["warmup_s"]
            .as_usize()
            .expect("Cannot find 'warmup_s' in settings dictionary"),
        exercise_s: value["exercise_s"]
            .as_usize()
            .expect("Cannot find 'exercises_s' in settings dictionary"),
        rest_s: value["rest_s"]
            .as_usize()
            .expect("Cannot find 'rest_s' in settings dictionary"),
    }
}

pub fn load_default_exercise_setup() -> ExerciseSetup {
    let settings = gio::Settings::new(APP_ID);
    let raw_json = settings.string("default-exercise-json");
    parse_json_to_exercise_setup(
        &json::parse(&raw_json).expect("Could not parse default exercise setup"),
    )
}

pub fn load_exercise_list_from_gsettings() -> Vec<ExerciseSetup> {
    let settings = gio::Settings::new(APP_ID);
    let raw_json = settings.string("exercise-json-list");
    let parsed = json::parse(&raw_json).expect("Could not parse exercise list");
    parsed.members().map(parse_json_to_exercise_setup).collect()
}

pub fn save_exercise_list_to_gsettings<'a>(exercises: impl Iterator<Item = &'a ExerciseSetup>) {
    let settings = gio::Settings::new(APP_ID);
    let json_list: Vec<json::JsonValue> = exercises
        .map(|exercise| {
            object! {
                name: exercise.name.clone(),
                sets: exercise.sets,
                warmup_s: exercise.warmup_s,
                exercise_s: exercise.exercise_s,
                rest_s: exercise.rest_s,
            }
        })
        .collect();
    settings
        .set("exercise-json-list", json::stringify(json_list))
        .expect("Could not update settings with exercise list");
}