use super::super::super::{
    config::{Config, Source},
    utils::config_file,
};
use super::super::args::Command::*;
use super::App;
use core::fmt::Display;
use serde_yaml::from_reader;
use std::{fs::File, path::PathBuf};

fn display(value: impl Display) -> String {
    format!("{}", value)
}

fn load_from_file(filename: &PathBuf) -> Result<Config, String> {
    let file = File::open(filename).map_err(display)?;
    let config = from_reader(file).map_err(display)?;
    Ok(config)
}

impl App {
    pub(crate) fn config(&self) -> Result<(Config, Source), String> {
        match &self.args.command {
            None => match config_file() {
                Some(config_file) => load_from_file(&config_file)
                    .map(|config| (config, Source::ConfigFile(config_file))),
                None => Err("Cannot determine config file location".to_owned()),
            },
            Some(LoadConfig { config_file }) => load_from_file(config_file)
                .map(|config| (config, Source::ConfigFile(config_file.clone()))),
            Some(CustomizeConfig(config)) => Ok((config.clone(), Source::CommandLineArguments)),
        }
    }
}
