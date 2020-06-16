use super::super::{Message, Model};
use iced::*;

pub fn create(model: &Model) -> Element<'_, Message> {
    Row::new()
        .push(Text::new("TaskRs").size(40))
        .push(Checkbox::new(
            model.ui_state.details.task_status_filter.active,
            "show active",
            Message::TaskStatusFilterActive,
        ))
        .push(Checkbox::new(
            model.ui_state.details.task_status_filter.completed,
            "show completed",
            Message::TaskStatusFilterCompleted,
        ))
        .into()
}
