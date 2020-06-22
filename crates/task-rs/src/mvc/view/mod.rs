use super::super::components::Main;
use super::{model::view::tasks::FilterMethod, Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    Main {
        model,
        set_task_status_filter: Message::SetTaskStatusFilter,
        set_dark_mode: Message::SetDarkMode,
        set_tag_filter_method_to_all: Message::SetTaskFilterMethod(FilterMethod::All),
        set_task_filter_method_to_single_tag: Message::SetTaskFilterMethod(FilterMethod::SingleTag),
        set_task_filter_method_to_multiple_tags: Message::SetTaskFilterMethod(
            FilterMethod::MultipleTags,
        ),
        filter_tasks_by_single_tag: |id| Message::FilterTasksBySingleTag(id.clone()),
        add_tag_to_multiple_tags: |id| Message::AddTagToMultipleTags(id.clone()),
        remove_tag_from_multiple_tags: |id| Message::RemoveTagFromMultipleTags(id.clone()),
    }
    .into()
}
