use maplit::btreemap;
use task_rs::{
    data::{Data, Status, TagData, TagId, Task},
    iced::{Application, Settings},
    mvc::Model,
};

fn create_task(
    completed: bool,
    summary: impl ToString,
    details: impl ToString,
    sub: impl IntoIterator<Item = Task>,
    tags: impl IntoIterator<Item = TagId>,
) -> Task {
    Task {
        status: if completed {
            Status::Completed
        } else {
            Status::Active
        },
        summary: summary.to_string(),
        details: details.to_string(),
        sub: sub.into_iter().collect(),
        tags: tags.into_iter().collect(),
    }
}

fn main() {
    let tags = btreemap! {
        "abc".into() => TagData::default(),
        "def".into() => TagData::default(),
        "ghi".into() => TagData::default()
            .with_name("GHI Tag"),
        "jkl".into() => TagData::default()
            .with_description("This is a tag with id 'jkl'"),
        "mno".into() => TagData::default()
            .with_name("MNO Tag")
            .with_description("This is a tag with id 'mno'"),
    }
    .into();

    let tasks = vec![create_task(false, "first task", "", vec![], vec![])];

    let data = Data {
        tags,
        tasks,
        ..Data::default()
    };

    let settings = Settings {
        flags: Model {
            data,
            ..Model::default()
        },
        ..Settings::default()
    };

    Model::run(settings);
}
