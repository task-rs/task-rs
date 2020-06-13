use super::super::super::{
    data::Data,
    mvc::model::{Model, Progress, Title, View},
    utils::ui_state_file,
};
use super::App;
use core::fmt::Display;
use serde_yaml::from_reader;
use std::{fs::File, path::PathBuf};

fn display(value: impl Display) -> String {
    format!("{}", value)
}

fn load_from_file(filename: &PathBuf) -> Result<Model, String> {
    let file = File::open(filename).map_err(display)?;
    let model = from_reader(file).map_err(display)?;
    Ok(model)
}

impl App {
    fn load_flags(&self) -> Result<Model, String> {
        match ui_state_file() {
            Some(ui_state_file) => load_from_file(&ui_state_file),
            None => Err("Cannot determine ui state file location".to_owned()),
        }
    }

    fn fallback_flags(&self) -> Model {
        let config = match self.config() {
            Ok(config) => Some(config),
            Err(error) => {
                eprintln!("WARN {}", error);
                None
            }
        };
        Model {
            config,
            data: Data::default(),
            progress: Progress::default(),
            title: Title::default(),
            view: View::default(),
        }
    }

    pub(crate) fn flags(&self) -> Model {
        self.load_flags().unwrap_or_else(|error| {
            eprintln!("WARN {}", error);
            self.fallback_flags()
        })
    }
}
