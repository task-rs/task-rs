use super::Config;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "todo-yaml")]
pub enum Args {
    LoadConfig {
        #[structopt(long, short = "c")]
        config_file: Option<PathBuf>,
    },
    CustomizeConfig(Config),
}
