pub mod sync;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[structopt(long)]
    pub local_repo_location: PathBuf,

    #[structopt(long, default_value = "git-push-pull --force")]
    pub sync: sync::Sync,
}
