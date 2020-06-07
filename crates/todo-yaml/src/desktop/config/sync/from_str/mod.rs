pub mod error;
pub mod reason;

pub use error::*;
pub use reason::*;

use super::Sync;

impl std::str::FromStr for Sync {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text == "no-sync" {
            return Ok(Sync::NoSync);
        }

        let (flags, segments): (Vec<_>, Vec<_>) = text
            .split_whitespace()
            .partition(|text| text.starts_with('-'));

        let get_force = || flags.contains(&"-f") || flags.contains(&"--force");

        match segments[..] {
            ["git-push-pull"] => Ok(Sync::GitPushPull {
                remote: "origin".to_owned(),
                branch: "master".to_owned(),
                force: get_force(),
            }),

            ["git-push-pull", remote, branch] => Ok(Sync::GitPushPull {
                remote: remote.to_owned(),
                branch: branch.to_owned(),
                force: get_force(),
            }),

            [] => Reason::Empty.err(text),

            _ => match segments[0] {
                "git-push-pull" => match segments.len() {
                    2 => Reason::GitPushPullMissingArgument,
                    1 | 3 => unreachable!(),
                    _ => Reason::GitPushPullRedundantArgument(
                        segments[1..]
                            .iter()
                            .map(|x: &&str| x.to_owned().to_owned())
                            .collect::<Vec<_>>(),
                    ),
                },

                tag => Reason::UndefinedTag(tag.to_owned()),
            }
            .err(text),
        }
    }
}

#[cfg(test)]
mod tests;
