use super::super::super::{
    config::{Config, Source},
    utils::{config_file, deserialize_file},
};
use super::super::args::Command::*;
use super::App;

impl App {
    pub(crate) fn config(&self) -> Result<(Config, Source), String> {
        match &self.args.command {
            None => match config_file() {
                Some(config_file) => deserialize_file(&config_file)
                    .map(|config| (config, Source::ConfigFile(config_file))),
                None => Err("Cannot determine config file location".to_owned()),
            },
            Some(LoadConfig { config_file }) => deserialize_file(config_file)
                .map(|config| (config, Source::ConfigFile(config_file.clone()))),
            Some(CustomizeConfig(config)) => Ok((config.clone(), Source::CommandLineArguments)),
        }
    }
}
