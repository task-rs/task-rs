pub mod callable;
pub mod data;
pub mod xdg;

pub use callable::Callable;
pub use data::from_cfg_opt as load_data_from_cfg_opt;
pub use xdg::{config_file, ui_state_file};

use pipe_trait::*;
use serde::de::DeserializeOwned;
use std::{fs::File, path::PathBuf};

pub fn deserialize_file<Output: DeserializeOwned>(filename: &PathBuf) -> Result<Output, String> {
    File::open(filename)
        .map_err(|error| format!("Failed to load {}: {}", filename.to_string_lossy(), error))?
        .pipe(serde_yaml::from_reader::<File, Output>)
        .map_err(|error| format!("Failed to parse {}: {}", filename.to_string_lossy(), error))?
        .pipe(Ok)
}
