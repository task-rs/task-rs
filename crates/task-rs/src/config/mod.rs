pub mod source;
pub mod sync;

pub use source::Source;
pub use sync::Sync;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[structopt(long)]
    pub local_repo_location: PathBuf,

    #[structopt(long, default_value = "git-push-pull --force")]
    pub sync: Sync,
}
