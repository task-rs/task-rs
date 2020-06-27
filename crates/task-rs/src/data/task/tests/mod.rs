use super::super::super::utils::collect;
use super::super::Status;
use super::Task;
use std::collections::BTreeSet;

const JUST_ENOUGH_FIELDS: &str = include_str!("./assets/just-enough-fields.yaml");
const WITH_SOME_SUBS_AND_TAGS: &str = include_str!("./assets/with-some-subs-and-tags.yaml");

#[test]
fn deserialize() {
    let load = serde_yaml::from_str::<Task>;

    let actual = load(JUST_ENOUGH_FIELDS).unwrap();
    let expected = Task {
        status: Status::Active,
        summary: "Task title".to_owned(),
        details: "".to_owned(),
        sub: vec![],
        tags: BTreeSet::new(),
    };
    assert_eq!(actual, expected);

    let actual = load(WITH_SOME_SUBS_AND_TAGS).unwrap();
    let expected = Task {
        status: Status::Completed,
        summary: "Completed task".to_owned(),
        details: "details of a completed task".to_owned(),
        sub: vec![expected],
        tags: collect(&["foo", "bar", "baz"]),
    };
    assert_eq!(actual, expected);
}

#[test]
fn serialize() {
    let task = Task {
        status: Status::Active,
        summary: "Task title".to_owned(),
        details: "".to_owned(),
        sub: vec![],
        tags: BTreeSet::new(),
    };
    let yaml = serde_yaml::to_string(&task).unwrap();
    assert_eq!(
        yaml,
        JUST_ENOUGH_FIELDS.to_owned().trim_end(),
        "just enough fields",
    );

    let task = Task {
        status: Status::Completed,
        summary: "Completed task".to_owned(),
        details: "details of a completed task".to_owned(),
        sub: vec![task],
        tags: collect(&["foo", "bar", "baz"]),
    };
    let yaml = serde_yaml::to_string(&task).unwrap();
    assert_eq!(
        yaml,
        WITH_SOME_SUBS_AND_TAGS.to_owned().trim_end(),
        "with some subs and tags",
    );
}
