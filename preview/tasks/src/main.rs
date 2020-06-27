use pipe_trait::*;
use task_rs::{
    iced::{Application, Settings},
    mvc::Model,
    serde_yaml,
};

fn main() {
    let data = include_str!("./assets/data.yaml")
        .pipe(serde_yaml::from_str)
        .unwrap();

    let settings = Settings {
        flags: Model {
            data,
            ..Model::default()
        },
        ..Settings::default()
    };

    Model::run(settings);
}
