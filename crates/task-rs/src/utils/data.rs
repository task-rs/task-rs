use super::super::{config::Config, data::Data, mvc::model::UiState, utils::deserialize_file};

pub fn from_ui_state(ui_state: &UiState) -> Data {
    if let Some((
        Config {
            local_repo_location,
            ..
        },
        _,
    )) = &ui_state.config
    {
        match deserialize_file(&local_repo_location) {
            Ok(data) => data,
            Err(error) => {
                eprintln!(
                    "Failed to load {}: {}",
                    local_repo_location.to_string_lossy(),
                    error
                );
                Data::default()
            }
        }
    } else {
        Data::default()
    }
}
