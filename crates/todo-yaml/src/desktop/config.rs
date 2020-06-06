use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize, StructOpt)]
#[serde(rename_all = "kebab-case")]
#[structopt(name = "todo-yaml")]
pub struct Config {
    #[structopt(long)]
    pub local_repo_location: String,
}
