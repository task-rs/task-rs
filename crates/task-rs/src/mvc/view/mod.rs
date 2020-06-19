use super::super::components::Main;
use super::{Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    Main {
        model,
        set_task_status_filter: Message::SetTaskStatusFilter,
        set_dark_mode: Message::SetDarkMode,
    }
    .into()
}
