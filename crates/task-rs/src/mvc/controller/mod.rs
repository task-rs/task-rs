use super::super::data::Data;
use super::{Message, Model, UiState};
use iced::Command;

pub fn new(ui_state: UiState) -> (Model, Command<Message>) {
    let data = Data::default();
    let model = Model { data, ui_state };
    (model, Command::none())
}

pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    Command::none()
}
