use super::super::{config::Config, data::Data, utils::deserialize_file};
use super::{Message, Model, UiState};
use iced::Command;

fn load_data(ui_state: &UiState) -> Data {
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

pub fn new(ui_state: UiState) -> (Model, Command<Message>) {
    let data = load_data(&ui_state);
    let model = Model { data, ui_state };
    (model, Command::none())
}

pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    Command::none()
}
