use super::super::Sync;
use std::str::FromStr;

#[test]
fn no_sync() {
    assert_eq!(Sync::from_str("no-sync").unwrap(), Sync::NoSync);
}

#[test]
fn git_push_pull_default() {
    assert_eq!(
        Sync::from_str("git-push-pull").unwrap(),
        Sync::GitPushPull {
            remote: "origin".to_owned(),
            branch: "master".to_owned(),
            force: false,
        }
    );
}

#[test]
fn git_push_pull_remote_branch() {
    assert_eq!(
        Sync::from_str("git-push-pull foo bar").unwrap(),
        Sync::GitPushPull {
            remote: "foo".to_owned(),
            branch: "bar".to_owned(),
            force: false,
        }
    );
}

#[test]
fn git_push_pull_force() {
    assert_eq!(
        Sync::from_str("git-push-pull --force").unwrap(),
        Sync::GitPushPull {
            remote: "origin".to_owned(),
            branch: "master".to_owned(),
            force: true,
        }
    )
}

#[test]
fn git_push_pull_force_short_alias() {
    assert_eq!(
        Sync::from_str("git-push-pull -f").unwrap(),
        Sync::GitPushPull {
            remote: "origin".to_owned(),
            branch: "master".to_owned(),
            force: true,
        }
    )
}

#[test]
fn git_push_pull_force_remote_branch() {
    assert_eq!(
        Sync::from_str("git-push-pull --force foo bar").unwrap(),
        Sync::GitPushPull {
            remote: "foo".to_owned(),
            branch: "bar".to_owned(),
            force: true,
        }
    )
}

#[test]
fn git_push_pull_remote_branch_force() {
    assert_eq!(
        Sync::from_str("git-push-pull foo bar --force").unwrap(),
        Sync::GitPushPull {
            remote: "foo".to_owned(),
            branch: "bar".to_owned(),
            force: true,
        }
    )
}
