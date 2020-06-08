use super::super::super::config::Config;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    LoadConfig {
        #[structopt(name = "command")]
        config_file: PathBuf,
    },
    CustomizeConfig(Config),
}
