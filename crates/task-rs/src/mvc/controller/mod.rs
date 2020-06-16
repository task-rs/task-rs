use super::{Message, Model};
use iced::Command;

pub fn new(model: Model) -> (Model, Command<Message>) {
    (model, Command::none())
}

pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    match message {
        Message::SetTaskStatusFilter(x) => model.ui_state.details.task_status_filter = x,
    }

    Command::none()
}
