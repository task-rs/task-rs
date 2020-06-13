pub mod xdg;

pub use xdg::{config_file, ui_state_file};

use core::fmt::Display;
use serde::de::DeserializeOwned;
use std::{fs::File, path::PathBuf};

pub fn display(value: impl Display) -> String {
    format!("{}", value)
}

pub fn deserialize_file<Output: DeserializeOwned>(filename: &PathBuf) -> Result<Output, String> {
    let file = File::open(filename).map_err(display)?;
    let model = serde_yaml::from_reader(file).map_err(display)?;
    Ok(model)
}
