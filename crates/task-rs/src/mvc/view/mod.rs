use super::super::components::Main;
use super::{model::view::tasks::FilterMethod, Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    Main {
        model,
        set_task_status_filter: Message::SetTaskStatusFilter,
        set_dark_mode: Message::SetDarkMode,
        set_tag_filter_method_to_all: Message::SetTaskFilterMethod(FilterMethod::All),
    }
    .into()
}
