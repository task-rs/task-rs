use super::super::components::Main;
use super::{Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let Model {
        ref mut data,
        ref mut ui_state,
        ref mut controls,
    } = model;

    Main {
        data,
        ui_state,
        controls,
        set_task_status_filter: Message::SetTaskStatusFilter,
        set_dark_mode: Message::SetDarkMode,
    }
    .into()
}
