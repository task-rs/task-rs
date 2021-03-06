pub mod bidirectional_map;
pub mod callable;
pub mod data;
pub mod indexed_map;
pub mod merge;
pub mod xdg;

pub use bidirectional_map::BidirectionalMap;
pub use callable::Callable;
pub use data::from_cfg_opt as load_data_from_cfg_opt;
pub use indexed_map::IndexedMap;
pub use merge::merge_sets_with_minimal_cloning;
pub use xdg::{config_file, ui_state_file};

use iced::*;
use pipe_trait::*;
use serde::de::DeserializeOwned;
use std::{fs::File, iter::FromIterator, path::PathBuf};

pub fn deserialize_file<Output: DeserializeOwned>(filename: &PathBuf) -> Result<Output, String> {
    File::open(filename)
        .map_err(|error| format!("Failed to load {}: {}", filename.to_string_lossy(), error))?
        .pipe(serde_yaml::from_reader::<File, Output>)
        .map_err(|error| format!("Failed to parse {}: {}", filename.to_string_lossy(), error))?
        .pipe(Ok)
}

pub fn on_press_if<'a, Message>(
    condition: bool,
    message: Message,
) -> impl FnOnce(Button<'a, Message>) -> Button<'a, Message>
where
    Message: 'a,
{
    move |button| {
        if condition {
            button.on_press(message)
        } else {
            button
        }
    }
}

pub fn collect<Result: FromIterator<Item>, Item>(
    iter: impl IntoIterator<Item = impl Into<Item>>,
) -> Result {
    iter.into_iter().map(Into::into).collect()
}
