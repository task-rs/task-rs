use super::super::{Error, Reason, Sync};
use std::str::FromStr;

#[test]
fn empty_structural() {
    assert_eq!(
        Sync::from_str("").unwrap_err(),
        Error {
            text: "".to_owned(),
            reason: Reason::Empty,
        },
    )
}

#[test]
fn empty_to_string() {
    assert_eq!(
        Error {
            text: "".to_owned(),
            reason: Reason::Empty
        }
        .to_string(),
        format!("{:?} is invalid. reason: empty", ""),
    )
}

#[test]
fn undefined_tag_structural() {
    assert_eq!(
        Sync::from_str("foo bar baz").unwrap_err(),
        Error {
            text: "foo bar baz".to_owned(),
            reason: Reason::UndefinedTag("foo".to_owned()),
        },
    )
}

#[test]
fn undefined_tag_to_string() {
    assert_eq!(
        Error {
            text: "foo bar baz".to_owned(),
            reason: Reason::UndefinedTag("foo".to_owned()),
        }
        .to_string(),
        format!(
            "{:?} is invalid. reason: tag {:?} is not defined",
            "foo bar baz", "foo",
        ),
    )
}

#[test]
fn git_push_pull_missing_argument_structural() {
    assert_eq!(
        Sync::from_str("git-push-pull foo").unwrap_err(),
        Error {
            text: "git-push-pull foo".to_owned(),
            reason: Reason::GitPushPullMissingArgument,
        },
    )
}

#[test]
fn git_push_pull_missing_argument_to_string() {
    assert_eq!(
        Error {
            text: "git-push-pull foo".to_owned(),
            reason: Reason::GitPushPullMissingArgument,
        }
        .to_string(),
        format!(
            "{:?} is invalid. reason: git-push-pull: missing argument",
            "git-push-pull foo",
        ),
    )
}

#[test]
fn git_push_pull_redundant_argument_structural() {
    assert_eq!(
        Sync::from_str("git-push-pull foo bar baz").unwrap_err(),
        Error {
            text: "git-push-pull foo bar baz".to_owned(),
            reason: Reason::GitPushPullRedundantArgument(vec![
                "foo".to_owned(),
                "bar".to_owned(),
                "baz".to_owned(),
            ]),
        },
    )
}

#[test]
fn git_push_pull_redundant_argument_to_string() {
    assert_eq!(
        Error {
            text: "git-push-pull foo bar baz".to_owned(),
            reason: Reason::GitPushPullRedundantArgument(vec![
                "foo".to_owned(),
                "bar".to_owned(),
                "baz".to_owned(),
            ]),
        }
        .to_string(),
        format!(
            "{:?} is invalid. reason: git-push-pull: redundant arguments: {:?}",
            "git-push-pull foo bar baz",
            vec!["foo", "bar", "baz"],
        ),
    )
}
