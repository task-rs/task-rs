use super::super::components::{theme::Theme, Header, Sidebar, TaskStatusFilter, ThemeSwitcher};
use super::{model, Message, Model};
use iced::*;
use pipe_trait::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let theme = &model.ui_state.theme;
    let task_status_filter = model.ui_state.details.task_status_filter;

    Column::new()
        .push(Header {
            task_status_filter: TaskStatusFilter {
                controls: &mut model.controls.task_status_filter,
                actual_value: task_status_filter,
                get_message: Message::SetTaskStatusFilter,
                theme,
            },
            theme_switcher: ThemeSwitcher {
                dark_mode: model.ui_state.theme == model::Theme::Dark,
                get_message: Message::SetDarkMode,
                controls: &mut model.controls.theme_switcher,
                theme,
            },
        })
        .push(Row::new().push(Sidebar(&model.data.tags)))
        .pipe(Container::new)
        .style(stylesheets::Container(theme.style()))
        .into()
}

mod stylesheets;
