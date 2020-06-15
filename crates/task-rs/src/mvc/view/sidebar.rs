use super::super::super::data::tag;
use super::super::{Message, Model};
use iced::*;
use pipe_trait::*;

pub fn create(model: &Model) -> Element<'_, Message> {
    let mut sidebar = Column::new()
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
        ));

    sidebar = "All".pipe(Text::new).pipe(|text| sidebar.push(text));

    for entry in &model.data.tags {
        sidebar = entry
            .pipe(tag::entry::display)
            .pipe(Text::new)
            .pipe(|text| sidebar.push(text))
    }

    sidebar.into()
}
