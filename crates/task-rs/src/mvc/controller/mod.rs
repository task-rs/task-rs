use super::{model::Theme, Message, Model};
use iced::Command;

pub fn new(model: Model) -> (Model, Command<Message>) {
    (model, Command::none())
}

pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    match message {
        Message::SetTaskStatusFilter(x) => model.ui_state.details.task_status_filter = x,
        Message::SetDarkMode(x) => {
            model.ui_state.theme = if x { Theme::Dark } else { Theme::Light }
        }
    }

    Command::none()
}
