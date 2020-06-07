use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sync {
    NoSync,
    GitPushPull {
        remote: String,
        branch: String,
        force: bool,
    },
}

impl std::str::FromStr for Sync {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text == "no-sync" {
            return Ok(Sync::NoSync);
        }

        let (flags, segments): (Vec<_>, Vec<_>) = text
            .split_whitespace()
            .partition(|text| text.starts_with('-'));

        let get_force = || flags.contains(&"-f") || flags.contains(&"--force");

        Ok(match segments[..] {
            ["git-push-pull"] => Sync::GitPushPull {
                remote: "origin".to_owned(),
                branch: "master".to_owned(),
                force: get_force(),
            },

            ["git-push-pull", remote, branch] => Sync::GitPushPull {
                remote: remote.to_owned(),
                branch: branch.to_owned(),
                force: get_force(),
            },

            _ => return Err(text.to_owned()),
        })
    }
}

#[cfg(test)]
mod test_sync_from_str;
