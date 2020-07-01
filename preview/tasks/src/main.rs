use pipe_trait::*;
use task_rs::{
    components::Main,
    iced::{Application, Settings},
    serde_yaml,
};

fn main() {
    let data = include_str!("./assets/data.yaml")
        .pipe(serde_yaml::from_str)
        .unwrap();

    let settings = Settings {
        flags: Main {
            data,
            ..Main::default()
        },
        ..Settings::default()
    };

    Main::run(settings);
}
