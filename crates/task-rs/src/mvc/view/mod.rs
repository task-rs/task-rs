use super::super::components::{theme::Theme, Header, Sidebar, TaskStatusFilter};
use super::{Message, Model};
use iced::*;
use pipe_trait::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let theme = &model.ui_state.theme;
    let task_status_filter = model.ui_state.details.task_status_filter;

    Column::new()
        .push(Header {
            task_status_filter: TaskStatusFilter {
                controls: &mut model.controls,
                actual_value: task_status_filter,
                get_message: Message::SetTaskStatusFilter,
                theme,
            },
        })
        .push(Row::new().push(Sidebar(&model.data.tags)))
        .pipe(Container::new)
        .style(stylesheets::Container(theme.style()))
        .into()
}

mod stylesheets;
