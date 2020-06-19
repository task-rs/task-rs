use maplit::btreemap;
use task_rs::{
    data::{Data, TagData},
    iced::{Application, Settings},
    mvc::{Model, UiState},
};

fn main() {
    let data = Data {
        tags: btreemap! {
            "abc".into() => TagData::default(),
            "def".into() => TagData::default(),
            "ghi".into() => TagData::default().with_name("GHI Tag"),
            "jkl".into() => TagData::default().with_description("This is a tag with id 'jkl'"),
            "mno".into() => TagData::default()
                .with_name("MNO Tag")
                .with_description("This is a tag with id 'mno'"),
        },
        ..Data::default()
    };

    let flags = Model {
        data,
        ui_state: UiState::default(),
        ..Model::default()
    };

    let settings = Settings {
        flags,
        ..Settings::default()
    };

    Model::run(settings);
}
